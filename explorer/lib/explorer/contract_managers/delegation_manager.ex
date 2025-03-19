defmodule DelegationManager do
  require Logger

  @environment System.get_env("ENVIRONMENT")

  @first_block (case @environment do
                  "devnet" -> 0
                  "holesky" -> 1_210_000
                  "mainnet" -> 19_000_000
                  _ -> raise("Invalid environment")
                end)

  eigenlayer_output_file_path =
    "../contracts/script/output/#{@environment}/eigenlayer_deployment_output.json"

  {status, config_json_string} = File.read(eigenlayer_output_file_path)

  case status do
    :ok ->
      Logger.debug("Eigenlayer deployment file read successfully")

    :error ->
      raise(
        "Config file not read successfully, did you run make explorer_create_env? If you did,\n make sure Eigenlayer config file is correctly stored"
      )
  end

  @delegation_manager_address Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("delegationManager")

  use Ethers.Contract,
    abi_file: "lib/abi/DelegationManager.json",
    default_address: @delegation_manager_address

  def get_delegation_manager_address() do
    @delegation_manager_address
  end

  def get_operator_url(operator_address) do
    DelegationManager.EventFilters.operator_metadata_uri_updated(operator_address)
      |> Ethers.get_logs(fromBlock: @first_block)
      |> case do
        {:ok, data} -> List.last(data).data |> hd() # most recent entry

        {:error, reason} ->
          Logger.error("Error getting operator url: #{inspect(reason)}")

        other ->
          Logger.debug("Unexpected response getting operator url: #{inspect(other)}")

      end
  end

  # This function makes use of Eigenlayer's Operator delegation structure and process (not AVS):
  # When Stakers want to stake to external Operators, they stake to an Operator address which was subscribed to Aligned's Slasher, not to any Operator that can be running any AVS.
  # This is how Eigenlayer maintains Staker's autonomy in choosing which AVS they want to restake to.
  # This way, even though we are querying Eigenlayer's DelegationManager, we are able to get the Operator's shares specifically in Aligned.
  def get_operator_all_strategies_shares(%Operators{id: operator_id, address: operator_address}) do
    all_strategies = Strategies.get_all_strategies_addresses()
    case DelegationManager.get_operator_shares(operator_address, all_strategies) |> Ethers.call do
      {:ok, shares} ->
        Enum.zip(all_strategies, shares)
          |> Enum.map(fn {strategy_address, share} ->
            %Restakings{operator_id: operator_id, operator_address: operator_address, stake: share, quorum_number: 0, strategy_address: strategy_address}
          end)
      {:error, error} ->
        Logger.error("Error getting operator shares: #{inspect(error)}")

        error
      other ->
        Logger.debug("Unexpected response getting operator shares: #{inspect(other)}")
        other
    end
  end


end
