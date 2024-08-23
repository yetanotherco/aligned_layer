defmodule BatcherPaymentServiceManager do
  require Logger

  @aligned_config_file System.get_env("ALIGNED_CONFIG_FILE")

  @environment System.get_env("ENVIRONMENT")
  @first_block (case @environment do
                  "devnet" -> 0
                  "holesky" -> 1_728_056
                  "mainnet" -> 20_020_000
                  _ -> raise("Invalid environment")
                end)

  config_file_path =
    case @aligned_config_file do
      nil -> raise("ALIGNED_CONFIG_FILE not set in .env")
      file -> file
    end

  {status_aligned_config, config_json_string} = File.read(config_file_path)

  @batcher_payment_service_address Jason.decode!(config_json_string)
    |> Map.get("addresses")
    |> Map.get("batcherPaymentService")

  use Ethers.Contract,
    abi_file: "lib/abi/BatcherPaymentService.json",
    default_address: @batcher_payment_service_address

  def get_batcher_payment_service_address() do
    @batcher_payment_service_address
  end

  def get_gas_per_proof(merkle_root) do
    BatcherPaymentServiceManager.EventFilters.new_task_created(
      merkle_root
      |> Utils.string_to_bytes32()
    )
    |> Ethers.get_logs(fromBlock: @first_block)
    |> case do
      {:ok, events} ->
        List.last(events).data |> hd()

      {:error, reason} ->
        Logger.error("Error getting gas per proof: #{inspect(reason)}")
        raise("Error getting gas per proof")

      other ->
        Logger.error("Unexpected response: #{inspect(other)}")
        raise("Unexpected response")
    end
  end
end
