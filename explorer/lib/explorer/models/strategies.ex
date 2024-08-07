defmodule Strategies do
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
      |> Enum.map(&add_strategy/1)
  end

  def get_by_strategy_address(strategy_address) do
    query = from(s in Strategies,
      where: s.strategy_address == ^strategy_address,
      select: s)
    Explorer.Repo.one(query)
  end

  def extract_info(strategy_address) do
    %Strategies{strategy_address: strategy_address}
    |> StrategyManager.fetch_token_address()
    |> StrategyManager.fetch_token_name()
    |> StrategyManager.fetch_token_symbol()
    # Total stake is set when inserting rows to `Restakings` table
  end

  def add_strategy(%Strategies{} = new_strategy) do
    dbg("adding strategy")
    Strategies.generate_changeset(new_strategy) |> Explorer.Repo.insert()
  end
  def add_strategy({:error, _error}) do
    :nil
  end

end
