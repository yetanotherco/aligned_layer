use ethers::{abi::ethereum_types::FromDecStrErr, types::U256};

/// Encodes Circom public inputs into a single byte vector.
///
/// Circom normally outputs public inputs as a JSON array of strings, where each
/// entry is actually a big integer represented in decimal. For example:
///
/// ```json
/// { "pubInputs": ["123", "456", "789"] }
/// ```
///
/// For on-chain usage (e.g. in Solidity), working with JSON is inefficient.
/// Instead, we prefer the **raw form**: each input is converted to a 32-byte
/// big-endian encoding of the integer, concatenated together. This makes it
/// simple to compute commitments and verify proofs on-chain, since the contract
/// receives a compact `bytes` array rather than parsing JSON.
///
/// Each input must fit in 32 bytes (i.e. < 2^256).
pub fn encode_circom_pub_inputs(input: &[String]) -> Result<Vec<u8>, FromDecStrErr> {
    let mut out = Vec::with_capacity(input.len() * 32);

    for s in input {
        // parse as decimal (base 10). Use from_str_radix(s, 16) if they're hex.
        let n = U256::from_dec_str(s)?;
        let mut bytes = [0u8; 32];
        n.to_big_endian(&mut bytes);

        out.extend_from_slice(&bytes);
    }

    Ok(out)
}
