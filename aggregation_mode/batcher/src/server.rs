use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use crate::{config::Config, db::Db};

/// Starts the http server to receive proofs
///
/// It is also thread safe to share and it is used as the shared state between endpoints
#[derive(Clone, Debug)]
pub struct BatcherServer {
    db: Db,
    config: Config,
}

/* TODO:
 * Create endpoints for:
 *   1. Receive proof
 *       - Check payment, Verify it, Store it in db, Return a Receipt
 *   2. Return nonce
 *   3. Return merkle path
 */
impl BatcherServer {
    pub fn new(db: Db, config: Config) -> Self {
        Self { db, config }
    }

    pub async fn start(&self) -> Result<(), std::io::Error> {
        // Note: BatcherServer is thread safe so we can just clone it (no need to add mutexes)
        let port = self.config.port;
        let state = self.clone();

        HttpServer::new(move || Self::build_app(state.clone()))
            .bind(("127.0.0.1", port))?
            .run()
            .await
    }

    fn build_app(state: BatcherServer) -> App {
        App::new()
            .app_data(Data::new(state))
            .route("/nonce/:address", web::get().to(Self::get_nonce))
            .route(
                "/proof/merkle/:receipt",
                web::get().to(Self::get_proof_merkle_path),
            )
            .route("/proof", web::post().to(Self::post_proof))
    }

    async fn get_nonce(req: HttpRequest) -> impl Responder {
        HttpResponse::Ok()
    }

    async fn post_proof(req: HttpRequest) -> impl Responder {
        HttpResponse::Ok()
    }

    async fn get_proof_merkle_path(req: HttpRequest) -> impl Responder {
        HttpResponse::Ok()
    }
}
