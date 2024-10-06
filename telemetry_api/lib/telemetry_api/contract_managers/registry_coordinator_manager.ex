defmodule TelemetryApi.ContractManagers.RegistryCoordinatorManager do
  alias TelemetryApi.ContractManagers.RegistryCoordinatorManager

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

  @registry_coordinator_address Jason.decode!(config_json_string)
                                |> Map.get("addresses")
                                |> Map.get("registryCoordinator")

  use Ethers.Contract,
    abi_file: "priv/abi/IRegistryCoordinator.json",
    default_address: @registry_coordinator_address

  def get_registry_coordinator_address() do
    @registry_coordinator_address
  end

  def fetch_operator_status(operator_address) do
    RegistryCoordinatorManager.get_operator_status(operator_address)
    |> Ethers.call()
  end
end
