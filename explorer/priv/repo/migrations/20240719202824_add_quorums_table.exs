defmodule Explorer.Repo.Migrations.AddQuorumsTable do
  use Ecto.Migration

  def change do
    create table("quorums", primary_key: false) do
      add :id, :integer, primary_key: true

      timestamps()
    end

    create table("quorum_strategies", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :quorum_id, references(:quorums, column: :id, on_delete: :delete_all), null: false
      add :strategy_id, references(:strategies, column: :id, on_delete: :delete_all), null: false
      timestamps()
    end
    create unique_index(:quorum_strategies, [:quorum_id, :strategy_id])

  end
end
