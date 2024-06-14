# struct BatchState {
#   uint32 taskCreatedBlock;
#   bool responded;
# }
defmodule BatchState do
  @enforce_keys [:taskCreatedBlock, :responded]
  defstruct [:taskCreatedBlock, :responded]
end

# event NewBatch(
#   bytes32 batchMerkleRoot,
#   uint32 taskCreatedBlock,
#   string batchDataPointer
# );
defmodule NewBatchEvent do
  @enforce_keys [:batchMerkleRoot, :taskCreatedBlock, :batchDataPointer]
  defstruct [:batchMerkleRoot, :taskCreatedBlock, :batchDataPointer]

  def extract_merkle_root(event) do
    event.topics_raw |> Enum.at(1)
  end
end

# %Aligned.NewBatchInfo{
#   address: event |> Map.get(:address),
#   block_hash: event |> Map.get(:block_hash),
#   block_number: event |> Map.get(:block_number),
#   transaction_hash: event |> Map.get(:transaction_hash),
#   new_batch: new_batch
# }
defmodule NewBatchInfo do
  @enforce_keys [:address, :block_hash, :block_number, :transaction_hash, :new_batch]
  defstruct [:address, :block_hash, :block_number, :transaction_hash, :new_batch]
end

# event BatchVerified(
#   bytes32 batchMerkleRoot
# );
defmodule BatchVerifiedEvent do
  @enforce_keys [:batchMerkleRoot]
  defstruct [:batchMerkleRoot]
end

# %Aligned.BatchVerifiedInfo{
#   address: event |> Map.get(:address),
#   block_hash: event |> Map.get(:block_hash),
#   block_number: event |> Map.get(:block_number),
#   transaction_hash: event |> Map.get(:transaction_hash),
#   batch_verified: batch_verified
# }
defmodule BatchVerifiedInfo do
  @enforce_keys [:address, :block_hash, :block_number, :transaction_hash, :batch_verified]
  defstruct [:address, :block_hash, :block_number, :transaction_hash, :batch_verified]
end


defmodule BatchPageItem do
  @enforce_keys [
    :batch_merkle_root,
    :task_created_block_number,
    :task_created_tx_hash,
    :task_responded_block_number,
    :task_responded_tx_hash,
    :batch_data_pointer,
    :responded
  ]
  defstruct [:batch_merkle_root, :task_created_block_number, :task_created_tx_hash, :task_responded_block_number, :task_responded_tx_hash, :batch_data_pointer, :responded]
end

defmodule Batch do
  @enforce_keys [:batch_merkle_root, :batch_data_pointer, :is_verified]
  defstruct [:batch_merkle_root, :batch_data_pointer, :is_verified]
end

defmodule BatchDB do
  @enforce_keys [:batch_merkle_root, :amount_of_proofs, :is_verified]
  defstruct [:batch_merkle_root, :amount_of_proofs, :is_verified]
end
