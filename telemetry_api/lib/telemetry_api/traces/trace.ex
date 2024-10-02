defmodule TelemetryApi.Traces.Trace do
  @enforce_keys [:parent_span, :context, :responses]
  defstruct [:parent_span, :context, :responses]
end
