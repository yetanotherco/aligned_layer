defmodule Explorer.Repo.Migrations.RenameColumnsInBatches do
  use Ecto.Migration

  def change do
    rename table("batches"), :submition_block_number, to: :submission_block_number
    rename table("batches"), :submition_transaction_hash, to: :submission_transaction_hash
    rename table("batches"), :submition_timestamp, to: :submission_timestamp
  end
end
