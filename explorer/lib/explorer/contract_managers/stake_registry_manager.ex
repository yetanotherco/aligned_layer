defmodule StakeRegistryManager do
  require Logger

  @aligned_config_file System.get_env("ALIGNED_CONFIG_FILE")

  config_file_path =
    case @aligned_config_file do
      nil -> raise("ALIGNED_CONFIG_FILE not set in .env")
      file -> file
    end

  {status, config_json_string} = File.read(config_file_path)

  case status do
    :ok ->
      Logger.debug("Aligned deployment file read successfully")

    :error ->
      raise(
        "Config file not read successfully, did you run make explorer_create_env? If you did,\n make sure Eigenlayer config file is correctly stored"
      )
  end

  @stake_registry_manager Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("stakeRegistry")

  use Ethers.Contract,
    abi_file: "lib/abi/StakeRegistry.json",
    default_address: @stake_registry_manager

  def get_stake_registry_manager() do
    @stake_registry_manager
  end

  def has_operator_changed_staking(%{fromBlock: fromBlock, operator_id: operator_id, operator_address: operator_address}) do
    StakeRegistryManager.EventFilters.operator_stake_update(operator_id)
      |> Ethers.get_logs(fromBlock: fromBlock)
      |> case do
        {:ok, data} ->
          {operator_id, operator_address, Enum.count(data) > 0}

        {:error, reason} ->
          "Error getting latest operator stake update: #{inspect(reason)}" |> Logger.error()

        other ->
          "Unexpected response: #{inspect(other)}" |> Logger.debug()
      end
  end

  def get_strategies_of_quorum(quorum_number) do
    amount_of_strategies = case StakeRegistryManager.strategy_params_length(quorum_number) |> Ethers.call() do
      {:ok, amount} ->
        amount
      {:error, error} ->
        "Error fetching amount of strategies: #{error}" |> Logger.error()
        raise("Error fetching amount of strategies: #{error}")
    end

    strategies = Enum.reduce(0..(amount_of_strategies - 1), [], fn index, acc ->
      case StakeRegistryManager.strategies_per_quorum(quorum_number, index) |> Ethers.call() do
        {:ok, strategy_address} ->
          [strategy_address | acc]
        {:error, error} ->
          "Error fetching strategy at index #{index}: #{error}" |> Logger.error()
          acc
      end
    end)

    strategies |> Enum.reverse()
  end

  def get_stake_of_quorum_for_operator(%Restakings{operator_address: operator_address, quorum_number: nil}) do # AT THE MOMENT, ONLY USING QUORUM 0
    get_stake_of_quorum_for_operator(%Restakings{operator_address: operator_address, quorum_number: 0})
  end

  def get_stake_of_quorum_for_operator(%Restakings{operator_address: operator_address, quorum_number: quorum_number}) do
    case StakeRegistryManager.weight_of_operator_for_quorum(quorum_number, operator_address) |> Ethers.call() do
      {:ok, stake_of_operator} ->
        stake_of_operator
      {:error, error} ->
        "Error fetching stake of operator: #{error}" |> Logger.error()
        raise("Error fetching stake of operator: #{error}")
    end
  end

end
