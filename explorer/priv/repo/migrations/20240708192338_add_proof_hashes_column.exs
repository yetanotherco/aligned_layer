defmodule Explorer.Repo.Migrations.AddProofHashesColumn do
  use Ecto.Migration

  def change do
    alter table("batches") do # add new columns
      add :proof_hashes, {:array, :string}
    end
  end
end
