defmodule AVSDirectory do
  require Logger

  @environment System.get_env("ENVIRONMENT")

  file_path =
    "../contracts/script/output/#{@environment}/eigenlayer_deployment_output.json"

  {status, config_json_string} = File.read(file_path)

  case status do
    :ok -> Logger.debug("File read successfully")
    :error -> raise("Config file not read successfully, did you run make create-env? If you did,\n make sure Eigenlayer config file is correctly stored")
  end

  @avs_directory_address Jason.decode!(config_json_string) |> Map.get("addresses") |> Map.get("avsDirectory")

  @first_block (
    case @environment do
      "devnet" -> 0
      "holesky" -> 1600000
      "mainnet" -> 20020000
      _ -> raise("Invalid environment")
    end
  )

  use Ethers.Contract,
    abi_file: "lib/abi/AVSDirectory.json",
    default_address:
      @avs_directory_address

  def get_avs_directory_address() do
    @avs_directory_address
  end

  def get_operator_status_updated_events() do
    AVSDirectory.EventFilters.operator_avs_registration_status_updated(
      nil,
      AlignedLayerServiceManager.get_aligned_layer_service_manager_address()
    )
    |> Ethers.get_logs(fromBlock: @first_block)
  end
end
