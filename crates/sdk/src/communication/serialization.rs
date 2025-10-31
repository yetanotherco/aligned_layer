use std::io::Read;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub fn cbor_serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, SerializationError> {
    let mut buf = Vec::new();
    ciborium::into_writer(value, &mut buf).map_err(|_| SerializationError)?;
    Ok(buf)
}

pub fn cbor_deserialize<R: Read, T: DeserializeOwned>(buf: R) -> Result<T, SerializationError> {
    ciborium::from_reader(buf).map_err(|_| SerializationError)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerializationError;

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Serialization error")
    }
}
