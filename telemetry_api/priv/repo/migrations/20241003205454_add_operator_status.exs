defmodule TelemetryApi.Repo.Migrations.AddOperatorStatus do
  use Ecto.Migration

  def change do
    alter table(:operators) do
      add :status, :string
    end
  end
end
