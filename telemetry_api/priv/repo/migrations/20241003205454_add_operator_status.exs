defmodule TelemetryApi.Repo.Migrations.AddOperatorStatus do
  use Ecto.Migration

  def change do
    alter table(:operators) do
      add :active, :boolean, default: false
    end
  end
end
