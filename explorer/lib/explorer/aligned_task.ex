defmodule DASolution do
  def option do [
      {:Calldata, 0},
      {:EigenDA, 1},
      {:Celestia, 2}
    ]
  end
end

defmodule Explorer.DAPayload do
  defstruct solution: {:empty, nil}, #enum DASolution, 0 for calldata, 1 for EigenDA, 2 for Celestia
    proof_associated_data: nil, #bytes ; Proof bytes for calldata - BatchHeaderHash for EigenDA - Commitment for Celestia
    index: nil #uint64 ; BlobIndex for EigenDA - Height for Celestia
end

defmodule Explorer.AlignedTask do
  defstruct provingSystemId: nil, #int
    da_payload: %Explorer.DAPayload{},
    pubInput: nil, #int
    verificationKey: nil, #bytes
    taskCreatedBlock: nil, #uint32
    quorumNumbers: nil, #bytes
    quorumThresholdPercentages: nil, #bytes
    fee: nil #uint256
end

defmodule AlignedTaskCreatedInfo do
  @enforce_keys [:address, :block_hash, :block_number, :taskId, :transaction_hash, :aligned_task]
  defstruct [:address, :block_hash, :block_number, :taskId, :transaction_hash, :aligned_task]
end

defmodule AlignedTaskRespondedInfo do
  @enforce_keys [
    :address,
    :block_hash,
    :block_number,
    :taskId,
    :transaction_hash,
    :proofIsCorrect
  ]
  defstruct [:address, :block_hash, :block_number, :taskId, :transaction_hash, :proofIsCorrect]
end

defmodule AlignedTaskPageItem do
  @enforce_keys [
    :taskId,
    :transaction_hash,
    :block_number,
    :proof_is_responded,
    :proof_is_correct
  ]
  defstruct [:taskId, :transaction_hash, :block_number, :proof_is_responded, :proof_is_correct]
end
