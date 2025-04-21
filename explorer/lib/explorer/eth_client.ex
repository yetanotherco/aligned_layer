defmodule Explorer.EthClient do
  require Logger
  @rpc_url System.get_env("RPC_URL")

  def get_block_by_number(block_number) do
    eth_send("eth_getBlockByNumber", [block_number, false])
  end

  defp eth_send(method, params, id \\ 1) do
    headers = [{"Content-Type", "application/json"}]
    body = Jason.encode!(%{jsonrpc: "2.0", method: method, params: params, id: id})
    request = Finch.build(:post, @rpc_url, headers, body)
    response = Finch.request(request, Explorer.Finch, [])

    case response do
      {:ok, %Finch.Response{status: 200, body: body}} ->
        case Jason.decode(body) do
          {:ok, %{error: error} = _} -> {:error, error.message}
          {:ok, body} -> {:ok, Map.get(body, "result")}
          {:error, _} -> {:error, :invalid_json}
        end

      {:ok, %Finch.Response{status: status}} ->
        {:error, status}

      {:error, reason} ->
        {:error, reason}
    end
  end
end
