use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
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
use log::info;
use sqlx::postgres::{PgPool, PgPoolOptions};

const PG_CONNECTION_STRING: &str = "postgres://postgres:admin@localhost:5432/postgres";
const RPC_URL: &str = "http://localhost:8545";
const REGISTRY_COORDINATOR_ADDRESS: &str = "0x851356ae760d987E095750cCeb3bC6014560891C";

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

#[tokio::main]
async fn main() {
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(PG_CONNECTION_STRING)
        .await
        .expect("Failed to create connection pool");

    let eth_rpc = Arc::new(Provider::<Http>::connect(RPC_URL).await);

    let registry_coordinator_addr: Address = REGISTRY_COORDINATOR_ADDRESS
        .parse()
        .expect("Failed to parse registry coordinator addr");

    let registry_coordinator = RegistryCoordinator::new(registry_coordinator_addr, eth_rpc.clone());

    let state = AppState {
        pool,
        registry_coordinator,
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind listener");

    let router = Router::new()
        .route("/version", post(operator_version))
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

async fn operator_version(
    state: State<AppState>,
    Json(payload): Json<OperatorVersionPayload>,
) -> Result<(), OperatorVersionError> {
    // Recover operator address from signature

    // hash keccak256(version) and recover address from signature
    let version = keccak256(payload.version.as_bytes());

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

    info!("Operator is registered");

    // TODO: Store operator version in database

    Ok(())
}
