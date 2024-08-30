defmodule ExplorerWeb.Calculator.Index do
  use ExplorerWeb, :live_view

  @aggregator_cost 400_000
  @batcher_submission_base_cost 100_000
  @additional_submission_cost_per_proof 13_000
  @constant_cost @aggregator_cost + @batcher_submission_base_cost

  @doc """
  The main components are:
      * **Cost of BLS aggregator task response in Ethereum**: ~constant, 400000 gas. Can vary depending on the amount of Operators that didn't sign but we can't know this beforehand). This cost is paid by the aggregator when interacting with the *ServiceManager* and must be refunded to it.
      * **Cost of creating the task**: $BaseCost$ (~100000 gas) + $CostPerProof$ (~13000 gas) x $NProofs$. The cost per proof is related to a for loop in the EVM. This cost is paid by the Batcher when interacting with the *BatcherPaymentService* and must be refunded to it.

  In the end, the total cost is,

    $C(n) = TaskResponseCost + BaseTaskCreationCost + CostPerProof * n$

  where $n$ is the number of proofs in a batch.

  The cost per proof is then,

    $c(n) = \frac{TaskResponseCost + BaseTaskCreationCost}{n} + CostPerProof$

  The value of $c(n)$ for a batch of $n$ proofs is what is charged to the user.

    $C(n) = \frac{TaskResponseCost + BaseTaskCreationCost}{n} + CostPerProof$

  """

  @impl true
  def mount(_, _, socket) do
    {:ok, assign(socket, number_of_proofs: 0)}
  end

  @impl true
  def handle_event("change_number_of_proofs", %{"proofs" => number_of_proofs}, socket) do
    {:noreply, socket |> assign(number_of_proofs: number_of_proofs)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 px-1 text-foreground max-w-[27rem] sm:max-w-3xl md:max-w-5xl mx-auto">
      <.card_preheding>
        Calculator
      </.card_preheding>
      <section class="space-y-3 text-base leading-7">
        <p>
          ALIGNED verifies your proofs for less than 10% of the cost of using Ethereum directly.
          <br /> Let's see how much you can save by verifying your proofs using:
        </p>
        <div class="flex items-center gap-3">
          <form phx-submit="change_number_of_proofs">
            <label for="proofs" class="text-foreground sr-only">Number of Proofs: </label>
            <input
              name="proofs"
              id="proofs"
              type="number"
              class={
                classes([
                  "border border-foreground/20 text-muted-foreground w-20 focus:ring-primary",
                  "phx-submit-loading:opacity-75 rounded-lg bg-card hover:bg-muted py-2 px-3",
                  "text-sm font-semibold leading-6 text-foregound active:text-foregound/80",
                  "[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                ])
              }
              value={@number_of_proofs}
              min="0"
            />
          </form>
          <form phx-submit="change_number_of_proofs" class="w-full">
            <input
              name="proofs"
              id="proofs_slider"
              type="range"
              class={
                classes([
                  "w-full appearance-none h-1 rounded-ful bg-muted",
                  "[&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:size-4 [&::-webkit-slider-thumb]:bg-accent",
                  "[&::-moz-range-thumb]:size-4 [&::-moz-range-thumb]:border-0 [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:bg-accent",
                  "[&::-moz-range-track]:bg-muted [&::-moz-range-track]:rounded-full"
                ])
              }
              value={@number_of_proofs}
              min="0"
              max="10000"
              phx-change="change_number_of_proofs"
            />
          </form>
          <p>
            <%= Numbers.format_number(10000) %>
          </p>
        </div>
        <p>
          Your estimated cost for verifying <%= @number_of_proofs |> Numbers.format_number() %> proofs in ALIGNED is
          <span class="text-xl font-bold ml-1">
            <%= if @number_of_proofs > 0 do %>
              <%= @number_of_proofs |> calculate_cost() |> Numbers.format_number() %>
            <% else %>
              0
            <% end %>
            WEI.
          </span>
        </p>
      </section>
    </div>
    """
  end

  defp calculate_cost(number_of_proofs) do
    case Integer.parse(number_of_proofs) |> elem(0) do
      n when n > 0 ->
        div(@constant_cost, n) + @additional_submission_cost_per_proof

      _ ->
        0
    end
  end
end
