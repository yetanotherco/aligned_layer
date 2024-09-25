defmodule TelemetryApi.Traces do
  @moduledoc """
  The Traces context.
  """
  alias TelemetryApi.Traces.Trace

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
    span_ctx = Tracer.start_span(
      "Task: #{merkle_root}",
      %{attributes: [
        {:merkle_root, merkle_root}
        ]}
      )
    ctx = Ctx.get_current()
    
    TraceStore.store_trace(merkle_root, %Trace{
      parent_span: span_ctx,
      context: ctx
    })
    IO.inspect(merkle_root)
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
        Tracer.set_attributes(%{status: "completed"})

        Tracer.end_span(trace.parent_span)

        # Clean up the context from the Agent
        TraceStore.delete_trace(merkle_root)
        IO.inspect(merkle_root)
        :ok
    end
  end
end
