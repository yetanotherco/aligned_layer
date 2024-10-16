defmodule TelemetryApi.Operators.Operator do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:address, :string, []}
  schema "operators" do
    field :id, :string
    field :stake, :string
    field :name, :string
    field :version, :string
    field :status, :string
    field :eth_rpc_url, :string
    field :eth_rpc_url_fallback, :string
    field :eth_ws_url, :string
    field :eth_ws_url_fallback, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(operator, attrs) do
    operator
    |> cast(attrs, [
      :address,
      :id,
      :stake,
      :name,
      :version,
      :status,
      :eth_rpc_url,
      :eth_rpc_url_fallback,
      :eth_ws_url,
      :eth_ws_url_fallback
    ])
    |> validate_required([:address, :id, :name, :stake])
  end
end

defimpl Phoenix.Param, for: TelemetryApi.Operators.Operator do
  def to_param(%TelemetryApi.Operators.Operator{address: address}), do: address
end
