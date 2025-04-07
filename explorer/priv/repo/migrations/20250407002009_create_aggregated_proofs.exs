defmodule Explorer.Repo.Migrations.CreateAggregatedProofs do
  use Ecto.Migration

  def change do
    create table(:aggregated_proofs, primary_key: false) do
      add(:number, :integer, primary_key: true)
      add(:merkle_root, :string)
      add(:status, :integer)
      add(:tx_hash, :string)
      add(:blob_versioned_hash, :string)
      add(:number_of_proofs, :integer)
      add(:block_number, :integer)

      timestamps()
    end

    create table(:proofs_agg_mode) do
      add(
        :aggregated_proof_number,
        references(:aggregated_proofs, column: :number, type: :integer, on_delete: :delete_all)
      )

      add(:proof_hash, :string)
      add(:index, :integer)

      timestamps()
    end

    create(index(:proofs_agg_mode, [:aggregated_proof_number]))
  end
end
