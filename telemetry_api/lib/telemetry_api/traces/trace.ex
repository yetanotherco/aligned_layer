defmodule TelemetryApi.Traces.Trace do
  @enforce_keys [:parent_span, :context, :total_stake, :current_stake, :responses]
  defstruct [:parent_span, :context, :total_stake, :current_stake, :responses]
end
