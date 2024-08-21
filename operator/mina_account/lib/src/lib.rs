use kimchi::o1_utils::FieldHelpers;
use merkle_verifier::verify_merkle_proof;
use mina_curves::pasta::Fp;
use mina_tree::MerklePath;

mod merkle_verifier;

// TODO(xqft): check sizes
const MAX_PROOF_SIZE: usize = 16 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 6 * 1024;
const HASH_SIZE: usize = 32;

#[no_mangle]
pub extern "C" fn verify_account_inclusion_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    public_input_bytes: &[u8; MAX_PUB_INPUT_SIZE],
    public_input_len: usize,
) -> bool {
    // TODO(xqft): we need to send account data as part of the proof. This way
    // some account fields (like public key) can be included in the public inputs
    // and validated in this verifier. A smart contract could implement Poseidon
    // and hash the data itself but it's prohibitively expensive.

    let (merkle_root, account_hash) =
        match parse_pub_inputs(&public_input_bytes[..public_input_len]) {
            Ok(pub_inputs) => pub_inputs,
            Err(err) => {
                eprintln!("Failed to parse public inputs: {}", err);
                return false;
            }
        };

    let merkle_proof = match parse_proof(&proof_bytes[..proof_len]) {
        Ok(proof) => proof,
        Err(err) => {
            eprintln!("Failed to parse merkle proof: {}", err);
            return false;
        }
    };

    verify_merkle_proof(account_hash, merkle_proof, merkle_root)
}

pub fn parse_hash(pub_inputs: &[u8], offset: &mut usize) -> Result<Fp, String> {
    let hash = pub_inputs
        .get(*offset..*offset + HASH_SIZE)
        .ok_or("Failed to slice candidate hash".to_string())
        .and_then(|bytes| Fp::from_bytes(bytes).map_err(|err| err.to_string()))?;

    *offset += HASH_SIZE;

    Ok(hash)
}

pub fn parse_pub_inputs(pub_inputs: &[u8]) -> Result<(Fp, Fp), String> {
    let mut offset = 0;

    let merkle_root = parse_hash(pub_inputs, &mut offset)?;
    let account_hash = parse_hash(pub_inputs, &mut offset)?;

    Ok((merkle_root, account_hash))
}

pub fn parse_proof(proof_bytes: &[u8]) -> Result<Vec<MerklePath>, String> {
    let merkle_path_bytes = proof_bytes.chunks_exact(HASH_SIZE + 1);

    if !merkle_path_bytes.remainder().is_empty() {
        return Err(format!(
            "Merkle path bytes not a multiple of HASH_SIZE + 1 ({})",
            HASH_SIZE + 1
        ));
    }

    merkle_path_bytes
        .map(|bytes| {
            let left_or_right = bytes
                .first()
                .ok_or("left_or_right byte not found".to_string())?;
            let hash = Fp::from_bytes(&bytes[1..]).map_err(|err| {
                format!("Failed to convert merkle hash into field element: {err}")
            })?;
            match left_or_right {
                0 => Ok(MerklePath::Left(hash)),
                1 => Ok(MerklePath::Right(hash)),
                _ => Err("Unexpected left_or_right byte".to_string()),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {}
