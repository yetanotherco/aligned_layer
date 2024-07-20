defmodule Restakings do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  schema "restakings" do
    field :operator_id, :bigint
    field :amount, :decimal

    timestamps()
  end

  @doc false
  def changeset(restaking, attrs) do
    restaking
    |> cast(attrs, [:operator_id, :amount])
    |> validate_required([:operator_id, :amount])
  end

end
