defmodule AVSDirectory do
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
        "Config file not read successfully, did you run make create-env? If you did,\n make sure Eigenlayer config file is correctly stored"
      )
  end

  @avs_directory_address Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("avsDirectory")

  # @first_block (case @environment do
  #                 "devnet" -> 0
  #                 "holesky" -> 1_600_000
  #                 "mainnet" -> 20_020_000
  #                 _ -> raise("Invalid environment")
  #               end)

  use Ethers.Contract,
    abi_file: "lib/abi/AVSDirectory.json",
    default_address: @avs_directory_address

  def get_avs_directory_address() do
    @avs_directory_address
  end

  # def get_operator_status_updated_events() do
  #   AVSDirectory.EventFilters.operator_avs_registration_status_updated(
  #     nil,
  #     AlignedLayerServiceManager.get_aligned_layer_service_manager_address()
  #   )
  #   |> Ethers.get_logs(fromBlock: @first_block)
  # end

  # tail-call recursion
  # defp count_operators_registered(list), do: sum_operators_registered(list, 0)
  # defp sum_operators_registered([], val), do: val

  # defp sum_operators_registered([head | tail], val),
  #   do: sum_operators_registered(tail, evaluate_operator(head, val))

  # defp evaluate_operator(event, val) do
  #   # registered or unregistered
  #   case event.data |> hd() == 1 do
  #     true -> val + 1
  #     false -> val - 1
  #   end
  # end

  def get_operator_registration_status_updated_events(%{fromBlock: fromBlock}) do
    AVSDirectory.EventFilters.operator_avs_registration_status_updated(
      nil, # any operator
      AlignedLayerServiceManager.get_aligned_layer_service_manager_address() # our AVS
    ) |> Ethers.get_logs(fromBlock: fromBlock)
  end

  def process_operator_data(%{fromBlock: fromBlock}) do
    AVSDirectory.get_operator_registration_status_updated_events(%{fromBlock: fromBlock})
      |> case do
        {:ok, events} ->
          Enum.map(events, &extract_operator_event_info/1)

        {:error, reason} ->
          IO.inspect("Error fetching operator events")
          IO.inspect(reason)
          []
        _ ->
          IO.inspect("Unexpected response fetching operator events")
          []
      end
  end

  def extract_operator_event_info(event) do
    case Mutex.lock(OperatorMutex, {event.topics |> Enum.at(1)}) do
      {:error, :busy} ->
        "Operator already being processed: #{event.topics |> Enum.at(1)}" |> IO.inspect()
        :empty

      {:ok, lock} ->
        case event.topics |> hd do
          "OperatorAVSRegistrationStatusUpdated(address,address,uint8)" ->
            case event.data |> hd do
              1 ->
                IO.inspect("Operator registered")
                AVSDirectory.handle_operator_registration(event)

              0 ->
                IO.inspect("Operator unregistered")
                AVSDirectory.handle_operator_unregistration(event)

              _other ->
                IO.inspect("Unexpected event data", event.data)
            end
          _ ->
            IO.inspect("Unexpected event")
            :empty
        end
        Mutex.release(OperatorMutex, lock)
    end
  end

  def handle_operator_registration(event) do
    IO.inspect("Handling operator registration")
    operator_url = DelegationManager.get_operator_url(Enum.at(event.topics, 1))
    operator_metadata = case Utils.fetch_eigen_operator_metadata(operator_url) do
      {:ok, operator_metadata} ->
        operator_metadata

      {:error, reason} ->
        IO.inspect("Error fetching operator metadata:")
        IO.inspect(reason)
        %EigenOperatorMetadataStruct{name: nil, website: nil, description: nil, logo: nil, twitter: nil}
    end
    Operators.register_operator(%Operators{name: operator_metadata.name, address: Enum.at(event.topics, 1), url: operator_url, website: operator_metadata.website, description: operator_metadata.description, logo_link: operator_metadata.logo, twitter: operator_metadata.twitter})
    # TODO read its first stake
  end

  def handle_operator_unregistration(_event) do
    IO.inspect("TODO: Handle operator unregistration")
    # Operators.unregister_operator(%Operators{address: Enum.at(event.topics, 1)})
  end
end
