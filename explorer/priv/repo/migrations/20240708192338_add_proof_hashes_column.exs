defmodule Explorer.Repo.Migrations.AddProofHashesColumn do
  use Ecto.Migration

  def change do
    # alter table("batches") do # add new columns
    #   add :proof_hashes, {:array, :string}
    # end
    create table("proofs", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :batch_merkle_root, references(:batches, column: :merkle_root, type: :string, size: 66), null: false
      add :proof_hash, :binary

      timestamps()
    end
  end
end
