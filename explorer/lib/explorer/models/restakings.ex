defmodule Restakings do
  require Logger
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
      |> Enum.reject(fn {operator_id, _operator_address, _has_changed_stake} -> not Operators.get_operator_by_id(operator_id).is_active end)
      |> Enum.map(fn {operator_id, operator_address, _has_changed_stake} -> DelegationManager.get_operator_all_strategies_shares(%Operators{id: operator_id, address: operator_address}) end)
      |> Enum.each(&insert_or_update_restakings/1)
  end

  def insert_or_update_restakings(restakings) when is_list(restakings) do
    Enum.each(restakings, &insert_or_update_restakings/1)
  end
  def insert_or_update_restakings(%Restakings{} = restaking) do
    changeset = restaking |> generate_changeset()

    multi =
      case Restakings.get_by_operator_and_strategy(%Restakings{operator_address: restaking.operator_address, strategy_address: restaking.strategy_address}) do
      nil ->
        "Inserting restaking" |> Logger.debug()
        Ecto.Multi.new()
          |> Ecto.Multi.insert(:insert_restaking, changeset)
          |> Ecto.Multi.update(:update_strategy_total_staked, Strategies.generate_update_total_staked_changeset(%{new_restaking: restaking}))

      existing_restaking ->
        "Updating restaking" |> Logger.debug()
        Ecto.Multi.new()
          |> Ecto.Multi.update(:update_restaking, Ecto.Changeset.change(existing_restaking, changeset.changes))
          |> Ecto.Multi.update(:update_strategy_total_staked, Strategies.generate_update_total_staked_changeset(%{new_restaking: restaking}))
      end

    multi = multi
      |> Ecto.Multi.update(:update_operator_total_stake, Operators.generate_new_total_stake_changeset(%{operator_address: restaking.operator_address}))

    case Explorer.Repo.transaction(multi) do
      {:ok, _} ->
        "Restaking inserted or updated" |> Logger.debug()
        {:ok, :empty}
      {:error, _, changeset, _} ->
        "Error updating restakings table: #{inspect(changeset.errors)}" |> Logger.error()
        {:error, changeset}
    end
  end

  def remove_restakes_of_operator(%{operator_address: operator_address}) do
    Logger.debug("Removing restakes of operator")
    query = from(r in Restakings, where: r.operator_address == ^operator_address)
    restakings = Explorer.Repo.all(query)

    Explorer.Repo.delete_all(query)
    Enum.each(restakings, &Strategies.discount_restaking/1)
  end

  def get_by_operator_and_strategy(%Restakings{operator_address: operator_address, strategy_address: strategy_address}) do
    query = from(
      r in Restakings,
      where: r.operator_address == ^operator_address and r.strategy_address == ^strategy_address,
      select: r
    )
    Explorer.Repo.one(query)
  end

  def get_aggregated_restakings() do
    query = from(
      r in Restakings,
      select: %{total_stake: sum(r.stake)}
    )
    Explorer.Repo.one(query)
  end

  def get_restakes_by_operator_id(operator_id) do
    query = from r in Restakings,
      join: s in Strategies, on: r.strategy_address == s.strategy_address,
      where: r.operator_id == ^operator_id,
      order_by: [desc: r.stake],
      select: %{
        restaking: r,
        strategy: %{
          name: s.name,
          symbol: s.symbol,
          token_address: s.token_address,
          total_staked: s.total_staked
        }
      }

    Explorer.Repo.all(query)
  end

  def get_restaked_amount_eth() do
    restaked_amount_wei =
      Restakings.get_aggregated_restakings()
      |> Map.get(:total_stake)

    case restaked_amount_wei do
      nil ->
        nil

      _ ->
        restaked_amount_wei
        |> EthConverter.wei_to_eth(2)
    end
  end
  
  def get_restaked_amount_usd() do
    restaked_amount_wei =
      Restakings.get_aggregated_restakings()
      |> Map.get(:total_stake)

    case restaked_amount_wei do
      nil ->
        nil

      _ ->
        case EthConverter.wei_to_usd(restaked_amount_wei, 0) do
          {:ok, usd_value} -> usd_value
          _ -> nil
        end
    end
  end
end
