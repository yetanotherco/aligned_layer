defmodule TelemetryApi.Traces do
  @moduledoc """
  The Traces context.
  """
  alias TelemetryApi.Traces.Trace
  alias TelemetryApi.Operators

  require OpenTelemetry.Tracer
  require OpenTelemetry.Ctx
  alias OpenTelemetry.Tracer, as: Tracer
  alias OpenTelemetry.Ctx, as: Ctx

  @doc """
  Send the trace to OpenTelemetry

  This function is responsible for creating a new span and storing the context in the Agent.

  ## Examples

      iex> merkle_root = "0x1234567890abcdef"
      iex> create_task_trace(merkle_root)
      {:ok, "merkle_root"}
  """
  def create_task_trace(merkle_root) do
    span_ctx =
      Tracer.start_span(
        "Task: #{merkle_root}",
        %{
          attributes: [
            {:merkle_root, merkle_root}
          ]
        }
      )

    ctx = Ctx.get_current()

    TraceStore.store_trace(merkle_root, %Trace{
      parent_span: span_ctx,
      context: ctx,
      responses: []
    })

    IO.inspect("New task trace with merkle_root: #{IO.inspect(merkle_root)}")
    {:ok, merkle_root}
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
    with operator <- Operators.get_operator_by_id(operator_id) do
      add_event(
        merkle_root,
        "Operator Response: " <> operator.name,
        [
          {:merkle_root, merkle_root},
          {:operator_id, operator_id},
          {:name, operator.name},
          {:address, operator.address},
          {:stake, operator.stake}
        ]
      )

      trace = TraceStore.get_trace(merkle_root)
      responses = trace.responses ++ [operator_id]
      TraceStore.store_trace(merkle_root, %{trace | responses: responses})

      IO.inspect(
        "Operator response included. merkle_root: #{IO.inspect(merkle_root)} operator_id: #{IO.inspect(operator_id)}"
      )

      {:ok, operator_id}
    end
  end

  @doc """
  Registers a reached quorum in the task trace.

  ## Examples

      iex> merkle_root = "0x1234567890abcdef"
      iex> quorum_reached(merkle_root)
      :ok
  """
  def quorum_reached(merkle_root) do
    add_event(
      merkle_root,
      "Quorum Reached",
      []
    )

    IO.inspect("Reached quorum registered. merkle_root: #{IO.inspect(merkle_root)}")

    {:ok, merkle_root}
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
    add_event(
      merkle_root,
      "Batch verification failed",
      [
        {:status, "error"},
        {:error, error}
      ]
    )

    IO.inspect("Task error registered. merkle_root: #{IO.inspect(merkle_root)}")
    {:ok, merkle_root}
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
    case TraceStore.get_trace(merkle_root) do
      nil ->
        IO.inspect("Context not found for #{merkle_root}")
        {:error, "Context not found for #{merkle_root}"}

      trace ->
        Ctx.attach(trace.context)
        Tracer.set_current_span(trace.parent_span)

        missing_operators =
          Operators.list_operators() |> Enum.filter(fn o -> o.id not in trace.responses end)

        add_missing_operators(missing_operators)

        Tracer.set_attributes(%{status: "completed"})

        Tracer.end_span(trace.parent_span)

        # Clean up the context from the Agent
        TraceStore.delete_trace(merkle_root)
        IO.inspect("Finished task trace with merkle_root: #{IO.inspect(merkle_root)}.")
        :ok
    end
  end

  defp add_missing_operators(merkle_root, []), do: :ok

  defp add_missing_operators(merkle_root, missing_operators) do
    missing_operators =
      missing_operators |> Enum.map(fn o -> o.name end) |> Enum.join(";")

    add_event(merkle_root, "Missing Operators", [{:operators, missing_operators}])
  end

  defp add_event(merkle_root, event_name, event_attributes) do
    case TraceStore.get_trace(merkle_root) do
      nil ->
        IO.inspect("Context not found for #{merkle_root}")
        {:error, "Context not found for #{merkle_root}"}

      trace ->
        Ctx.attach(trace.context)
        Tracer.set_current_span(trace.parent_span)

        Tracer.add_event(event_name, event_attributes)
    end
  end
end
