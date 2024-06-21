use ethers::core::k256::sha2::Digest;
use sha3::Keccak256;

pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn hash_with_hasher(data: &[u8], hasher: &mut Keccak256) -> [u8; 32] {
    hasher.update(data);
    hasher.finalize_reset().into()
}
