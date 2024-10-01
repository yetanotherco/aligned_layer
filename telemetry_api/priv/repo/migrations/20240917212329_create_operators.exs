defmodule TelemetryApi.Repo.Migrations.CreateOperators do
  use Ecto.Migration

  def change do
    create table(:operators, primary_key: false) do
      add :address, :string, primary_key: true
      add :id, :string
      add :stake, :string
      add :name, :string
      add :version, :string
      add :eth_rpc_url, :string
      add :eth_rpc_url_fallback, :string
      add :eth_ws_url, :string
      add :eth_ws_url_fallback, :string

      timestamps(type: :utc_datetime)
    end

    create unique_index(:operators, [:address])
  end
end
