use actix_web::{
    web::{self, Data, Query},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

use super::{
    helpers::format_merkle_path,
    types::{AppResponse, ProofMerkleQuery},
};

use crate::{config::Config, db::Db};

#[derive(Clone, Debug)]
pub struct BatcherServer {
    db: Db,
    config: Config,
}

impl BatcherServer {
    pub fn new(db: Db, config: Config) -> Self {
        Self { db, config }
    }

    pub async fn start(&self) -> Result<(), std::io::Error> {
        // Note: BatcherServer is thread safe so we can just clone it (no need to add mutexes)
        let port = self.config.port;
        let state = self.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(state.clone()))
                .route("/nonce/{address}", web::get().to(Self::get_nonce))
                .route("/proof/merkle", web::get().to(Self::get_proof_merkle_path))
                .route("/proof", web::post().to(Self::post_proof))
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
    }

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
        match state.db.count_proofs_by_address(address).await {
            Ok(count) => HttpResponse::Ok().json(AppResponse::new_sucessfull(serde_json::json!(
                {
                    "nonce": count
                }
            ))),
            Err(err) => {
                tracing::error!(error = ?err, "failed to count proofs");
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    // TODO: receive the proof and 1. decode it, 2. verify it, 3. add to the db
    async fn post_proof(req: HttpRequest) -> impl Responder {
        HttpResponse::Ok()
    }

    async fn get_proof_merkle_path(
        req: HttpRequest,
        params: web::Query<ProofMerkleQuery>,
    ) -> impl Responder {
        let Some(state) = req.app_data::<Data<BatcherServer>>() else {
            return HttpResponse::InternalServerError()
                .json(AppResponse::new_unsucessfull("Internal server error", 500));
        };

        let state = state.get_ref();

        // TODO: maybe also accept proof commitment in query param
        let Some(id) = params.id.clone() else {
            return HttpResponse::BadRequest().json(AppResponse::new_unsucessfull(
                "Provide proof `id` query param",
                400,
            ));
        };

        if id.is_empty() {
            return HttpResponse::BadRequest().json(AppResponse::new_unsucessfull(
                "Proof id cannot be empty",
                400,
            ));
        }

        let Ok(proof_id) = sqlx::types::Uuid::parse_str(&id) else {
            return HttpResponse::BadRequest()
                .json(AppResponse::new_unsucessfull("Proof id invalid uuid", 400));
        };

        let db_result = state.db.get_merkle_path_by_proof_id(proof_id).await;
        let merkle_path = match db_result {
            Ok(Some(merkle_path)) => merkle_path,
            Ok(None) => {
                return HttpResponse::NotFound().json(AppResponse::new_unsucessfull(
                    "Proof merkle path not found",
                    404,
                ))
            }
            Err(s) => {
                return HttpResponse::InternalServerError()
                    .json(AppResponse::new_unsucessfull("Internal server error", 500));
            }
        };

        match format_merkle_path(&merkle_path) {
            Ok(merkle_path) => {
                HttpResponse::Ok().json(AppResponse::new_sucessfull(serde_json::json!({
                    "merkle_path": merkle_path
                })))
            }
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(AppResponse::new_unsucessfull("Internal server error", 500))
            }
        }
    }
}
