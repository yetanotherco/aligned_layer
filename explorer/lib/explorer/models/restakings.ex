defmodule Restakings do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  schema "restakings" do
    field :operator_id, :binary
    field :operator_address, :binary
    field :stake, :decimal
    field :quorum_number, :integer
    field :strategy_address, :binary

    timestamps()
  end

  @doc false
  def changeset(restaking, attrs) do
    restaking
    |> cast(attrs, [:operator_id, :operator_address, :stake, :quorum_number, :strategy_address])
    |> validate_required([:operator_id, :stake, :quorum_number, :strategy_address])
  end

  def generate_changeset(%Restakings{} = restaking) do
    Restakings.changeset(%Restakings{}, Map.from_struct(restaking))
  end

  def process_restaking_changes(%{fromBlock: from_block}) do
    Operators.get_operators()
      |> Enum.map(fn operator -> StakeRegistryManager.has_operator_changed_staking(%{fromBlock: from_block, operator_id: operator.id, operator_address: operator.address}) end)
      |> Enum.reject(fn {_operator_id, _operator_address, has_changed_stake} -> not has_changed_stake end)
      |> Enum.map(fn {operator_id, operator_address, _has_changed_stake} -> DelegationManager.get_operator_all_strategies_shares(%Operators{id: operator_id, address: operator_address}) end)
      |> Enum.map(&insert_or_update_restakings/1)
  end

  def insert_or_update_restakings(%Restakings{} = restaking) do
    changeset = restaking |> generate_changeset()

    # Temporal solution to handle new quorums, until Eigenlayer implements emition of QuorumCreated event
    Quorums.handle_quorum(%Quorums{id: restaking.quorum_number})

    case Restakings.get_by_operator_and_strategy(%Restakings{operator_address: restaking.operator_address, strategy_address: restaking.strategy_address}) do
      nil ->
        "nil, inserting restaking" |> dbg
        Explorer.Repo.insert(changeset)

      [] ->
        "[], inserting restaking" |> dbg
        Explorer.Repo.insert(changeset)

      existing_restaking ->
        "updating restaking" |> dbg
        Explorer.Repo.update(Ecto.Changeset.change(existing_restaking, changeset.changes))

    end
  end

   def get_by_operator_and_strategy(%Restakings{operator_address: operator_address, strategy_address: strategy_address}) do
    query = from(
      r in Restakings,
      where: r.operator_address == ^operator_address and r.strategy_address == ^strategy_address,
      select: r
    )
    Explorer.Repo.all(query)
  end

  def get_aggregated_restakings() do
    query = from(
      r in Restakings,
      select: %{total_stake: sum(r.stake)}
    )
    Explorer.Repo.one(query)
  end

  def get_restakes_by_operator(operator_id) do
    query = from(
      r in Restakings,
      where: r.operator_id == ^operator_id,
      select: r
    )
    Explorer.Repo.all(query)
  end

end
