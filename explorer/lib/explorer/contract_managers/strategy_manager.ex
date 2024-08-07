defmodule StrategyManager do
  require Logger

  use Ethers.Contract,
  abi_file: "lib/abi/IStrategy.json"

  def fetch_token_address(%Strategies{strategy_address: strategy_address} = strategy) do
    case StrategyManager.underlying_token() |> Ethers.call(to: strategy_address) do
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

  def fetch_token_name(%Strategies{token_address: token_address} = strategy) do
    case ERC20Manager.name(token_address) do
      {:ok, name} -> %{strategy | name: name}
      error ->
        dbg("Error fetching token name")
        dbg(error)
        error
    end
  end
  def fetch_token_name({:error, error}) do
    {:error, error}
  end

  def fetch_token_symbol(%Strategies{token_address: token_address} = strategy) do
    case ERC20Manager.symbol(token_address) do
      {:ok, symbol} -> %{strategy | symbol: symbol}
      error ->
        dbg("Error fetching token symbol")
        dbg(error)
        error
    end
  end
  def fetch_token_symbol({:error, error}) do
    {:error, error}
  end
end
