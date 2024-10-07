defmodule AVSDirectoryManager do
  require Logger

  @environment System.get_env("ENVIRONMENT")

  file_path =
    "../contracts/script/output/#{@environment}/eigenlayer_deployment_output.json"

  {status, config_json_string} = File.read(file_path)

  case status do
    :ok ->
      Logger.debug("Eigenlayer deployment file read successfully")

    :error ->
      raise(
        "Config file not read successfully, did you run make explorer_create_env? If you did,\n make sure Eigenlayer config file is correctly stored"
      )
  end

  @avs_directory_address Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("avsDirectory")

  use Ethers.Contract,
    abi_file: "lib/abi/AVSDirectory.json",
    default_address: @avs_directory_address

  def get_avs_directory_address() do
    @avs_directory_address
  end

  def process_and_store_operator_data(%{fromBlock: fromBlock}) do
    AVSDirectoryManager.get_operator_registration_status_updated_events(%{fromBlock: fromBlock})
      |> case do
        {:ok, events} ->
          Enum.map(events, &extract_operator_event_info/1)

        {:error, reason} ->
          Logger.error("Error fetching operator events: #{inspect(reason)}")
          []
        _ ->
          Logger.debug("Unexpected response fetching operator events")
          []
      end
  end

  def get_operator_registration_status_updated_events(%{fromBlock: fromBlock}) do
    AVSDirectoryManager.EventFilters.operator_avs_registration_status_updated(
      nil, # any operator
      AlignedLayerServiceManager.get_aligned_layer_service_manager_address() # our AVS
    ) |> Ethers.get_logs(fromBlock: fromBlock)
  end

  def extract_operator_event_info(event) do
    case Mutex.lock(OperatorMutex, {event.topics |> Enum.at(1)}) do
      {:error, :busy} ->
        "Operator already being processed: #{event.topics |> Enum.at(1)}" |> Logger.error()
        :empty

      {:ok, lock} ->
        case event.topics |> hd do
          "OperatorAVSRegistrationStatusUpdated(address,address,uint8)" ->
            case event.data |> hd do
              1 ->
                Logger.debug("Operator registered")
                Operators.handle_operator_registration(event)

              0 ->
                Logger.debug("Operator unregistered")
                Operators.handle_operator_unregistration(event)

              _other ->
                Logger.error("Unexpected event data: #{inspect(event.data)}")
            end
          _ ->
            Logger.error("Unexpected event")
            :empty
        end
        Mutex.release(OperatorMutex, lock)
    end
  end
end
