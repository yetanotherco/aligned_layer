defmodule Explorer.Repo.Migrations.AddOperatorsTable do
  use Ecto.Migration

  def change do
    create table("operators", primary_key: false) do
      add :address, :binary, primary_key: true
      add :id, :binary
      add :name, :string
      add :url, :string, null: false
      add :website, :string
      add :description, :text
      add :logo_link, :string
      add :twitter, :string
      add :is_active, :boolean, default: false

      timestamps()
    end
  end
end
