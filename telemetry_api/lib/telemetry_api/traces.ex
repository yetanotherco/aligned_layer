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
      context: ctx
    })

    IO.inspect("New task trace with merkle_root: #{IO.inspect(merkle_root)}")
    :ok
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
    with {:ok, operator} <- Operators.get_operator_by_id(operator_id),
         :ok <-
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
           ) do
      IO.inspect(
        "Operator response included. merkle_root: #{IO.inspect(merkle_root)} operator_id: #{IO.inspect(operator_id)}"
      )

      :ok
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
    with :ok <-
           add_event(
             merkle_root,
             "Quorum Reached",
             []
           ) do
      IO.inspect("Reached quorum registered. merkle_root: #{IO.inspect(merkle_root)}")
      :ok
    end
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
    with :ok <-
           add_event(
             merkle_root,
             "Batch verification failed",
             [
               {:status, "error"},
               {:error, error}
             ]
           ) do
      IO.inspect("Task error registered. merkle_root: #{IO.inspect(merkle_root)}")
      :ok
    end
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
    with {:ok, trace} <- TraceStore.get_trace(merkle_root) do
      Ctx.attach(trace.context)
      Tracer.set_current_span(trace.parent_span)
      Tracer.set_attributes(%{status: "completed"})

      Tracer.end_span(trace.parent_span)

      # Clean up the context from the Agent
      TraceStore.delete_trace(merkle_root)
      IO.inspect("Finished task trace with merkle_root: #{IO.inspect(merkle_root)}.")
      :ok
    end
  end

  defp add_event(merkle_root, event_name, event_attributes) do
    with {:ok, trace} <- TraceStore.get_trace(merkle_root) do
      Ctx.attach(trace.context)
      Tracer.set_current_span(trace.parent_span)

      Tracer.add_event(event_name, event_attributes)
      :ok
    end
  end
end
