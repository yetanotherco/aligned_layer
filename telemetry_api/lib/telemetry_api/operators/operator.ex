defmodule TelemetryApi.Operators.Operator do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:address, :string, []}
  schema "operators" do
    field :id, :string
    field :stake, :string
    field :name, :string
    field :version, :string
    field :status, :integer

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(operator, attrs) do
    operator
    |> cast(attrs, [:address, :id, :stake, :name, :version, :status])
    |> validate_required([:address, :id, :name, :stake])
  end
end

defimpl Phoenix.Param, for: TelemetryApi.Operators.Operator do
  def to_param(%TelemetryApi.Operators.Operator{address: address}), do: address
end
