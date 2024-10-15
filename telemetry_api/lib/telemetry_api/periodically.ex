defmodule TelemetryApi.Periodically do
  use GenServer
  alias TelemetryApi.Operators
  alias TelemetryApi.EthereumMetrics

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
    GenServer.start_link(__MODULE__, %{})
  end

  def init(_) do
    send_work()
    {:ok, %{}}
  end

  def send_work() do
    one_second = 1000
    :timer.send_interval(@wait_time_ms, :fetch_operators)
    :timer.send_interval(one_second * 10, :gas_price) # every 10 seconds, once per block + some margin
  end

  def handle_info(:fetch_operators, _state) do
        case Operators.fetch_all_operators() do
          {:ok, _} -> :ok
          {:error, message} -> IO.inspect "Couldn't fetch operators: #{IO.inspect message}"
        end
        {:noreply, %{}}
  end

  def handle_info(:gas_price, _state) do
    case Ethers.current_gas_price() do
      {:ok, gas_price} ->
        EthereumMetrics.new_gas_price(gas_price)

      {:error, error} ->
        IO.inspect("Error fetching gas price: #{error}")
    end
        {:noreply, %{}}
  end
end
