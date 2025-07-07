use std::io::Read;

use ciborium::de::Error as CiboriumDeError;
use ciborium::ser::Error as CiboriumSerError;
use serde::{de::DeserializeOwned, Serialize};
use std::io::Error;

pub type CiboriumSerializationError = CiboriumSerError<Error>;
pub type CiboriumDeserializationError = CiboriumDeError<Error>;

pub fn cbor_serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, CiboriumSerializationError> {
    let mut buf = Vec::new();
    ciborium::into_writer(value, &mut buf)?;
    Ok(buf)
}

pub fn cbor_deserialize<R: Read, T: DeserializeOwned>(
    buf: R,
) -> Result<T, CiboriumDeserializationError> {
    ciborium::from_reader(buf)
}
