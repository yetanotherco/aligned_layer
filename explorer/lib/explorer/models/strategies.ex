defmodule Strategies do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  use Ethers.Contract,
  abi_file: "lib/abi/IStrategy.json"

  schema "strategies" do
    field :strategy_address, :binary
    field :token_address, :binary
    field :name, :string
    field :symbol, :string
    field :total_staked, :decimal

    timestamps()
  end

  @doc false
  def changeset(strategy, attrs) do
    strategy
    |> cast(attrs, [:name, :symbol, :address, :total_staked])
    |> validate_required([:name, :symbol, :address, :total_staked])
    |> unique_constraint(:address)
  end

  def generate_changeset(%Strategies{} = strategy) do
    Strategies.changeset(%Strategies{}, Map.from_struct(strategy))
  end

  def update(restakeable_strategies) do
    restakeable_strategies
      |> Enum.map(&remove_redundant/1)
      |> Enum.map(&extract_info/1)
      |> Enum.map(&add_strategy/1)
  end

  def remove_redundant(strategy_address) do
    case Strategies.get_by_strategy_address(strategy_address) do
      :nil ->
        strategy_address
      _ -> :nil
    end
  end

  def get_by_strategy_address(strategy_address) do
    query = from(s in Strategies,
      where: s.strategy_address == ^strategy_address,
      select: s)
    Explorer.Repo.one(query)
  end

  def extract_info(strategy_address) do
    current_strategy = %Strategies{strategy_address: strategy_address}

    current_strategy = case Strategies.underlying_token() |> Ethers.call(to: current_strategy.strategy_address) do
      {:ok, token_address} ->
        %{current_strategy | token_address: token_address}
      error ->
        dbg("Error fetching token address")
        dbg(error)
    end

    current_strategy |> dbg
    current_strategy = case ERC20.name(current_strategy.token_address) do
      {:ok, name} ->
        %{current_strategy | name: name}
      error ->
        dbg("Error fetching token name")
        dbg(error)
    end

    current_strategy = case ERC20.symbol(current_strategy.token_address) do
      {:ok, symbol} ->
        %{current_strategy | symbol: symbol}

      error ->
        dbg("Error fetching token symbol")
        dbg(error)
    end

    dbg(current_strategy)

    # WIP handle errors

    # Total stake is set when inserting rows to `Restakings` table
  end

  def add_strategy(new_strategy) do
    Strategies.generate_changeset(new_strategy) |> Explorer.Repo.insert()
  end

end
