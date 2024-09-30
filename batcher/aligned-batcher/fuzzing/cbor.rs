extern crate honggfuzz;

use aligned_sdk::{
    communication::communication::serialization::{cbor_deserialize, cbor_serialize},
    core::types::AlignedVerificationData,
};
use arbitrary::Arbitrary;

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            let marshalled = cbor_serialize(data);
            let unmarshalled = cbor_deserialize(marshalled);
            assert_eq!(data, &unmarshalled);
        });
    }
}
