defmodule Explorer.Repo.Migrations.CreateAggregatedProofs do
  use Ecto.Migration

  def change do
    create table(:aggregated_proofs, primary_key: false) do
      add(:merkle_root, :string, primary_key: true)
      add(:tx_hash, :string)
      add(:blob_versioned_hash, :string)
      add(:number_of_proofs, :integer)
      add(:block_number, :integer)
      add(:block_timestamp, :utc_datetime)

      timestamps()
    end

    create table(:proofs_agg_mode) do
      add(
        :merkle_root,
        references(:aggregated_proofs,
          column: :merkle_root,
          type: :string,
          on_delete: :delete_all
        )
      )

      add(:proof_hash, :string)
      add(:index, :integer)

      timestamps()
    end
  end
end
