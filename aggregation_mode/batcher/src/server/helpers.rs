use crate::{db::Receipt, server::types::GetReceiptsResponse};

pub(super) fn format_merkle_paths(paths: Vec<Receipt>) -> Result<Vec<GetReceiptsResponse>, String> {
    paths
        .into_iter()
        .map(|receipt| {
            if let Some(merkle_path) = &receipt.merkle_path {
                if merkle_path.is_empty() {
                    return Ok(GetReceiptsResponse {
                        status: receipt.status,
                        merkle_path: Vec::new(),
                        nonce: receipt.nonce,
                        address: receipt.address,
                    });
                }
                if merkle_path.len() % 32 != 0 {
                    return Err("merkle path length is not a multiple of 32 bytes".into());
                }

                let formatted_merkle_path = merkle_path
                    .chunks(32)
                    .map(|chunk| format!("0x{}", hex::encode(chunk)))
                    .collect();

                Ok(GetReceiptsResponse {
                    status: receipt.status,
                    merkle_path: formatted_merkle_path,
                    nonce: receipt.nonce,
                    address: receipt.address,
                })
            } else {
                Ok(GetReceiptsResponse {
                    status: receipt.status,
                    merkle_path: Vec::new(),
                    nonce: receipt.nonce,
                    address: receipt.address,
                })
            }
        })
        .collect()
}
