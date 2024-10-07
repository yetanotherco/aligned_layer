defmodule Strategies do
  require Logger
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  schema "strategies" do
    field :strategy_address, :binary
    field :token_address, :binary
    field :name, :string
    field :symbol, :string
    field :total_staked, :decimal
    many_to_many :quorums, Quorums, join_through: "quorum_strategies"

    timestamps()
  end

  @doc false
  def changeset(strategy, attrs) do
    strategy
      |> cast(attrs, [:strategy_address, :token_address, :name, :symbol, :total_staked])
      |> validate_required([:strategy_address, :token_address, :name, :symbol])
      |> unique_constraint(:strategy_address)
      |> unique_constraint(:token_address)
  end

  def generate_changeset(%Strategies{} = strategy) do
    Strategies.changeset(%Strategies{}, Map.from_struct(strategy))
  end

  def update(restakeable_strategies) do
    restakeable_strategies
      |> Enum.reject(&Strategies.get_by_strategy_address/1)
      |> Enum.map(&extract_info/1)
      |> Enum.reject(&is_nil/1)
      |> Enum.each(&add_strategy/1)
  end

  def get_by_strategy_address(strategy_address) do
    query = from(s in Strategies,
      where: s.strategy_address == ^strategy_address,
      select: s)
    Explorer.Repo.one(query)
  end

  def get_total_staked(strategy_address) do
    query = from(s in Strategies,
      where: s.strategy_address == ^strategy_address,
      select: s.total_staked)
    Explorer.Repo.one(query)
  end

  def extract_info(strategy_address) do
    %Strategies{strategy_address: strategy_address}
    |> StrategyInterfaceManager.fetch_token_address()
    |> StrategyInterfaceManager.fetch_token_name()
    |> StrategyInterfaceManager.fetch_token_symbol()
      # Total stake is set when inserting values to `Restakings` table
  end

  def add_strategy(%Strategies{} = new_strategy) do
    "Adding strategy" |> Logger.debug()
    Strategies.generate_changeset(new_strategy) |> Explorer.Repo.insert()
  end
  def add_strategy({:error, _error}) do
    :nil
  end

  def get_all_strategies() do
    query = from(s in Strategies,
      select: s,
      order_by: [desc: s.total_staked])
    Explorer.Repo.all(query)
  end

  def get_all_strategies_addresses() do
    query = from(s in Strategies,
      order_by: [asc: s.id],
      select: s.strategy_address)
    Explorer.Repo.all(query)
  end

  def generate_update_total_staked_changeset(%{new_restaking: new_restaking}) do
    query = from(s in Strategies,
      where: s.strategy_address == ^new_restaking.strategy_address,
      select: s)
    strategy = Explorer.Repo.one(query)

    query = from(r in Restakings,
      where: r.strategy_address == ^new_restaking.strategy_address and r.operator_address == ^new_restaking.operator_address,
      select: r)
    old_restaking = case Explorer.Repo.one(query) do
      nil -> %Restakings{stake: 0}
      restaking -> restaking
    end

    restaking_amount_diff = Decimal.sub(new_restaking.stake, old_restaking.stake)
    new_stake = Decimal.add(strategy.total_staked, (restaking_amount_diff))
    Strategies.changeset(strategy, %{total_staked: new_stake})
  end

  def discount_restaking(restaking) do
    query = from(s in Strategies,
      where: s.strategy_address == ^restaking.strategy_address,
      select: s)
    strategy = Explorer.Repo.one(query)

    new_stake =
      strategy.total_staked
      |> Decimal.sub(restaking.stake)
      |> (fn stake -> if Decimal.compare(stake, 0) == :lt, do: Decimal.new(0), else: stake end).()
      
    Strategies.changeset(strategy, %{total_staked: new_stake}) |> Explorer.Repo.update()

  end
end
