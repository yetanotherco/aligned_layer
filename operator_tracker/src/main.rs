use std::sync::Arc;

use axum::body::Body;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use ethers::providers::{Http, Provider, ProviderExt};
use ethers::types::Address;
use log::debug;
use log::info;
use operator_tracker::create_or_update_operator_version;
use operator_tracker::serialize_or_err;
use operator_tracker::{OperatorVersionPayload, RegistryCoordinator, RegistryCoordinatorContract};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Executor;

// The RegistryCoordinator contract
// This is the contract that the service will interact with to get the status of the operator
// The response is a uint8 that represents the status of the operator
// 0 - Never registered
// 1 - Registered
// 2 - Deregistered

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    registry_coordinator: RegistryCoordinatorContract,
}

/// The main entry point for the application
/// This function initializes the database, the Ethereum provider, and the RegistryCoordinator contract
#[derive(argh::FromArgs)]
struct Args {
    /// path to the .env file
    #[argh(option)]
    env_file: Option<String>,
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    if let Some(env_file) = args.env_file {
        dotenvy::from_filename(env_file).expect("Failed to load env file");
    } else {
        dotenvy::dotenv().expect("Failed to load env file");
    }

    env_logger::init();

    let database_url = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let rpc_url = &std::env::var("RPC_URL").expect("RPC_URL must be set");

    let registry_coordinator_addr = &std::env::var("REGISTRY_COORDINATOR_ADDRESS")
        .expect("REGISTRY_COORDINATOR_ADDRESS must be set");

    let listen_addr = &std::env::var("LISTEN_ADDR").expect("LISTEN_ADDR must be set");

    let registry_coordinator_addr: Address = registry_coordinator_addr
        .parse()
        .expect("Failed to parse registry coordinator addr");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create connection pool");

    pool.execute(include_str!("../sql/schema.sql"))
        .await
        .expect("Failed to initialize DB");

    let eth_rpc = Arc::new(Provider::<Http>::connect(rpc_url).await);

    let registry_coordinator = RegistryCoordinator::new(registry_coordinator_addr, eth_rpc.clone());

    let state = AppState {
        pool,
        registry_coordinator,
    };

    let listener = tokio::net::TcpListener::bind(listen_addr)
        .await
        .expect("Failed to bind listener");

    let router = Router::new()
        .route("/versions", post(post_operator_version))
        .route("/versions", get(list_operator_versions))
        .route("/versions/:address", get(get_operator_version))
        .with_state(state);

    axum::serve(listener, router)
        .await
        .expect("Failed to serve app");
}

async fn post_operator_version(
    state: State<AppState>,
    Json(payload): Json<OperatorVersionPayload>,
) -> axum::http::Response<Body> {
    debug!(
        "Received operator version request. Version {}",
        payload.version
    );

    match create_or_update_operator_version(&state.pool, &state.registry_coordinator, payload).await
    {
        Ok(Some(body)) => {
            info!("Operator version created successfully");

            let (mut status, serialized_body) = serialize_or_err(&body);
            if status == StatusCode::OK {
                status = StatusCode::CREATED;
            }

            axum::http::Response::builder()
                .header("Location", format!("/versions/{}", body.address))
                .status(status)
                .body(serialized_body)
                .unwrap_or_default() // Should never fail but dont panic
        }
        Ok(None) => {
            info!("Operator version updated or already matched");

            axum::http::Response::builder()
                .status(axum::http::StatusCode::NO_CONTENT)
                .body(Body::empty())
                .unwrap_or_default() // Should never fail but dont panic
        }
        Err(err) => {
            info!("Operator version already exists");
            err.into_response()
        }
    }
}

async fn list_operator_versions(state: State<AppState>) -> axum::http::Response<Body> {
    let rows = match operator_tracker::list_operator_versions(&state.pool).await {
        Ok(rows) => rows,
        Err(err) => return err.into_response(),
    };

    let (status, body) = if rows.is_empty() {
        (axum::http::StatusCode::NO_CONTENT, Body::empty())
    } else {
        serialize_or_err(rows)
    };

    axum::http::Response::builder()
        .status(status)
        .body(body)
        .unwrap_or_default() // Should never fail but dont panic
}

async fn get_operator_version(
    state: State<AppState>,
    Path(address): Path<String>,
) -> axum::http::Response<Body> {
    let row = match operator_tracker::get_operator_version(&state.pool, &address).await {
        Ok(row) => row,
        Err(err) => return err.into_response(),
    };

    let (status, body) = if let Some(row) = row {
        serialize_or_err(row)
    } else {
        (axum::http::StatusCode::NOT_FOUND, Body::empty())
    };

    axum::http::Response::builder()
        .status(status)
        .body(body)
        .unwrap_or_default() // Should never fail but dont panic
}
