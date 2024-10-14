defmodule TelemetryApi.Periodic.OperatorFetcher do
  use GenServer
  alias TelemetryApi.Operators

  @wait_time_str = System.get_env("OPERATOR_FETCHER_WAIT_TIME_MS") ||
    raise """
    environment variable OPERATOR_FETCHER_WAIT_TIME_MS is missing.
    """

  @wait_time_ms (
    case Integer.parse(@wait_time_str) do
      :error -> raise("OPERATOR_FETCHER_WAIT_TIME_MS is not a number, received: #{@wait_time_str}")
      {num, _} -> num
    end
  )

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{})
  end

  def init(_) do
    send_work()
    {:ok, %{}}
  end

  def send_work() do
    :timer.send_interval(@wait_time_ms, :fetch_operators)
  end

  def handle_info(:fetch_operators, _state) do
        case Operators.fetch_all_operators() do
          {:ok, _} -> :ok
          {:error, message} -> IO.inspect "Couldn't fetch operators: #{IO.inspect message}"
        end
        {:noreply, %{}}
  end
end
