defmodule ExplorerWeb.Home.Index do
  require Logger
  use ExplorerWeb, :live_view

  @impl true
  def handle_info(_, socket) do
    verified_batches = Batches.get_amount_of_verified_batches()

    operators_registered = AVSDirectory.get_operators_registered()

    latest_batches =
      Batches.get_latest_batches(%{amount: 5})
      # extract only the merkle root
      |> Enum.map(fn %Batches{merkle_root: merkle_root} -> merkle_root end)

    verified_proofs = Batches.get_amount_of_verified_proofs()

    {:noreply,
     assign(
       socket,
       verified_batches: verified_batches,
       operators_registered: operators_registered,
       latest_batches: latest_batches,
       verified_proofs: verified_proofs
     )}
  end

  @impl true
  def mount(_, _, socket) do
    verified_batches = Batches.get_amount_of_verified_batches()

    operators_registered = AVSDirectory.get_operators_registered()

    latest_batches =
      Batches.get_latest_batches(%{amount: 5})
      # extract only the merkle root
      |> Enum.map(fn %Batches{merkle_root: merkle_root} -> merkle_root end)

    verified_proofs = Batches.get_amount_of_verified_proofs()

    Phoenix.PubSub.subscribe(Explorer.PubSub, "update_views")

    {:ok,
     assign(socket,
       verified_batches: verified_batches,
       operators_registered: operators_registered,
       latest_batches: latest_batches,
       verified_proofs: verified_proofs,
       service_manager_address:
         AlignedLayerServiceManager.get_aligned_layer_service_manager_address(),
       network: System.get_env("ENVIRONMENT"),
       page_title: "Welcome"
     )}
  rescue
    e in Mint.TransportError ->
      case e do
        %Mint.TransportError{reason: :econnrefused} ->
          {
            :ok,
            assign(socket,
              verified_batches: :empty,
              operators_registered: :empty,
              latest_batches: :empty,
              verified_proofs: :empty
            )
            |> put_flash(:error, "Could not connect to the backend, please try again later.")
          }

        _ ->
          IO.puts("Other transport error: #{inspect(e)}")
      end

    e in FunctionClauseError ->
      case e do
        %FunctionClauseError{
          module: ExplorerWeb.Home.Index
        } ->
          {
            :ok,
            assign(socket,
              verified_batches: :empty,
              operators_registered: :empty,
              latest_batches: :empty,
              verified_proofs: :empty
            )
            |> put_flash(:error, "Something went wrong with the RPC, please try again later.")
          }
      end

    e ->
      Logger.error("Other error: #{inspect(e)}")
      {:ok, socket |> put_flash(:error, "Something went wrong, please try again later.")}
  end

  embed_templates "*"
end
