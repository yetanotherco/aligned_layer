defmodule Explorer.Repo.Migrations.AddRestakingsTable do
  use Ecto.Migration

  def change do
    create table("restakings", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :operator_id, references(:operators, column: :id, type: :binary), null: false
      add :operator_address, references(:operators, column: :address, type: :binary)
      add :stake, :decimal, precision: 30, scale: 0, null: false
      add :quorum_number, references(:quorums, column: :id, type: :integer), null: false
      add :strategy_address, references(:strategies, column: :strategy_address, type: :binary), null: false

      timestamps()
    end

  end
end
