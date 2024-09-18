defmodule TelemetryApi.Operators.Operator do
  use Ecto.Schema
  import Ecto.Changeset

  schema "operators" do
    field :version, :string
    field :address, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(operator, attrs) do
    operator
    |> cast(attrs, [:address, :version])
    |> validate_required([:address, :version])
  end
end
