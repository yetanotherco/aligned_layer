defmodule Explorer.Repo.Migrations.AddAggregatorToAggProofs do
  use Ecto.Migration

  def change do
    alter table(:aggregated_proofs) do
      add(:aggregator, :string, default: nil)
    end
  end
end
