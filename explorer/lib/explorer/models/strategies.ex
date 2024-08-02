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
    |> fetch_token_address()
    |> fetch_token_name()
    |> fetch_token_symbol()
    |> tap(&dbg/1)
    # Total stake is set when inserting rows to `Restakings` table
  end

  defp fetch_token_address(%Strategies{strategy_address: strategy_address} = strategy) do
    case Strategies.underlying_token() |> Ethers.call(to: strategy_address) do
      {:ok, "0x"} ->
        dbg("Strategy has invalid underlying token: #{strategy_address}, token_address: '0x'")
        {:error, :invalid_token_address}
      {:ok, token_address} -> %{strategy | token_address: token_address}

      {:error, %{"code" => -32015}} ->
        dbg("Strategy has no underlying token: #{strategy_address}") # thus, its not a strategy contract
        {:error, :not_strategy}

        other_error ->
        dbg("Error fetching token address for #{strategy_address}")
        dbg(other_error)
        other_error
    end
  end

  defp fetch_token_name(%Strategies{token_address: token_address} = strategy) do
    case ERC20.name(token_address) do
      {:ok, name} -> %{strategy | name: name}
      error ->
        dbg("Error fetching token name")
        dbg(error)
        error
    end
  end
  defp fetch_token_name({:error, error}) do
    {:error, error}
  end

  defp fetch_token_symbol(%Strategies{token_address: token_address} = strategy) do
    case ERC20.symbol(token_address) do
      {:ok, symbol} -> %{strategy | symbol: symbol}
      error ->
        dbg("Error fetching token symbol")
        dbg(error)
        error
    end
  end
  defp fetch_token_symbol({:error, error}) do
    {:error, error}
  end

  def add_strategy(%Strategies{} = new_strategy) do
    dbg("adding strategy")
    dbg(new_strategy)
    Strategies.generate_changeset(new_strategy) |> Explorer.Repo.insert()
  end
  def add_strategy({:error, error}) do
    :nil
  end

end
