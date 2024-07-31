defmodule Explorer.Repo.Migrations.AddTokensTable do
  use Ecto.Migration

  def change do
    create table("tokens", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :name, :string, null: false
      add :symbol, :string, null: false
      add :address, :binary, null: false
      add :total_staked, :decimal, precision: 22, scale: 0, null: false

      timestamps()
    end
  end
end
