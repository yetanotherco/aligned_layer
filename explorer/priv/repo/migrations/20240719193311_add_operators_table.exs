defmodule Explorer.Repo.Migrations.AddOperatorsTable do
  use Ecto.Migration

  def change do
    create table("operators", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :name, :string, null: false
      add :address, :binary, null: false
      add :URI, :string, null: false

      timestamps()
    end
  end
end
