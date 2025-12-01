use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use super::types::AppResponse;

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
                .route(
                    "/proof/merkle/:receipt",
                    web::get().to(Self::get_proof_merkle_path),
                )
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

    // TODO: get the proof merkle path for the receipt id (proof commitment)
    async fn get_proof_merkle_path(req: HttpRequest) -> impl Responder {
        HttpResponse::Ok()
    }
}
