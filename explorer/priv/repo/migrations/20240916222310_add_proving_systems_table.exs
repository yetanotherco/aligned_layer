defmodule Explorer.Repo.Migrations.AddProvingSystemsTable do
  use Ecto.Migration

  def change do
    create table("proving_systems", primary_key: false) do

      timestamps()
    end

    create unique_index("operators", [:id], name: :operator_id_index)

  end
end
