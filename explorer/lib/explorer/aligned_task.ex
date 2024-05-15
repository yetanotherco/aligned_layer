defmodule DASolution do
  def option do
    [
      {:Calldata, 0},
      {:EigenDA, 1},
      {:Celestia, 2}
    ]
  end
end

defmodule Explorer.DAPayload do
  # enum DASolution, 0 for calldata, 1 for EigenDA, 2 for Celestia
  defstruct solution: {:empty, nil},
            # bytes ; Proof bytes for calldata - BatchHeaderHash for EigenDA - Commitment for Celestia
            proof_associated_data: nil,
            # uint64 ; BlobIndex for EigenDA - Height for Celestia
            index: nil
end

defmodule Explorer.AlignedTask do
  # int
  defstruct provingSystemId: nil,
            da_payload: %Explorer.DAPayload{},
            # int
            pubInput: nil,
            # bytes
            verificationKey: nil,
            # uint32
            taskCreatedBlock: nil,
            # bytes
            quorumNumbers: nil,
            # bytes
            quorumThresholdPercentages: nil,
            # uint256
            fee: nil
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
