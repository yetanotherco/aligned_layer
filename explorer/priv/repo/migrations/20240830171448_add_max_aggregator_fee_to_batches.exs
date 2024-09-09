defmodule Explorer.Repo.Migrations.AddMaxAggregatorFeeToBatches do
  use Ecto.Migration

  def change do
    alter table("batches") do
      add :max_aggregator_fee, :decimal, precision: 30, scale: 0, default: nil
    end
  end
end
