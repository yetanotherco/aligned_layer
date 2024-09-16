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
      add :total_stake, :decimal, precision: 30, scale: 0, null: false, default: 0

      timestamps()
    end

    create unique_index("operators", [:id], name: :operator_id_index)

  end
end
