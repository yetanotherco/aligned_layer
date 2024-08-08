defmodule Operators do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  @primary_key {:address, :binary, autogenerate: false}
  schema "operators" do
    field :id, :binary
    field :name, :string
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
    |> cast(attrs, [:address, :id, :name, :url, :website, :description, :logo_link, :twitter, :is_active])
    |> validate_required([:address, :id, :url, :is_active])
    |> unique_constraint(:address)
    |> unique_constraint(:id)
  end

  def generate_changeset(%Operators{} = operator) do
    Operators.changeset(%Operators{}, Map.from_struct(operator))
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
      select: count(o.address)
    )
    Explorer.Repo.one(query)
  end

  def register_or_update_operator(%Operators{} = operator) do
    changeset = case Operators.generate_changeset(operator) do
      %Ecto.Changeset{valid?: false} = changeset ->
        dbg("Invalid changeset: #{inspect(changeset)}")
        :nil
      changeset ->
        changeset
    end
    case Explorer.Repo.get_by(Operators, address: operator.address) do
      nil ->
        dbg("Inserting new operator")
        Explorer.Repo.insert(changeset)

      existing_operator ->
        dbg("Updating operator")
        Ecto.Changeset.change(existing_operator, changeset.changes)
        |> Explorer.Repo.update()
    end
  end

  def unregister_operator(%Operators{address: address}) do
    query = from(o in Operators, where: o.address == ^address)
    Explorer.Repo.update_all(query, set: [is_active: false])
  end

end
