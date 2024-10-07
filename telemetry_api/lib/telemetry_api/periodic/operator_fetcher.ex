defmodule TelemetryApi.Periodic.OperatorFetcher do
  use Task
  require Logger
  alias TelemetryApi.Operators
  alias TelemetryApi.ContractManagers.RegistryCoordinatorManager

  @never_registered 0
  @registered 1
  @deregistered 2

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
    Task.start_link(&poll_service/0)
  end

  defp poll_service() do
    receive do
    after
      @wait_time_ms ->
        fetch_operators_info()
        fetch_operators_status()
        poll_service()
    end
  end

  defp fetch_operators_info() do
    case Operators.fetch_all_operators() do
      {:ok, _} -> :ok
      {:error, message} -> IO.inspect("Couldn't fetch operators: #{IO.inspect(message)}")
    end
  end

  defp fetch_operators_status() do
    Operators.list_operators()
    |> Enum.map(fn op ->
      case RegistryCoordinatorManager.fetch_operator_status(op.address) do
        {:ok, status} ->

          Operators.update_operator(op, %{status: string_status(status)})

        error ->
          Logger.error("Error when updating status: #{error}")
      end
    end)
  end

  defp string_status(@never_registered), do: "NEVER_REGISTERED"
  defp string_status(@registered), do: "REGISTERED"
  defp string_status(@deregistered), do: "DEREGISTERED"
end
