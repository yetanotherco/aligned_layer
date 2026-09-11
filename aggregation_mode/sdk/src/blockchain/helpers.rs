pub fn decoded_blob(blob_data: &[u8]) -> Vec<[u8; 32]> {
    let mut proof_hashes = vec![];

    let mut current_hash = [0u8; 32];
    let mut current_hash_count = 0;
    let mut total_bytes_count = 0;

    while total_bytes_count < blob_data.len() {
        // Every 32 bytes there is a 0x0 acting as padding, so we need to skip the byte
        let is_pad = total_bytes_count % 32 == 0;
        if is_pad {
            total_bytes_count += 1;
            continue;
        }

        current_hash[current_hash_count] = blob_data[total_bytes_count];

        if current_hash_count + 1 == 32 {
            // if the current_hash is the zero hash, then there are no more proofs in the blob
            if current_hash == [0u8; 32] {
                break;
            }
            proof_hashes.push(current_hash);
            current_hash = [0u8; 32];
            current_hash_count = 0;
        } else {
            current_hash_count += 1;
        }

        total_bytes_count += 1;
    }

    proof_hashes
}
