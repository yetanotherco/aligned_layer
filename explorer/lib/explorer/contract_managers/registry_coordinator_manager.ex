defmodule RegistryCoordinatorManager do
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

  @registry_coordinator_address Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("registryCoordinator")

  use Ethers.Contract,
    abi_file: "lib/abi/IRegistryCoordinator.json",
    default_address: @registry_coordinator_address

  def get_registry_coordinator_address() do
    @registry_coordinator_address
  end

  def get_operator_id_from_chain(operator_address) do
    case RegistryCoordinatorManager.get_operator_id(Utils.string_to_bytes32(operator_address))
      |> Ethers.call() do
        {:ok, data} ->
          data
        error ->
          {:error, error}
      end
  end

end
