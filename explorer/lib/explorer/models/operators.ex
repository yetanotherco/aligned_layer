defmodule Operators do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  schema "operators" do
    field :name, :string
    field :address, :binary

    timestamps()
  end

  @doc false
  def changeset(operator, attrs) do
    operator
    |> cast(attrs, [:name, :address])
    |> validate_required([:name, :address])
  end

end
