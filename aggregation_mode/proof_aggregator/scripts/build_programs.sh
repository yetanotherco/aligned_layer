# Get the program ids
cargo run --release --bin write_program_image_id_vk_hash

# Copy the user proofs aggregator program ids and put them on the root aggregator
SP1_USER_PROOFS_AGGREGATOR_VK_HASH_WORDS=`jq -r .sp1_user_proofs_aggregator_vk_hash_words programs_ids.json`
SP1_LINE="pub const USER_PROOFS_AGGREGATOR_PROGRAM_VK_HASH: [u32; 8] = $SP1_USER_PROOFS_AGGREGATOR_VK_HASH_WORDS;\n"
sed -i '' -e "/^pub const USER_PROOFS_AGGREGATOR_PROGRAM_VK_HASH.*/{N;N;s|.*|$SP1_LINE|;}" aggregation_programs/sp1/src/chunk_aggregator_main.rs

RISC0_USER_PROOFS_AGGREGATOR_IMAGE_ID_BYTES=`jq -r .risc0_user_proofs_aggregator_image_id_bytes programs_ids.json`
RISC0_LINE="pub const USER_PROOFS_AGGREGATOR_PROGRAM_IMAGE_ID: [u8; 32] = $RISC0_USER_PROOFS_AGGREGATOR_IMAGE_ID_BYTES;\n"
sed -i '' -e "/^pub const USER_PROOFS_AGGREGATOR_PROGRAM_IMAGE_ID.*/{N;N;N;s|.*|$RISC0_LINE|;}" aggregation_programs/risc0/src/chunk_aggregator_main.rs

cd aggregation_programs
cargo fmt --all

cd .. 
# Re compute the program ids
cargo run --release --bin write_program_image_id_vk_hash
