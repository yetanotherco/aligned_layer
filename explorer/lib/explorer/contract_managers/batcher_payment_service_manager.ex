defmodule BatcherPaymentServiceManager do
  require Logger

  @environment System.get_env("ENVIRONMENT")
  @first_block (case @environment do
                  "devnet" -> 0
                  "holesky" -> 1_728_056
                  "mainnet" -> 20_020_000
                  _ -> raise("Invalid environment")
                end)

  payment_service_address = AlignedLayerServiceManager.get_batcher_payment_service_address()

  use Ethers.Contract,
    abi_file: "lib/abi/BatcherPaymentService.json",
    default_address: payment_service_address

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
