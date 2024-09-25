defmodule TelemetryApi.Traces.Trace do
  @enforce_keys [:parent_span, :context]
  defstruct [:parent_span, :context]
end
