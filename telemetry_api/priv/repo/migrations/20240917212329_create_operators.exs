defmodule TelemetryApi.Repo.Migrations.CreateOperators do
  use Ecto.Migration

  def change do
    create table(:operators) do
      add :address, :string, primary_key: true
      add :version, :string

      timestamps(type: :utc_datetime)
    end

    create unique_index(:operators, [:address])
  end
end
