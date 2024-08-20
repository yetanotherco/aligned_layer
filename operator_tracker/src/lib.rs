use axum::http::StatusCode;
use axum::{body::Body, response::IntoResponse};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use ethers::providers::{Http, Provider};
use ethers::utils::keccak256;
use ethers::{contract::abigen, core::types::Signature};
use log::{debug, error, info};
use serde::Serialize;
use sqlx::{Executor, PgPool};

abigen!(
    RegistryCoordinator,
    r#"[function getOperatorStatus(address) external view returns (uint8)]"#,
);

pub type RegistryCoordinatorContract = RegistryCoordinator<Provider<Http>>;

#[derive(serde::Deserialize)]
pub struct OperatorVersionPayload {
    pub version: String,
    pub signature: String,
}

#[derive(serde::Serialize, Debug)]
pub enum OperatorVersionError {
    InvalidSignature,
    OperatorNotRegistered,
    InternalServerError,
    BadRequest,
}

impl IntoResponse for OperatorVersionError {
    fn into_response(self) -> axum::http::Response<Body> {
        let err_msg = serde_json::to_string(&self).unwrap_or("Unkown error".to_string());

        let builder = axum::http::Response::builder();
        let builder = match self {
            OperatorVersionError::InvalidSignature => builder
                .status(axum::http::StatusCode::UNAUTHORIZED)
                .body(Body::from(err_msg)),
            OperatorVersionError::InternalServerError => builder
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err_msg)),
            OperatorVersionError::OperatorNotRegistered => builder
                .status(axum::http::StatusCode::BAD_REQUEST)
                .body(Body::from(err_msg)),
            OperatorVersionError::BadRequest => builder
                .status(axum::http::StatusCode::BAD_REQUEST)
                .body(Body::from(err_msg)),
        };

        builder.unwrap_or_default()
    }
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct OperatorVersion {
    address: String,
    pub version: String,
}

impl IntoResponse for OperatorVersion {
    fn into_response(self) -> axum::http::Response<Body> {
        let (status, body) = serialize_or_err(self);

        axum::http::Response::builder()
            .status(status)
            .body(body)
            .unwrap_or_default()
    }
}

pub async fn create_or_update_operator_version(
    db: &PgPool,
    registry_coordinator: &RegistryCoordinatorContract,
    payload: OperatorVersionPayload,
) -> Result<Option<OperatorVersion>, OperatorVersionError> {
    debug!(
        "Received operator version request. Version {}",
        payload.version
    );

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

    // hash keccak256(version) and recover address from signature
    let hashed_version = keccak256(payload.version.as_bytes());

    let operator_address = signature
        .recover(hashed_version)
        .map_err(|_| OperatorVersionError::InvalidSignature)?;

    info!("Operator address: {:?}", operator_address);

    // Check if operator is registered on registry coordinator
    let status = registry_coordinator
        .get_operator_status(operator_address)
        .await
        .map_err(|_| OperatorVersionError::InternalServerError)?;

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
    let row = match get_operator_version(db, &operator_address).await {
        Ok(row) => row,
        Err(_) => return Err(OperatorVersionError::InternalServerError),
    };

    let (query, response) = if let Some(row) = row {
        let version: String = row.version;
        if version == payload.version {
            debug!("Operator {} version already up to date", operator_address);
            return Ok(None); // No need to update
        }

        debug!(
            "Updating operator {} version from {} to {}",
            operator_address, version, payload.version
        );

        (
            sqlx::query("UPDATE operator_versions SET version = $2 WHERE address = $1"),
            None,
        )
    } else {
        debug!(
            "Inserting operator {} version {}",
            operator_address, payload.version
        );

        (
            sqlx::query("INSERT INTO operator_versions (address, version) VALUES ($1, $2)"),
            Some(OperatorVersion {
                address: operator_address.clone(),
                version: payload.version.clone(),
            }),
        )
    };

    db.execute(query.bind(&operator_address).bind(&payload.version))
        .await
        .map_err(|_| OperatorVersionError::InternalServerError)?;

    Ok(response)
}

pub async fn list_operator_versions(
    db: &PgPool,
) -> Result<Vec<OperatorVersion>, OperatorVersionError> {
    sqlx::query_as::<_, OperatorVersion>("SELECT * FROM operator_versions")
        .fetch_all(db)
        .await
        .map_err(|_| OperatorVersionError::InternalServerError)
}

pub async fn get_operator_version(
    db: &PgPool,
    address: &String,
) -> Result<Option<OperatorVersion>, OperatorVersionError> {
    // check operator address is hex and 42 characters long
    if address.len() != 42 || !address.starts_with("0x") {
        return Err(OperatorVersionError::BadRequest);
    }

    sqlx::query_as::<_, OperatorVersion>("SELECT * FROM operator_versions WHERE address = $1")
        .bind(address)
        .fetch_optional(db)
        .await
        .map_err(|_| OperatorVersionError::InternalServerError)
}

pub fn serialize_or_err<T: Serialize>(content: T) -> (StatusCode, Body) {
    match serde_json::to_string(&content) {
        Ok(body) => (StatusCode::OK, Body::from(body)),
        Err(_) => {
            error!("Failed to serialize response");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from("Failed to serialize response"),
            )
        }
    }
}
