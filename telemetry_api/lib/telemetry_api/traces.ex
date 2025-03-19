defmodule TelemetryApi.Traces do
  @moduledoc """
  The Traces context.
  """
  alias TelemetryApi.Traces.Trace
  alias TelemetryApi.Operators
  alias TelemetryApi.ContractManagers.StakeRegistry
  alias TelemetryApi.PrometheusMetrics

  require OpenTelemetry.Tracer
  require OpenTelemetry.Ctx
  alias OpenTelemetry.Tracer, as: Tracer
  alias OpenTelemetry.Ctx, as: Ctx

  use GenServer

  ########################################
  ############## PUBLIC API ##############
  ########################################
 
  def start_link(_) do
    GenServer.start_link(__MODULE__, :ok, name: __MODULE__)
  end
  
  @doc """
  Send the trace to OpenTelemetry

  This function is responsible for creating a new span and storing the context.

  ## Examples

      iex> merkle_root = "0x1234567890abcdef"
      iex> create_task_trace(merkle_root)
      :ok
  """
  def create_task_trace(merkle_root) do
    GenServer.call(__MODULE__, {:create_task_trace, merkle_root})
  end

  @doc """
  Registers an operator response in the task trace.

  ## Examples

      iex> merkle_root = "0x1234567890abcdef"
      iex> operator_id = "0x..."
      iex> register_operator_response(merkle_root, operator_id)
      :ok
  """
  def register_operator_response(merkle_root, operator_id) do
    GenServer.call(__MODULE__, {:register_operator_response, merkle_root, operator_id})
  end

  @doc """
  Registers the failure creating a batcher task in the task trace.

  ## Examples

      iex> merkle_root
      iex> error
      iex> batcher_task_creation_failed(merkle_root, error)
      :ok
  """
  def batcher_task_creation_failed(merkle_root, error) do
    GenServer.call(__MODULE__, {:batcher_task_creation_failed, merkle_root, error})
  end

  @doc """
  Create a new task trace for the batcher and starts the subspan for the batcher.

  ## Examples

      iex> merkle_root
      iex> create_batcher_task_trace(merkle_root)
      :ok
  """
  def create_batcher_task_trace(merkle_root) do
    GenServer.call(__MODULE__, {:create_batcher_task_trace, merkle_root})
  end

  @doc """
  Registers the uploading of a batcher task to S3 in the task trace.

  ## Examples

      iex> merkle_root
      iex> batcher_task_uploaded_to_s3(merkle_root)
      :ok
  """
  def batcher_task_uploaded_to_s3(merkle_root) do
    GenServer.call(__MODULE__, {:batcher_task_uploaded_to_s3, merkle_root})
  end

  @doc """
  Registers the sending of a batcher task to Ethereum in the task trace.

  ## Examples

      iex> merkle_root
      iex> tx_hash
      iex> batcher_task_sent(merkle_root, tx_hash)
      :ok
  """
  def batcher_task_sent(merkle_root, tx_hash) do
    GenServer.call(__MODULE__, {:batcher_task_sent, merkle_root, tx_hash})
  end

  @doc """
  Registers the start of the creation of a batcher task in the task trace.

  ## Examples

      iex> merkle_root
      iex> batcher_task_started(merkle_root)
      :ok
  """
  def batcher_task_started(merkle_root, fee_per_proof, total_proofs) do
    GenServer.call(__MODULE__, {:batcher_task_started, merkle_root, fee_per_proof, total_proofs})
  end

  @doc """
  Registers a reached quorum in the task trace.

  ## Examples

      iex> merkle_root = "0x1234567890abcdef"
      iex> quorum_reached(merkle_root)
      :ok
  """
  def quorum_reached(merkle_root) do
    GenServer.call(__MODULE__, {:quorum_reached, merkle_root})
  end

  @doc """
  Registers an error in the task trace.

  ## Examples

      iex> merkle_root = "0x1234567890abcdef"
      iex> error = "Some error.."
      iex> task_error(merkle_root, error)
      :ok
  """
  def task_error(merkle_root, error) do
    GenServer.call(__MODULE__, {:task_error, merkle_root, error})
  end

  @doc """
  Registers a set gas price when the aggregator tries to respond to a task in the task trace.

  ## Examples

      iex> merkle_root
      iex> gas_price
      iex> aggregator_task_set_gas_price(merkle_root, gas_price)
      :ok
  """
  def aggregator_task_set_gas_price(merkle_root, gas_price) do
    GenServer.call(__MODULE__, {:aggregator_task_set_gas_price, merkle_root, gas_price})
  end

  @doc """
  Registers the sending of an aggregator task to Ethereum in the task trace.

  ## Examples

      iex> merkle_root
      iex> tx_hash
      iex> aggregator_task_sent(merkle_root, tx_hash, effective_gas_price)
      :ok
  """
  def aggregator_task_sent(merkle_root, tx_hash, effective_gas_price) do
    GenServer.call(__MODULE__, {:aggregator_task_sent, merkle_root, tx_hash, effective_gas_price})
  end


  @doc """
  Finish the task trace

  This function is responsible for ending the span and cleaning up the context.

  ## Examples

      iex> merkle_root = "0x1234567890abcdef"
      iex> finish_task_trace(merkle_root)
      :ok
  """
  def finish_task_trace(merkle_root) do
    GenServer.call(__MODULE__, {:finish_task_trace, merkle_root})
  end

  
  ########################################
  ############### HANDLERS ###############
  ########################################

  @impl true
  def init(:ok) do
    {:ok, %{}}
  end

  @impl true
  def handle_call({:create_task_trace, merkle_root}, _from, traces) do
    with {:ok, trace} <- set_current_trace(traces, merkle_root),
      {:ok, total_stake} <- StakeRegistry.get_current_total_stake() do
        aggregator_subspan_ctx =
          Tracer.start_span(
            "Aggregator",
            %{
              attributes: [
                {:merkle_root, merkle_root},
                {:total_stake, total_stake}
              ]
            }
          )

        Tracer.set_current_span(aggregator_subspan_ctx)
        Tracer.add_event("New task event received", [])

        traces = store_trace(traces, merkle_root, %{
          trace
          | subspans: Map.put(trace.subspans, :aggregator, aggregator_subspan_ctx)
        })

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end

  @impl true
  def handle_call({:register_operator_response, merkle_root, operator_id}, _from, traces) do
    with {:ok, operator} <- Operators.get_operator(%{id: operator_id}),
         :ok <- validate_operator_registration(operator),
         {:ok, trace} <- set_current_trace_with_subspan(traces, merkle_root, :aggregator) do

      operator_stake = Decimal.new(operator.stake)
      new_stake = Decimal.add(trace.current_stake, operator_stake)
      new_stake_fraction = Decimal.div(new_stake, trace.total_stake)
      operator_stake_fraction = Decimal.div(operator_stake, trace.total_stake)

      Tracer.add_event(
        "Operator Response: " <> operator.name,
        [
          {:merkle_root, merkle_root},
          {:operator_id, operator_id},
          {:name, operator.name},
          {:address, operator.address},
          {:operator_stake, Decimal.to_string(operator_stake)},
          {:current_stake, Decimal.to_string(new_stake)},
          {:current_stake_fraction, Decimal.to_string(new_stake_fraction)},
          {:operator_stake_fraction, Decimal.to_string(operator_stake_fraction)}
        ]
      )

      responses = trace.responses ++ [operator_id]

      traces = store_trace(traces, merkle_root, %{
        trace
        | responses: responses,
          current_stake: new_stake
      })

      PrometheusMetrics.operator_response(
        operator.name <> " - " <> String.slice(operator.address, 0..7)
      )

      IO.inspect(
        "Operator response included. merkle_root: #{inspect(merkle_root)} operator_id: #{inspect(operator_id)}"
      )

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end

  @impl true
  def handle_call({:batcher_task_creation_failed, merkle_root, error}, _from, traces) do
    with {:ok, trace} <- set_current_trace_with_subspan(traces, merkle_root, :batcher) do
      Tracer.add_event(
        "Batcher Task Creation Failed",
        [
          {:error, error}
        ]
      )

      Tracer.end_span()

      traces = store_trace(traces, merkle_root, %{
        trace
        | subspans: Map.delete(trace.subspans, :batcher)
      })

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end

  @impl true
  def handle_call({:create_batcher_task_trace, merkle_root}, _from, traces) do
    with {:ok, total_stake} <- StakeRegistry.get_current_total_stake() do
      Ctx.clear()

      root_span_ctx =
        Tracer.start_span(
          "Task: #{merkle_root}",
          %{
            attributes: [
              {:merkle_root, merkle_root}
            ]
          }
        )

      ctx = Ctx.get_current()

      traces = store_trace(traces, merkle_root, %Trace{
        parent_span: root_span_ctx,
        context: ctx,
        total_stake: total_stake,
        current_stake: 0,
        responses: [],
        subspans: %{}
      })

      with {:ok, trace} <- set_current_trace(traces, merkle_root) do
        # This span ends inmediately after it's created just to set the correct title to the final task.
        Tracer.with_span "Task: #{merkle_root}" do
          Tracer.set_attributes(%{merkle_root: merkle_root})
        end

        batcher_subspan_ctx =
          Tracer.start_span(
            "Batcher",
            %{
              attributes: [
                {:merkle_root, merkle_root}
              ]
            }
          )

        Tracer.set_current_span(batcher_subspan_ctx)
        Tracer.add_event("New batch", [{:merkle_root, merkle_root}])

        traces = store_trace(traces, merkle_root, %{
          trace
          | subspans: Map.put(trace.subspans, :batcher, batcher_subspan_ctx)
        })

        {:reply, :ok, traces}
      end
    else
      error -> {:reply, handle_error(error), traces}
    end
  end

  @impl true
  def handle_call({:batcher_task_uploaded_to_s3, merkle_root}, _from, traces) do
    with {:ok, _trace} <- set_current_trace_with_subspan(traces, merkle_root, :batcher) do
      Tracer.add_event("Batcher Task Uploaded to S3", [])
      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end

  @impl true
  def handle_call({:batcher_task_sent, merkle_root, tx_hash}, _from, traces) do
    with {:ok, trace} <- set_current_trace_with_subspan(traces, merkle_root, :batcher) do
      Tracer.add_event("Batcher Task Sent to Ethereum", [{"tx_hash", tx_hash}])
      Tracer.end_span()
      traces = store_trace(traces, merkle_root, %{
        trace
        | subspans: Map.delete(trace.subspans, :batcher)
      })

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end


  @impl true
  def handle_call({:batcher_task_started, merkle_root, fee_per_proof, total_proofs}, _from, traces) do
    with {:ok, _trace} <- set_current_trace_with_subspan(traces, merkle_root, :batcher) do
      Tracer.add_event("Batcher Task being created",
        fee_per_proof: fee_per_proof,
        total_proofs: total_proofs
      )

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end
  
  @impl true
  def handle_call({:quorum_reached, merkle_root}, _from, traces) do
    with {:ok, _trace} <- set_current_trace_with_subspan(traces, merkle_root, :aggregator) do
      Tracer.add_event("Quorum Reached", [])
      IO.inspect("Reached quorum registered. merkle_root: #{merkle_root}")

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end

  @impl true
  def handle_call({:task_error, merkle_root, error}, _from, traces) do
    with {:ok, _trace} <- set_current_trace_with_subspan(traces, merkle_root, :aggregator) do
      Tracer.add_event(
        "Batch verification failed",
        [
          {:status, "error"},
          {:error, error}
        ]
      )

      IO.inspect("Task error registered. merkle_root: #{IO.inspect(merkle_root)}")

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end

  @impl true
  def handle_call({:aggregator_task_set_gas_price, merkle_root, gas_price}, _from, traces) do
    with {:ok, _trace} <- set_current_trace_with_subspan(traces, merkle_root, :aggregator) do
      Tracer.add_event("Gas price set", [{"gas_price", gas_price}])

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end
  
  @impl true
  def handle_call({:aggregator_task_sent, merkle_root, tx_hash, effective_gas_price}, _from, traces) do
    with {:ok, _trace} <- set_current_trace_with_subspan(traces, merkle_root, :aggregator) do
      Tracer.add_event("Task Sent to Ethereum", [{"tx_hash", tx_hash}, {"effective_gas_price", effective_gas_price}])

      {:reply, :ok, traces}
    else
      error -> {:reply, handle_error(error), traces}
    end
  end

  @impl true
  def handle_call({:finish_task_trace, merkle_root}, _from, traces) do
    with {:ok, trace} <- set_current_trace_with_subspan(traces, merkle_root, :aggregator) do
      missing_operators =
        Operators.list_operators()
        |> Enum.filter(fn o -> o.id not in trace.responses and Operators.is_registered?(o) end)

      add_missing_operators(missing_operators)

      Tracer.end_span()

      # Clean up the context
      with {:ok, _trace} <- set_current_trace(traces, merkle_root) do
        IO.inspect("Finished task trace with merkle_root: #{merkle_root}.")
        Tracer.end_span()
        traces = delete_trace(traces, merkle_root)

        {:reply, :ok, traces}
      end
    else
      error -> {:reply, handle_error(error), traces}
    end
  end


  #########################################
  ########## AUXILIARY FUNCTIONS ##########
  #########################################

  defp add_missing_operators([]), do: :ok

  # Updates the missing operator metric and adds a "Missing operator" trace event
  # for operators missing on the provided missing_operator parameter
  defp add_missing_operators(missing_operators) do
    # Concatenate name + address
    missing_operators =
      missing_operators
      |> Enum.map(fn op -> op.name <> " - " <> String.slice(op.address, 0..7) end)

    # Send to prometheus
    missing_operators
    |> Enum.map(fn o -> PrometheusMetrics.missing_operator(o) end)

    missing_operators =
      missing_operators |> Enum.join(";")

    Tracer.add_event("Missing Operators", [{:operators, missing_operators}])
  end

  # Validates that provided operator is registered
  defp validate_operator_registration(operator) do
    if Operators.is_registered?(operator) do
      :ok
    else
      {:error, :bad_request, "Operator not registered"}
    end
  end

  # Store the trace using the merkle_root as the key
  defp store_trace(traces, merkle_root, trace) do
    Map.put(traces, merkle_root, trace)
  end

  # Retrieve the trace by merkle_root
  defp get_trace(traces, merkle_root) do
    case Map.get(traces, merkle_root) do
      nil ->
        IO.inspect("Context not found for #{merkle_root}")
        {:error, :not_found, "Context not found for #{merkle_root}"}

      trace ->
        {:ok, trace}
    end
  end

  # Delete the trace after it's used
  defp delete_trace(traces, merkle_root) do
    Map.delete(traces, merkle_root)
  end

  # Sets the trace corresponding to the provided merkle_root 
  defp set_current_trace(traces, merkle_root) do
    with {:ok, trace} <- get_trace(traces, merkle_root) do
      Ctx.attach(trace.context)
      Tracer.set_current_span(trace.parent_span)
      {:ok, trace}
    end
  end

  # Sets the trace and subspan corresponding to the provided merkle_root and span_name
  defp set_current_trace_with_subspan(traces, merkle_root, span_name) do
    with {:ok, trace} <- get_trace(traces, merkle_root) do
      Ctx.attach(trace.context)
      Tracer.set_current_span(trace.subspans[span_name])
      {:ok, trace}
    end
  end

  # Handles different types of errors that may be returned on the GenServer
  defp handle_error(error) do
    case error do
      {:error, information} -> {:error, information}
      {:error, error_type, message} -> {:error, error_type, message}
      unknown -> {:error, unknown}
    end
  end
end
