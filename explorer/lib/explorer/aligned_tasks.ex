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
    :batchMerkleRoot,
    :taskCreatedBlock,
    :batchDataPointer,
    :responded
  ]
  defstruct [:batchMerkleRoot, :taskCreatedBlock, :batchDataPointer, :responded]
end

# TODO pagination
# defmodule AlignedTaskPageItem do
#   @enforce_keys [
#     :taskId,
#     :transaction_hash,
#     :block_number,
#     :proof_is_responded,
#     :proof_is_correct
#   ]
#   defstruct [:taskId, :transaction_hash, :block_number, :proof_is_responded, :proof_is_correct]
# end
