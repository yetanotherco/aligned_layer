use aligned_sdk::common::types::ProvingSystemId;
use alloy::{hex, primitives::Keccak256};

fn main() {
    let vk_bytes =
        std::fs::read("circuits/verification_key.json").expect("verification key to be created");

    let mut hasher = Keccak256::new();
    hasher.update(vk_bytes);
    hasher.update([ProvingSystemId::CircomGroth16Bn256 as u8]);
    let vk_commitment = hasher.finalize().0;

    println!("VK COMMITMENT IS: `0x{}`", hex::encode(vk_commitment));
}
