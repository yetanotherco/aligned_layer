defmodule Operators do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  schema "operators" do
    field :name, :string
    field :address, :binary
    field :url, :string
    field :website, :string
    field :description, :string
    field :logo_link, :string
    field :twitter, :string
    field :is_active, :boolean

    timestamps()
  end

  @doc false
  def changeset(operator, attrs) do
    operator
    |> cast(attrs, [:name, :address, :url, :website, :description, :logo_link, :twitter, :is_active])
    |> validate_required([:address, :url, :is_active])
    |> unique_constraint(:address)
  end

  def get_operator_by_address(address) do
    query = from(o in Operators, where: o.address == ^address, select: o)
    Explorer.Repo.one(query)
  end

  # TODO: add pagination
  def get_operators() do
    query = from(o in Operators, select: o)
    Explorer.Repo.all(query)
  end

  def get_amount_of_operators do
    query = from(
      o in Operators,
      where: o.is_active == true,
      select: count(o.id)
    )
    Explorer.Repo.one(query)
  end

  # def register_operator(%Operators{} = operator) do
  #   Explorer.Repo.insert(operator)
  # end

  def register_operator(%Operators{} = operator) do
    Operators.changeset(operator, %{}) |> Explorer.Repo.insert()
  end

  def unregister_operator(%Operators{address: address}) do
    query = from(o in Operators, where: o.address == ^address)
    #TODO delete? or update status? also update their stake?
    Explorer.Repo.delete(query)
  end

end
