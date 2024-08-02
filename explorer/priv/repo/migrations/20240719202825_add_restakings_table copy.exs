defmodule Explorer.Repo.Migrations.AddRestakingsTable do
  use Ecto.Migration

  def change do
    create table("restakings", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :operator_address, references(:operators, column: :address, type: :binary), null: false
      add :amount, :decimal, precision: 22, scale: 0, null: false

      timestamps()
    end
  end
end
