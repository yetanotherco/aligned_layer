defmodule ExplorerWeb.Batch.Index do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    merkle_root = params["merkle_root"]

    if merkle_root == nil do
      {
        :empty,
        assign(socket, newBatchEvent: :empty, batchWasResponded: :empty)
      }
    end

    current_batch = Batches.get_batch(%{merkle_root: merkle_root})
    "current_batch_frontend" |> IO.inspect()
    current_batch |> IO.inspect() #it has empties, i think im inserting wrong to the DB

    {
      :ok,
      assign(socket,
        merkle_root: merkle_root,
        current_batch: current_batch,
        page_title: Utils.shorten_hash(merkle_root)
      )
    }
  rescue
    _ ->
      # TODO handle different the 'without 0x prefix' error, for usability
      # ex.message == "Invalid hex string" or ex.message == "Invalid hex string, missing '0x' prefix" do
      {:ok, assign(socket, merkle_root: :empty, newBatchInfo: :empty, batchWasResponded: :empty)}
  end

  embed_templates "*"
end
