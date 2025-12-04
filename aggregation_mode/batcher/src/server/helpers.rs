pub(super) fn format_merkle_path(bytes: &[u8]) -> Result<Vec<String>, String> {
    if bytes.is_empty() {
        return Ok(vec![]);
    }

    if bytes.len() % 32 != 0 {
        return Err("merkle path length is not a multiple of 32 bytes".into());
    }

    Ok(bytes
        .chunks(32)
        .map(|chunk| format!("0x{}", hex::encode(chunk)))
        .collect())
}
