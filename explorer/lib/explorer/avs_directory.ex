defmodule AVSDirectory do
  require Logger

  file_path =
    "../contracts/script/output/#{System.get_env("ENVIRONMENT")}/eigenlayer_deployment_output.json"

  {status, config_json_string} = File.read(file_path)

  case status do
    :ok -> Logger.debug("File read successfully")
    :error -> raise("Config file not read successfully, did you run make create-env ?")
  end

  @avs_directory_address Jason.decode!(config_json_string) |> Map.get("addresses") |> Map.get("avsDirectory")

  use Ethers.Contract,
    abi_file: "lib/abi/AVSDirectory.json",
    default_address:
      @avs_directory_address

  def get_avs_directory_address() do
    @avs_directory_address
  end

  def get_operator_status_updated_events() do
    # event OperatorAVSRegistrationStatusUpdated(address indexed operator, address indexed avs, OperatorAVSRegistrationStatus status);
    AVSDirectory.EventFilters.operator_avs_registration_status_updated(
      nil,
      AlignedLayerServiceManager.get_aligned_layer_service_manager_address()
    )
    |> Ethers.get_logs(fromBlock: 0)
  end
end
