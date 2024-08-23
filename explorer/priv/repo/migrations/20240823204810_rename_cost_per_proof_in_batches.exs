defmodule Explorer.Repo.Migrations.AddWeatherTable do
  use Ecto.Migration

  def change do
    rename table("batches"), :cost_per_proof, to: :fee_per_proof
  end
end
