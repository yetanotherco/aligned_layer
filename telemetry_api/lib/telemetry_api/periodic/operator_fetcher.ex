defmodule TelemetryApi.Periodic.OperatorFetcher do
  use Task
  alias TelemetryApi.Operators

  wait_time_str = System.get_env("OPERATOR_FETCHER_WAIT_TIME_MS") ||
    raise """
    environment variable OPERATOR_FETCHER_WAIT_TIME_MS is missing.
    """

  @wait_time_ms (
    case Integer.parse(wait_time_str) do
      :error -> raise("OPERATOR_FETCHER_WAIT_TIME_MS is not a number, received: #{wait_time_str}")
      {num, _} -> num
    end
  )

  def start_link(_) do
    Task.start_link(&poll_serivce/0)
  end

  defp poll_service() do
    receive do
    after
      @wait_time_ms ->
        case Operators.fetch_all_operators() do
          {:ok, _} -> :ok
          {:error, message} -> IO.inspect "Couldn't fetch operators: #{IO.inspect message}"
        end
        poll_service()
    end
  end
end
