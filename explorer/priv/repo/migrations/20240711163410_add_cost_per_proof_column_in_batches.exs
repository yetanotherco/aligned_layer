defmodule Explorer.Repo.Migrations.AddCostPerProofColumnInBatches do
  use Ecto.Migration

  def change do
    alter table("batches") do # add new columns
      add :cost_per_proof, :integer
    end
  end
end
