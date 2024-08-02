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
      Logger.debug("Eigenlayer deployment file read successfully")

    :error ->
      raise(
        "Config file not read successfully, did you run make create-env? If you did,\n make sure Eigenlayer config file is correctly stored"
      )
  end

  @stake_registry_manager Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("stakeRegistry")

  use Ethers.Contract,
    abi_file: "lib/abi/IStakeRegistry.json",
    default_address: @stake_registry_manager

  def get_stake_registry_manager() do
    @stake_registry_manager
  end

end
