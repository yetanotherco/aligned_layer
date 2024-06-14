defmodule Explorer.Repo.Migrations.Batch do
use Ecto.Migration

  def change do
    create table("batches", primary_key: false) do
      add :merkle_root, :string, size: 66, primary_key: true, null: false
      add :amount_of_proofs, :integer, null: false
      add :is_verified, :boolean, null: false

      timestamps()
    end

    create unique_index("batches", [:merkle_root])
    create index("batches", [:is_verified])
    
  end
end
