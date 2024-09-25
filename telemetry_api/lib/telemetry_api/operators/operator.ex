defmodule TelemetryApi.Operators.Operator do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:address, :string, []}
  schema "operators" do
    field :version, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(operator, attrs) do
    operator
    |> cast(attrs, [:address, :version])
    |> validate_required([:address, :version])
  end
end

defimpl Phoenix.Param, for: TelemetryApi.Operators.Operator do
  def to_param(%TelemetryApi.Operators.Operator{address: address}), do: address
end
