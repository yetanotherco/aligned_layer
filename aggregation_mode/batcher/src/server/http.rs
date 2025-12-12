use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_multipart::form::MultipartForm;
use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use aligned_sdk::aggregation_layer::AggregationModeProvingSystem;
use sp1_sdk::{SP1ProofWithPublicValues, SP1VerifyingKey};
use sqlx::types::BigDecimal;

use super::{
    helpers::format_merkle_path,
    types::{AppResponse, GetReceiptsQueryParams},
};

use crate::{
    config::Config,
    db::Db,
    server::types::{GetReceiptsResponse, SubmitProofRequestRisc0, SubmitProofRequestSP1},
    verifiers::{verify_sp1_proof, VerificationError},
};

#[derive(Clone, Debug)]
pub struct BatcherServer {
    db: Db,
    config: Config,
}

impl BatcherServer {
    pub fn new(db: Db, config: Config) -> Self {
        Self { db, config }
    }

    pub async fn start(&self) {
        // Note: BatcherServer is thread safe so we can just clone it (no need to add mutexes)
        let port = self.config.port;
        let state = self.clone();

        tracing::info!("Starting server at port {}", self.config.port);
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(state.clone()))
                .route("/nonce/{address}", web::get().to(Self::get_nonce))
                .route("/receipts", web::get().to(Self::get_receipts))
                .route("/proof/sp1", web::post().to(Self::post_proof_sp1))
                .route("/proof/risc0", web::post().to(Self::post_proof_risc0))
        })
        .bind(("127.0.0.1", port))
        .expect("To bind socket correctly")
        .run()
        .await
        .expect("Server to never end");
    }

    // Returns the nonce (number of submitted tasks) for a given address
    async fn get_nonce(req: HttpRequest) -> impl Responder {
        let Some(address_raw) = req.match_info().get("address") else {
            return HttpResponse::BadRequest()
                .json(AppResponse::new_unsucessfull("Missing address", 400));
        };

        // Check that the address is a valid ethereum address
        if alloy::primitives::Address::from_str(address_raw.trim()).is_err() {
            return HttpResponse::BadRequest()
                .json(AppResponse::new_unsucessfull("Invalid address", 400));
        }

        let address = address_raw.to_lowercase();

        let Some(state) = req.app_data::<Data<BatcherServer>>() else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };

        let state = state.get_ref();
        match state.db.count_tasks_by_address(&address).await {
            Ok(count) => HttpResponse::Ok().json(AppResponse::new_sucessfull(serde_json::json!(
                {
                    "nonce": count
                }
            ))),
            Err(_) => HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500)),
        }
    }

    // Posts an SP1 proof to the batcher, recovering the address from the signature
    async fn post_proof_sp1(
        req: HttpRequest,
        MultipartForm(data): MultipartForm<SubmitProofRequestSP1>,
    ) -> impl Responder {
        let recovered_address = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_lowercase();

        let Some(state) = req.app_data::<Data<BatcherServer>>() else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };
        let state = state.get_ref();

        // Checking if this address has submited more proofs than the ones allowed per day
        let Ok(daily_tasks_by_address) = state
            .db
            .get_daily_tasks_by_address(&recovered_address)
            .await
        else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };

        if daily_tasks_by_address >= state.config.max_daily_proofs_per_user {
            return HttpResponse::InternalServerError().json(AppResponse::new_unsucessfull(
                "Request denied: Query limit exceeded.",
                400,
            ));
        }

        let Ok(count) = state.db.count_tasks_by_address(&recovered_address).await else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };

        if data.nonce.0 != (count as u64) {
            return HttpResponse::BadRequest().json(AppResponse::new_unsucessfull(
                &format!("Invalid nonce, expected nonce = {count}"),
                400,
            ));
        }

        let now_epoch = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(AppResponse::new_unsucessfull("Internal server error", 500));
            }
        };

        let has_payment = match state
            .db
            .has_active_payment_event(
                &recovered_address,
                // safe unwrap the number comes from a valid u64 primitive
                BigDecimal::from_str(&now_epoch.to_string()).unwrap(),
            )
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(AppResponse::new_unsucessfull("Internal server error", 500));
            }
        };

        if !has_payment {
            return HttpResponse::BadRequest().json(AppResponse::new_unsucessfull(
                "You have to pay before submitting a proof",
                400,
            ));
        }

        let Ok(proof_content) = tokio::fs::read(data.proof.file.path()).await else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };

        let Ok(proof) = bincode::deserialize::<SP1ProofWithPublicValues>(&proof_content) else {
            return HttpResponse::BadRequest()
                .json(AppResponse::new_unsucessfull("Invalid SP1 proof", 400));
        };

        let Ok(vk_content) = tokio::fs::read(data.program_vk.file.path()).await else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };

        let Ok(vk) = bincode::deserialize::<SP1VerifyingKey>(&vk_content) else {
            return HttpResponse::BadRequest()
                .json(AppResponse::new_unsucessfull("Invalid vk", 400));
        };

        if let Err(e) = verify_sp1_proof(&proof, &vk) {
            let message = match e {
                VerificationError::InvalidProof => "Proof verification failed",
                VerificationError::UnsupportedProof => "Unsupported proof",
            };

            return HttpResponse::BadRequest().json(AppResponse::new_unsucessfull(message, 400));
        };

        match state
            .db
            .insert_task(
                &recovered_address,
                AggregationModeProvingSystem::SP1.as_u16() as i32,
                &proof_content,
                &vk_content,
                None,
                data.nonce.0 as i64,
            )
            .await
        {
            Ok(task_id) => HttpResponse::Ok().json(AppResponse::new_sucessfull(
                serde_json::json!({ "task_id": task_id.to_string() }),
            )),
            Err(_) => HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500)),
        }
    }

    /// TODO: complete for risc0 (see `post_proof_sp1`)
    // Posts a Risc0 proof to the batcher, recovering the address from the signature
    async fn post_proof_risc0(
        _req: HttpRequest,
        MultipartForm(_): MultipartForm<SubmitProofRequestRisc0>,
    ) -> impl Responder {
        HttpResponse::Ok().json(AppResponse::new_sucessfull(serde_json::json!({})))
    }

    // Returns the last 100 receipt merkle proofs for the address received in the URL.
    // In case of also receiving a nonce on the query param, it returns only the merkle proof for that nonce.
    async fn get_receipts(
        req: HttpRequest,
        params: web::Query<GetReceiptsQueryParams>,
    ) -> impl Responder {
        let Some(state) = req.app_data::<Data<BatcherServer>>() else {
            return HttpResponse::InternalServerError().json(AppResponse::new_unsucessfull(
                "Internal server error: Failed to get app data",
                500,
            ));
        };

        let state = state.get_ref();

        if alloy::primitives::Address::from_str(params.address.clone().trim()).is_err() {
            return HttpResponse::BadRequest()
                .json(AppResponse::new_unsucessfull("Invalid address", 400));
        }

        let limit = match params.limit {
            Some(received_limit) => received_limit.min(100),
            None => 100,
        };

        let address = params.address.to_lowercase();

        let query = if let Some(nonce) = params.nonce {
            state
                .db
                .get_tasks_by_address_and_nonce(&address, nonce)
                .await
        } else {
            state
                .db
                .get_tasks_by_address_with_limit(&address, limit)
                .await
        };

        let Ok(receipts) = query else {
            return HttpResponse::InternalServerError().json(AppResponse::new_unsucessfull(
                "Internal server error: Failed to get tasks by address and nonce",
                500,
            ));
        };

        let responses: Result<Vec<GetReceiptsResponse>, String> = receipts
            .into_iter()
            .map(|receipt| {
                let Some(merkle_path) = receipt.merkle_path else {
                    return Ok(GetReceiptsResponse {
                        status: receipt.status,
                        merkle_path: Vec::new(),
                        nonce: receipt.nonce,
                        address: receipt.address,
                    });
                };

                let Ok(formatted) = format_merkle_path(&merkle_path) else {
                    return Err("Error formatting merkle path".into());
                };

                Ok(GetReceiptsResponse {
                    status: receipt.status,
                    merkle_path: formatted,
                    nonce: receipt.nonce,
                    address: receipt.address,
                })
            })
            .collect();

        match responses {
            Ok(resp) => HttpResponse::Ok().json(AppResponse::new_sucessfull(serde_json::json!({
                "receipts": resp
            }))),
            Err(_) => HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500)),
        }
    }
}
