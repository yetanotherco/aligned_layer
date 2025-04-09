defmodule Explorer.BeaconClient do
  require Logger
  @beacon_url System.get_env("BEACON_CLIENT")
  # See https://eips.ethereum.org/EIPS/eip-4844#parameters
  @versioned_hash_version_kzg 0x01

  def fetch_blob_by_versioned_hash!(slot, blob_versioned_hash) do
    {:ok, blobs} = get_block_blobs(slot)
    data = Map.get(blobs, "data")

    Enum.find(data, fn blob ->
      get_blob_versioned_hash(blob) == blob_versioned_hash
    end)
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

  def get_block_slot(beacon_block) do
    String.to_integer(
      beacon_block
      |> Map.get("data")
      |> Map.get("header")
      |> Map.get("message")
      |> Map.get("slot")
    )
  end

  def get_block_header_by_hash(block_hash) do
    beacon_get("/eth/v1/beacon/headers/#{block_hash}")
  end

  def get_block_header_by_parent_hash(parent_block_hash) do
    case beacon_get("/eth/v1/beacon/headers?parent_root=#{parent_block_hash}") do
      {:ok, header} ->
        data = header["data"] |> Enum.at(0)

        {:ok, %{header | "data" => data}}

      other ->
        other
    end
  end

  def get_block_blobs(slot) do
    beacon_get("/eth/v1/beacon/blob_sidecars/#{slot}")
  end

  defp beacon_get(method) do
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
