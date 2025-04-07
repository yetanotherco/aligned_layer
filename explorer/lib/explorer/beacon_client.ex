defmodule Explorer.BeaconClient do
  require Logger
  @beacon_url System.get_env("BEACON_CLIENT")
  @rpc_url System.get_env("RPC_URL")
  # See https://eips.ethereum.org/EIPS/eip-4844#parameters
  @versioned_hash_version_kzg 0x01

  def fetch_blob_by_versioned_hash(beacon_blob_hash, blob_versioned_hash) do
    {:ok, beacon_block} = get_beacon_block_header_by_hash(beacon_blob_hash)

    slot =
      String.to_integer(
        Map.get(Map.get(Map.get(Map.get(beacon_block, "data"), "header"), "message"), "slot")
      )

    case get_block_blobs(slot + 1) do
      {:ok, blobs} ->
        data = Map.get(blobs, "data")

        blob =
          Enum.find(data, fn blob ->
            get_blob_versioned_hash(blob) == blob_versioned_hash
          end)

        {:ok, blob}

      {:error, reason} ->
        {:error, reason}
    end
  end

  def get_blob_versioned_hash(blob) do
    kzg_commitment = String.replace(Map.get(blob, "kzg_commitment"), "0x", "")
    kzg_commitment = Base.decode16!(kzg_commitment, case: :mixed)
    hash = Explorer.Utils.sha256_hash_raw(kzg_commitment)
    # See https://eips.ethereum.org/EIPS/eip-4844#helpers
    <<_first::8, rest::binary>> = hash
    raw = <<@versioned_hash_version_kzg::8>> <> rest
    "0x" <> Base.encode16(raw, case: :lower)
  end

  def get_block_blobs(slot) do
    beacon_get("/eth/v1/beacon/blob_sidecars/#{slot}")
  end

  def get_beacon_block_header_by_hash(block_hash) do
    beacon_get("/eth/v1/beacon/headers/#{block_hash}")
  end

  def beacon_get(method) do
    headers = [{"Content-Type", "application/json"}]
    request = Finch.build(:get, "#{@beacon_url}#{method}", headers)
    response = Finch.request(request, Explorer.Finch)

    case response do
      {:ok, %Finch.Response{status: 200, body: body}} ->
        case Jason.decode(body) do
          {:ok, decoded_body} ->
            {:ok, decoded_body}

          {:error, _} ->
            {:error, :invalid_json}
        end

      {:ok, %Finch.Response{status: status}} ->
        {:error, status}

      {:error, reason} ->
        {:error, reason}
    end
  end
end
