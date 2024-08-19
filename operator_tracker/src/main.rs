use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use ethers::contract::abigen;
use ethers::core::types::Signature;
use ethers::providers::{Http, Provider, ProviderExt};
use ethers::types::Address;
use ethers::utils::keccak256;
use log::debug;
use log::info;
use serde::Serialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Executor;

// The RegistryCoordinator contract
// This is the contract that the service will interact with to get the status of the operator
// The response is a uint8 that represents the status of the operator
// 0 - Never registered
// 1 - Registered
// 2 - Deregistered
abigen!(
    RegistryCoordinator,
    r#"[function getOperatorStatus(address) external view returns (uint8)]"#,
);

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    registry_coordinator: RegistryCoordinator<Provider<Http>>,
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
        dotenv::from_filename(env_file).expect("Failed to load env file");
    } else {
        dotenv::dotenv().expect("Failed to load env file");
    }

    env_logger::init();

    let database_url = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let rpc_url = &std::env::var("RPC_URL").expect("RPC_URL must be set");

    let registry_coordinator_addr = &std::env::var("REGISTRY_COORDINATOR_ADDRESS")
        .expect("REGISTRY_COORDINATOR_ADDRESS must be set");

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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind listener");

    let router = Router::new()
        .route("/versions", post(post_operator_version))
        .route("/versions", get(list_operator_versions))
        .with_state(state);

    axum::serve(listener, router)
        .await
        .expect("Failed to serve app");
}

#[derive(serde::Deserialize)]
struct OperatorVersionPayload {
    pub version: String,
    pub signature: String,
}

#[derive(serde::Serialize, Debug)]
enum OperatorVersionError {
    InvalidSignature,
    OperatorNotRegistered,
}

impl IntoResponse for OperatorVersionError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        axum::http::Response::builder()
            .status(axum::http::StatusCode::BAD_REQUEST)
            .body(axum::body::Body::from(
                serde_json::to_string(&self).unwrap(),
            ))
            .unwrap()
    }
}

#[derive(sqlx::FromRow, Debug, Serialize)]
struct OperatorVersion {
    address: String,
    version: String,
}

impl IntoResponse for OperatorVersion {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        axum::http::Response::builder()
            .status(axum::http::StatusCode::OK)
            .body(axum::body::Body::from(
                serde_json::to_string(&self).unwrap(),
            ))
            .unwrap()
    }
}

async fn post_operator_version(
    state: State<AppState>,
    Json(payload): Json<OperatorVersionPayload>,
) -> Result<(), OperatorVersionError> {
    debug!(
        "Received operator version request. Version {}",
        payload.version
    );

    // Recover operator address from signature

    // hash keccak256(version) and recover address from signature
    let version = keccak256(payload.version.as_bytes());

    // check version matches v*.*.* format with regex
    if !regex::Regex::new(r"^v\d+\.\d+\.\d+$")
        .unwrap()
        .is_match(&payload.version)
    {
        return Err(OperatorVersionError::InvalidSignature);
    }

    // decode base64 signature
    let signature = BASE64_STANDARD
        .decode(payload.signature)
        .map_err(|_| OperatorVersionError::InvalidSignature)?;

    if signature.len() != 65 {
        return Err(OperatorVersionError::InvalidSignature);
    }

    let signature = Signature {
        r: ethers::types::U256::from_big_endian(&signature[0..32]),
        s: ethers::types::U256::from_big_endian(&signature[32..64]),
        v: signature[64] as u64,
    };

    let operator_address = signature
        .recover(version)
        .map_err(|_| OperatorVersionError::InvalidSignature)?;

    info!("Operator address: {:?}", operator_address);

    // Check if operator is registered on registry coordinator
    let status = state
        .registry_coordinator
        .get_operator_status(operator_address)
        .await
        .expect("Failed to get operator status");

    if status != 1 {
        return Err(OperatorVersionError::OperatorNotRegistered);
    }

    info!("Operator is registered, updating version");

    // Convert to string
    // Using {:?} because to_string() gives shortened address
    let operator_address = format!("{:?}", operator_address);

    // Store operator version in database
    // - If operator version already exists, update it
    // - If operator version does not exist, insert it
    let row =
        sqlx::query_as::<_, OperatorVersion>("SELECT * FROM operator_versions WHERE address = $1")
            .bind(&operator_address)
            .fetch_optional(&state.pool)
            .await
            .expect("Failed to execute query");

    let query = if let Some(row) = row {
        let version: String = row.version;
        if version == payload.version {
            debug!("Operator {} version already up to date", operator_address);
            return Ok(()); // No need to update
        }

        debug!(
            "Updating operator {} version from {} to {}",
            operator_address, version, payload.version
        );

        sqlx::query("UPDATE operator_versions SET version = $2 WHERE address = $1")
    } else {
        debug!(
            "Inserting operator {} version {}",
            operator_address, payload.version
        );

        sqlx::query("INSERT INTO operator_versions (address, version) VALUES ($1, $2)")
    };

    state
        .pool
        .execute(query.bind(&operator_address).bind(&payload.version))
        .await
        .expect("Failed to update operator version");

    Ok(())
}

async fn list_operator_versions(state: State<AppState>) -> Json<Vec<OperatorVersion>> {
    let rows = sqlx::query_as::<_, OperatorVersion>("SELECT * FROM operator_versions")
        .fetch_all(&state.pool)
        .await
        .expect("Failed to execute query");

    Json(rows)
}
