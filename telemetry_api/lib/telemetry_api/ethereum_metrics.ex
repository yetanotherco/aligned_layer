defmodule TelemetryApi.EthereumMetrics do
  use Prometheus.Metric

  @gauge [name: :gas_price, help: "Ethereum Gas Price.", labels: []]

  def new_gas_price(gas_price) do
    Gauge.set(
      [name: :gas_price, labels: []],
      gas_price
    )
  end
end
