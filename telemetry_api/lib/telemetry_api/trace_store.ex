defmodule TraceStore do
  use Agent

  # Start the agent
  def start_link(_opts) do
    Agent.start_link(fn -> %{} end, name: __MODULE__)
  end

  # Store the trace using the merkle_root as the key
  def store_trace(merkle_root, trace) do
    Agent.update(__MODULE__, fn state ->
      Map.put(state, merkle_root, trace)
    end)
  end

  # Retrieve the trace by merkle_root
  def get_trace(merkle_root) do
    Agent.get(__MODULE__, fn state ->
      case Map.get(state, merkle_root) do
        nil ->
          IO.inspect("Context not found for #{merkle_root}")
          {:error, :not_found, "Context not found for #{merkle_root}"}

        trace ->
          {:ok, trace}
      end
    end)
  end

  # Delete the trace after it's used
  def delete_trace(merkle_root) do
    Agent.update(__MODULE__, fn state ->
      Map.delete(state, merkle_root)
    end)
  end
end
