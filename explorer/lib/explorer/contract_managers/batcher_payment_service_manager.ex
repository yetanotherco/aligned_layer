defmodule BatcherPaymentServiceManager do
  require Logger

  @aligned_config_file System.get_env("ALIGNED_CONFIG_FILE")

  @environment System.get_env("ENVIRONMENT")
  @first_block (case @environment do
                  "devnet" -> 0
                  "holesky" -> 1_728_056
                  "mainnet" -> 19_000_000
                  _ -> raise("Invalid environment")
                end)

  config_file_path =
    case @aligned_config_file do
      nil -> raise("ALIGNED_CONFIG_FILE not set in .env")
      file -> file
    end

  {status_aligned_config, config_json_string} = File.read(config_file_path)

  case status_aligned_config do
    :ok ->
      Logger.debug("Aligned config file read successfully")

    :error ->
      raise(
        "Config file not read successfully, did you run make explorer_create_env? If you did,\n make sure AlignedLayer config file is correctly stored"
      )
  end

  @batcher_payment_service_address Jason.decode!(config_json_string)
                                   |> Map.get("addresses")
                                   |> Map.get("batcherPaymentService")

  use Ethers.Contract,
    abi_file: "lib/abi/BatcherPaymentService.json",
    default_address: @batcher_payment_service_address

  def get_batcher_payment_service_address() do
    @batcher_payment_service_address
  end

  def get_fee_per_proof(%{merkle_root: merkle_root}) do
    BatcherPaymentServiceManager.EventFilters.task_created(
      merkle_root
      |> Utils.string_to_bytes32()
    )
    |> Ethers.get_logs(fromBlock: @first_block)
    |> case do
      {:ok, []} ->
        Logger.warning("No fee per proof events found for merkle root: #{merkle_root}.")
        0

      {:ok, events} ->
        event = events |> hd()
        fee_per_proof = event.data |> hd()
        Logger.debug("Fee per proof of #{merkle_root}: #{fee_per_proof} WEI.")

        fee_per_proof

      {:error, reason} ->
        Logger.error("Error getting fee per proof: #{inspect(reason)}.")
        raise("Error getting fee per proof events.")

      other ->
        Logger.error("Unexpected response on fee per proof events: #{inspect(other)}")
        raise("Unexpected response on fee per proof events.")
    end
  end
end
