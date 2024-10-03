defmodule TelemetryApi.Periodic.OperatorFetcher do
  use Task
  alias TelemetryApi.Operators
  alias TelemetryApi.ContractManagers.RegistryCoordinatorManager

  wait_time_str =
    "5" ||
      raise """
      environment variable OPERATOR_FETCHER_WAIT_TIME_MS is missing.
      """

  @wait_time_ms (case Integer.parse(wait_time_str) do
                   :error ->
                     raise(
                       "OPERATOR_FETCHER_WAIT_TIME_MS is not a number, received: #{wait_time_str}"
                     )

                   {num, _} ->
                     num
                 end)

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
      active = RegistryCoordinatorManager.is_operator_active?(op.address)
      IO.inspect("OPERATOR #{op.address} STATUS #{inspect(active)}")
      Operators.update_operator(%{op | active: active})
    end)
  end
end
