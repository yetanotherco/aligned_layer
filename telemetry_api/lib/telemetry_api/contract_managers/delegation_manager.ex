defmodule TelemetryApi.ContractManagers.DelegationManager do
  alias TelemetryApi.ContractManagers.DelegationManager
  require Logger

  @environment System.get_env("ENVIRONMENT")

  @first_block (case @environment do
                  "devnet" -> 0
                  "holesky" -> 1_210_000
                  "mainnet" -> 20_020_000
                  _ -> raise("Invalid environment")
                end)

  eigenlayer_output_file_path =
    "../contracts/script/output/#{@environment}/eigenlayer_deployment_output.json"

  {status, config_json_string} = File.read(eigenlayer_output_file_path)

  case status do
    :ok ->
      Logger.debug("Eigenlayer deployment file read successfully")

    :error ->
      raise("Config file not read successfully")
  end

  @contract_address Jason.decode!(config_json_string)
                    |> Map.get("addresses")
                    |> Map.get("delegationManager")

  use Ethers.Contract,
    abi_file: "priv/abi/DelegationManager.json",
    default_address: @contract_address

  def get_contract_address() do
    @contract_address
  end

  def get_operator_url(operator_address) do
    DelegationManager.EventFilters.operator_metadata_uri_updated(operator_address)
    |> Ethers.get_logs(fromBlock: @first_block)
    |> case do
      {:ok, data} ->
        # The head (hd) is the most recent entry
        url = List.last(data).data |> hd()
        {:ok, url}

      {:error, reason} ->
        {:error, reason}

      other ->
        {:error, "Unexpected response getting operator url: #{inspect(other)}"}
    end
  end
end
