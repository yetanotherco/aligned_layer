defmodule NetworkStorage do
  require Logger

  def extract_info_from_data_pointer(%BatchDB{} = batch) do
    Logger.debug("Extracting batch's proofs info: #{batch.merkle_root}")

    batch_data_pointer =
      batch.data_pointer
      |> fetch_batch_data_pointer()

    # only get from s3 if not already in DB
    proof_hashes =
      case Proofs.get_proofs_from_batch(%{merkle_root: batch.merkle_root}) do
        nil ->
          Logger.debug("Fetching proof hashes from S3")
          batch_data_pointer |> calculate_proof_hashes()

        proof_hashes ->
          # already processed and stored the S3 data
          Logger.debug("Fetching proof hashes from DB")
          proof_hashes
      end

    proving_system = case Proofs.get_proving_systems_from_batch(%{merkle_root: batch.merkle_root}) do
      nil ->
        Logger.debug("Fetching proving system from S3")
        batch_data_pointer |> get_proving_system()

      proving_system ->
        # already processed and stored the S3 data
        Logger.debug("Fetching proving system from DB")
        proving_system
    end

    batch
    |> Map.put(:proof_hashes, proof_hashes)
    |> Map.put(:proving_systems, proving_system)
    |> Map.put(:amount_of_proofs, proof_hashes |> Enum.count())
  end

  def fetch_batch_data_pointer(batch_data_pointer) do
    case Finch.build(:get, batch_data_pointer) |> Finch.request(Explorer.Finch) do
      {:ok, %Finch.Response{status: 200, body: body}} ->
        case decode_body(body) do
          {:ok, data} -> {:ok, data}
          {:error, reason} -> {:error, reason}
        end

      {:ok, %Finch.Response{status: status_code}} ->
        {:error, {:http_error, status_code}}

      {:error, reason} ->
        {:error, {:http_error, reason}}
    end
  end

  defp decode_body(body) when is_binary(body) do
    with {:ok, json} <- Jason.decode(body) do
      {:ok, json}
    else
      _ ->
        case CBOR.decode(body) do
          {:ok, cbor_data, _} -> {:ok, cbor_data}
          {:error, reason} -> {:error, {:cbor_decode, reason}}
          _other -> {:error, :unknown_format}
        end
    end
  end

  def calculate_proof_hashes({:ok, deserialized_batch}) do
    deserialized_batch
    |> Enum.map(fn s3_object ->
      :crypto.hash(:sha3_256, s3_object["proof"])
    end)
  end

  def calculate_proof_hashes({:error, reason}) do
    Logger.error("Error calculating proof hashes: #{inspect(reason)}")
    []
  end

  def get_proving_system({:ok, deserialized_batch}) do
    deserialized_batch
    |> Enum.map(fn s3_object ->
      s3_object["proving_system"]
    end)
  end

  def get_proving_system({:error, reason}) do
    Logger.error("Error getting proving system: #{inspect(reason)}")
    nil
  end
end
