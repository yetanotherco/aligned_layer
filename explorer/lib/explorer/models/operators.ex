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
    field :total_stake, :decimal

    timestamps()
  end

  @doc false
  def changeset(operator, attrs) do
    operator
    |> cast(attrs, [:address, :id, :name, :url, :website, :description, :logo_link, :twitter, :is_active, :total_stake])
    |> validate_required([:address, :id, :url, :is_active, :total_stake])
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

  def get_operator_by_id(id) do
    query = from(o in Operators, where: o.id == ^id, select: o)
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

  def handle_operator_registration(event) do
    operator_address = Enum.at(event.topics, 1)
    operator_id = RegistryCoordinatorManager.get_operator_id_from_chain(operator_address)
    operator_url = DelegationManager.get_operator_url(operator_address)
    dbg operator_url
    operator_metadata = case Utils.fetch_eigen_operator_metadata(operator_url) do
      {:ok, operator_metadata} ->
        operator_metadata

      {:error, reason} ->
        case reason do
          %Jason.DecodeError{} ->
            dbg("Error decoding operator metadata: operator link does not contain a JSON")
          _ ->
            dbg("Error fetching operator metadata:", reason)
        end
        %EigenOperatorMetadataStruct{name: nil, website: nil, description: nil, logo: nil, twitter: nil}
    end
    total_stake = StakeRegistryManager.get_stake_of_quorum_for_operator(%Restakings{operator_address: operator_address})
    register_or_update_operator(%Operators{id: operator_id, name: operator_metadata.name, address: operator_address, url: operator_url, website: operator_metadata.website, description: operator_metadata.description, logo_link: operator_metadata.logo, twitter: operator_metadata.twitter, is_active: true, total_stake: total_stake})
  end

  def handle_operator_unregistration(event) do
    unregister_operator(%Operators{address: Enum.at(event.topics, 1)})
  end

  def unregister_operator(%Operators{address: address}) do
    query = from(o in Operators, where: o.address == ^address)
    Explorer.Repo.update_all(query, set: [is_active: false])
  end

  def get_total_stake(%Operators{} = operator) do
    query = from(o in Operators, where: o.address == ^operator.address, select: o.total_stake)
    Explorer.Repo.one(query)
  end

end
