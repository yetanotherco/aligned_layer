defmodule Explorer.BeaconClient do
  @beacon_url System.get_env("BEACON_API_URL")
  # See https://eips.ethereum.org/EIPS/eip-4844#parameters
  @versioned_hash_version_kzg 0x01

  def fetch_blob_by_versioned_hash(block_number, blob_versioned_hash) do
    case get_block_blobs(block_number) do
      {:ok, blobs} ->
        Enum.find(blobs, fn blob -> get_blob_versioned_hash(blob) == blob_versioned_hash end)

      {:error, reason} ->
        {:error, reason}
    end
  end

  def get_blob_versioned_hash(blob) do
    hash = Explorer.Utils.sha256_hash_raw(blob.kzg_commitment)
    # See https://eips.ethereum.org/EIPS/eip-4844#helpers
    <<_first::8, rest::binary>> = hash
    <<@versioned_hash_version_kzg::8>> <> rest
  end

  def get_block_blobs(block_number) do
    case beacon_get("/eth/v1/beacon/blob_sidecars/" <> block_number) do
      {:ok, res} -> res.data
      {:error, reason} -> {:error, reason}
    end
  end

  def beacon_get(method) do
    headers = [{"Content-Type", "application/json"}]
    request = Finch.build(:get, @beacon_url <> method, headers)
    response = Finch.request(request, Explorer.Finch)

    case status do
      {:ok, %Finch.Response{status: 200, body: body}} ->
        case Jason.decode(body) do
          {:ok, decoded_body} -> {:ok, decoded_body}
          {:error, _} -> {:error, :invalid_json}
        end

      {:ok, %Finch.Response{status: status}} ->
        {:error, status}

      {:error, reason} ->
        {:error, reason}
    end
  end
end
