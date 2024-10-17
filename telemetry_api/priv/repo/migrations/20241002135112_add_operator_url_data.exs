defmodule TelemetryApi.Repo.Migrations.AddOperatorUrlData do
  use Ecto.Migration

  def change do
    alter table(:operators) do
      add :eth_rpc_url, :string
      add :eth_rpc_url_fallback, :string
      add :eth_ws_url, :string
      add :eth_ws_url_fallback, :string
    end
  end
end
