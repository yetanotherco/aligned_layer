defmodule Explorer.Repo.Migrations.AddOperatorsTable do
  use Ecto.Migration

  def change do
    create table("operators", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :name, :string
      add :address, :binary, null: false
      add :url, :string, null: false
      add :website, :string
      add :description, :text
      add :logo_link, :string
      add :twitter, :string

      timestamps()
    end
  end
end
