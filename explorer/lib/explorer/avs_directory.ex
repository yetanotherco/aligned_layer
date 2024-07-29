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

  @first_block (case @environment do
                  "devnet" -> 0
                  "holesky" -> 1_600_000
                  "mainnet" -> 20_020_000
                  _ -> raise("Invalid environment")
                end)

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

  def get_operator_status_updated_events(%{fromBlock: fromBlock}) do
    AVSDirectory.EventFilters.operator_avs_registration_status_updated(
      nil,
      AlignedLayerServiceManager.get_aligned_layer_service_manager_address()
    )
    |> Ethers.get_logs(fromBlock: fromBlock)
  end

  def process_operator_data(%{fromBlock: fromBlock}) do
    AVSDirectory.get_operator_status_updated_events(%{fromBlock: fromBlock})
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
    IO.inspect(event)
    case event.topics |> hd do
      "OperatorAVSRegistrationStatusUpdated(address,address,uint8)" ->
        case event.data |> hd do
          1 ->
            IO.inspect("Operator registered")
            #TODO where to get operator name? from URI
            AVSDirectory.handle_operator_registration(event)

          0 ->
            IO.inspect("Operator unregistered")
            Operators.unregister_operator(%Operators{address: Enum.at(event.topics, 1)})

          other ->
            IO.inspect("Unexpected event data", event.data)
        end
      _ ->
        IO.inspect("Unexpected event")
        nil
    end
  end

  def handle_operator_registration(event) do
    # operator_name = AVSDirectory.get_operator_name(Enum.at(event.topics, 1))
    # URI = read latest 'OperatorMetadataURIUpdated(msg.sender, metadataURI)' event from DelegationManager.sol
    # operator_name get from inside URI resource
    operator_uri = DelegationManager.get_operator_uri(Enum.at(event.topics, 1)) #is not being called for each operator, only for our operator?
    operator_name = "wip" # TODO parse previous URI to get relevant info
    Operators.register_operator(%Operators{name: operator_name, address: Enum.at(event.topics, 1), URI: operator_uri})
  end
end
