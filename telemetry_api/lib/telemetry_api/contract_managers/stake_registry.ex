defmodule TelemetryApi.ContractManagers.StakeRegistry do
  alias TelemetryApi.ContractManagers.StakeRegistry

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
      raise("Config file not read successfully")
  end

  @stake_registry_address Jason.decode!(config_json_string)
                        |> Map.get("addresses")
                        |> Map.get("stakeRegistry")

  use Ethers.Contract,
    abi_file: "priv/abi/IStakeRegistry.json",
    default_address: @stake_registry_address

    @doc """
    Get the current total stake
    We only use quorum 0

    ## Examples

        iex> get_current_total_stake()
        {:ok, 100}

        iex> get_current_total_stake()
        {:error, "Error message"}
    """
    def get_current_total_stake() do
      StakeRegistry.get_current_total_stake(0)
        |> Ethers.call()
    end
end
