defmodule Explorer.Repo.Migrations.AddRestakingsTable do
  use Ecto.Migration

  def change do
    create table("restakings", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :staker_id, references(:stakers, column: :id, type: :bigint), null: false
      add :amount, :decimal, precision: 22, scale: 0, null: false
      add :tx_id, references(:transactions, column: :id, type: :bigint), null: false

      timestamps()
    end
  end
end
