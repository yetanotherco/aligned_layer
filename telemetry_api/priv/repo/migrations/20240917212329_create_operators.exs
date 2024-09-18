defmodule TelemetryApi.Repo.Migrations.CreateOperators do
  use Ecto.Migration

  def change do
    create table(:operators) do
      add :address, :string
      add :version, :string

      timestamps(type: :utc_datetime)
    end
  end
end
