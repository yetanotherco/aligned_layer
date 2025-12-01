use actix_web::{web, App, HttpResponse, HttpServer};

use crate::{config::Config, db::Db};

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
        HttpServer::new(|| {
            App::new().route(
                "/",
                web::get().to(async || HttpResponse::Ok().body("Hey there!")),
            )
        })
        .bind(("127.0.0.1", self.config.port))?
        .run()
        .await
    }
}
