use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use aligned_sdk::aggregation_layer::AggregationModeProvingSystem;
use sqlx::types::BigDecimal;

use super::{
    helpers::format_merkle_paths,
    types::{AppResponse, GetReceiptsQueryParams},
};

use crate::{
    config::Config,
    db::Db,
    server::types::{
        SubmitProofRequest, SubmitProofRequestMessageRisc0, SubmitProofRequestMessageSP1,
    },
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
        let Some(address) = req.match_info().get("address") else {
            return HttpResponse::BadRequest()
                .json(AppResponse::new_unsucessfull("Missing address", 400));
        };

        // TODO: validate valid ethereum address

        let Some(state) = req.app_data::<Data<BatcherServer>>() else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };

        let state = state.get_ref();
        match state.db.count_tasks_by_address(address).await {
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
        body: web::Json<SubmitProofRequest<SubmitProofRequestMessageSP1>>,
    ) -> impl Responder {
        let data = body.into_inner();

        // TODO: validate signature
        let recovered_address = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_lowercase();

        let Some(state) = req.app_data::<Data<BatcherServer>>() else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };
        let state = state.get_ref();

        let Ok(count) = state.db.count_tasks_by_address(&recovered_address).await else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };

        if data.nonce != (count as u64) {
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

        // TODO: decode proof and validate it

        match state
            .db
            .insert_task(
                &recovered_address,
                AggregationModeProvingSystem::SP1.as_u16() as i32,
                &data.message.proof,
                &data.message.program_vk_commitment,
                None,
                count,
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
        _body: web::Json<SubmitProofRequest<SubmitProofRequestMessageRisc0>>,
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

        let Some(address) = params.address.clone() else {
            return HttpResponse::BadRequest()
                .json(AppResponse::new_unsucessfull("Missing address", 400));
        };

        // TODO: maybe also accept proof commitment in query param
        let receipts = if let Some(nonce) = params.nonce {
            match state
                .db
                .get_tasks_by_address_and_nonce(&address, nonce)
                .await
            {
                Ok(receipts) => receipts,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(
                        AppResponse::new_unsucessfull(
                            format!(
                            "Internal server error: Failed to get tasks by address and nonce: {e}"
                        )
                            .as_str(),
                            500,
                        ),
                    );
                }
            }
        } else {
            match state.db.get_tasks_by_address(&address, 100).await {
                Ok(receipts) => receipts,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(
                        AppResponse::new_unsucessfull(
                            format!(
                            "Internal server error: Failed to get tasks by address and nonce: {e}"
                        )
                            .as_str(),
                            500,
                        ),
                    );
                }
            }
        };

        match format_merkle_paths(receipts) {
            Ok(receipts) => {
                HttpResponse::Ok().json(AppResponse::new_sucessfull(serde_json::json!({
                    "receipts": receipts
                })))
            }
            Err(e) => HttpResponse::InternalServerError().json(AppResponse::new_unsucessfull(
                format!("Internal server error: {e}").as_str(),
                500,
            )),
        }
    }
}
