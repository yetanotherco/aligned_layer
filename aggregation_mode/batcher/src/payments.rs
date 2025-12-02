use crate::{config::Config, db::Db};

struct PaymentsPooler {
    db: Db,
}

impl PaymentsPooler {
    pub fn new(db: Db, config: Config) -> Self {
        Self { db }
    }

    pub async fn start() {}
}
