defmodule Restakings do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  schema "restakings" do
    field :operator_id, :binary
    field :stake, :decimal
    field :quorum_number, :integer

    timestamps()
  end

  @doc false
  def changeset(restaking, attrs) do
    restaking
    |> cast(attrs, [:operator_id, :stake, :quorum_number])
    |> validate_required([:operator_id, :stake, :quorum_number])
  end

  def generate_changeset(%Restakings{} = restaking) do
    Restakings.changeset(%Restakings{}, Map.from_struct(restaking))
  end

  def process_restaking_changes(%{fromBlock: from_block}) do
    Operators.get_operators()
      |> Enum.map(fn operator -> StakeRegistryManager.get_latest_stake_update(%{fromBlock: from_block, operator_id: operator.id}) end)
      |> Enum.map(&parse_stake_update_event/1)
      |> Enum.map(&insert_or_update_restakings/1)
  end

  def parse_stake_update_event(%Ethers.Event{} = event) do
    %Restakings{
      operator_id: Enum.at(event.topics, 1),
      quorum_number: Enum.at(event.data, 0),
      stake: Enum.at(event.data, 1)
    }
  end

  def insert_or_update_restakings(%Restakings{} = restaking) do
    dbg restaking
    changeset = restaking |> generate_changeset()
    Quorums.handle_quorum(%Quorums{id: restaking.quorum_number})

    case Restakings.get_by_quorum_and_operator_id(restaking.quorum_number, restaking.operator_id) do
      nil ->
        "inserting restaking" |> dbg
        Explorer.Repo.insert(changeset)

      [] ->
        "inserting restaking" |> dbg
        Explorer.Repo.insert(changeset)

      existing_restaking ->
        "updating restaking" |> dbg
        Explorer.Repo.update(Ecto.Changeset.change(existing_restaking, changeset.changes))

    end
  end

  def get_by_quorum_and_operator_id(quorum_number, operator_id) do
    query = from(
      r in Restakings,
      where: r.quorum_number == ^quorum_number
           and r.operator_id == ^operator_id,
      select: r
    )

    Explorer.Repo.one(query)
  end

end
