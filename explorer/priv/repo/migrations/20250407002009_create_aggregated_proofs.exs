defmodule Explorer.Repo.Migrations.CreateAggregatedProofs do
  use Ecto.Migration

  def change do
    create table(:aggregated_proofs, primary_key: false) do
      add(:id, :binary_id, primary_key: true)
      add(:merkle_root, :string)
      add(:tx_hash, :string)
      add(:blob_versioned_hash, :string)
      add(:number_of_proofs, :integer)
      add(:block_number, :integer)
      add(:block_timestamp, :utc_datetime)

      timestamps()
    end

    create table(:proofs_agg_mode) do
      add(
        :agg_proof_id,
        references(:aggregated_proofs,
          column: :id,
          type: :binary_id,
          on_delete: :delete_all
        )
      )

      add(:proof_hash, :string)
      add(:index, :integer)

      timestamps()
    end
  end
end
