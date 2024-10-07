defmodule StrategyInterfaceManager do
  require Logger

  use Ethers.Contract,
  abi_file: "lib/abi/IStrategy.json"

  def fetch_token_address(%Strategies{strategy_address: strategy_address} = strategy) do
    case StrategyInterfaceManager.underlying_token() |> Ethers.call(to: strategy_address) do
      {:ok, "0x"} -> # Strategy is native ETH
        %{strategy | token_address: "0x"} #storing "0x" as its token address, and handling its cases in ERC20InterfaceManager

      {:ok, token_address} -> %{strategy | token_address: token_address}

      {:error, %{"code" => -32015}} ->
        "Strategy has no underlying token: #{strategy_address}" |> Logger.debug() # thus, its not a strategy contract
        {:error, :not_strategy}

        other_error ->
          "Error fetching token address for #{strategy_address}: #{inspect(other_error)}" |> Logger.error()
          other_error
    end
  end

  def fetch_token_name(%Strategies{token_address: token_address} = strategy) do
    case ERC20InterfaceManager.name(token_address) do
      {:ok, name} -> %{strategy | name: name}
      error ->
        case error do
          {:error, %{"code" => 3, "data" => "0x", "message" => "execution reverted"}} -> %{strategy | name: "‎"} # token has no Name (empty char), not a common practice but still an ERC20
          _ ->
            "Error fetching token name for #{token_address}: #{inspect(error)}" |> Logger.error()
            error
        end
    end
  end
  def fetch_token_name({:error, error}) do
    {:error, error}
  end

  def fetch_token_symbol(%Strategies{token_address: token_address} = strategy) do
    case ERC20InterfaceManager.symbol(token_address) do
      {:ok, symbol} -> %{strategy | symbol: symbol}
      error ->
        case error do
          {:error, %{"code" => 3, "data" => "0x", "message" => "execution reverted"}} -> %{strategy | symbol: "‎"} # token has no Symbol (empty char), not a common practice but still an ERC20
          _ ->
            "Error fetching token symbol for #{token_address}: #{inspect(error)}" |> Logger.error()
            error
        end
    end
  end
  def fetch_token_symbol({:error, error}) do
    {:error, error}
  end
end
