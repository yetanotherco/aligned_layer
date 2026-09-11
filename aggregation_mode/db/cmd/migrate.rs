use sqlx::postgres::PgPoolOptions;

fn read_db_conn_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!(
            "You must provide a config file. Usage: {} <config-file-path>",
            args[0]
        );
    }

    args[1].clone()
}

#[tokio::main]
async fn main() {
    let db_connection_url = read_db_conn_from_args();
    let pool = PgPoolOptions::new()
        .connect(&db_connection_url)
        .await
        .expect("To connect to db");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations to run correctly")
}
