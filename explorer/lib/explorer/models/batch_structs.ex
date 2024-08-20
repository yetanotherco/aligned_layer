# Raised event in batch creation
defmodule NewBatchEvent do
  @enforce_keys [:batchMerkleRoot, :senderAddress, :taskCreatedBlock, :batchDataPointer]
  defstruct [:batchMerkleRoot, :senderAddress, :taskCreatedBlock, :batchDataPointer]

  def extract_merkle_root(event) do
    event.topics_raw |> Enum.at(1)
  end
end

# Blockchain Information about the new batch event
defmodule NewBatchInfo do
  @enforce_keys [:address, :block_number, :block_timestamp, :transaction_hash, :new_batch]
  defstruct [:address, :block_number, :block_timestamp, :transaction_hash, :new_batch]
end

# Blockchain Information about the batch response event
defmodule BatchVerifiedInfo do
  @enforce_keys [:address, :block_number, :block_timestamp, :transaction_hash, :batch_merkle_root, :sender_address]
  defstruct [:address, :block_number, :block_timestamp, :transaction_hash, :batch_merkle_root, :sender_address]
end

# Database model for batches
defmodule BatchDB do
  @enforce_keys [
    :merkle_root,
    :amount_of_proofs,
    :is_verified,
    :submission_block_number,
    :submission_transaction_hash,
    :submission_timestamp,
    :proof_hashes,
    :cost_per_proof,
    :sender_address
  ]
  defstruct [
    :merkle_root,
    :amount_of_proofs,
    :is_verified,
    :submission_block_number,
    :submission_transaction_hash,
    :submission_timestamp,
    :response_block_number,
    :response_transaction_hash,
    :response_timestamp,
    :data_pointer,
    :proof_hashes,
    :cost_per_proof,
    :sender_address
  ]
end
