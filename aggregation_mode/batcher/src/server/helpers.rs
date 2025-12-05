use crate::db::Receipt;

pub(super) fn format_merkle_paths(
    paths: Vec<Receipt>,
) -> Result<Vec<(Receipt, Vec<String>)>, String> {
    paths
        .into_iter()
        .map(|receipt| {
            if let Some(merkle_path) = &receipt.merkle_path {
                if merkle_path.is_empty() {
                    return Ok((receipt, vec![]));
                }
                if merkle_path.len() % 32 != 0 {
                    return Err("merkle path length is not a multiple of 32 bytes".into());
                }

                let formatted_merkle_path = merkle_path
                    .chunks(32)
                    .map(|chunk| format!("0x{}", hex::encode(chunk)))
                    .collect();

                Ok((receipt, formatted_merkle_path))
            } else {
                Ok((receipt, vec![]))
            }
        })
        .collect()
}
