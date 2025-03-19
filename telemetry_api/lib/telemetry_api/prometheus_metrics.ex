defmodule TelemetryApi.PrometheusMetrics do
  use Prometheus.Metric

  @gauge [name: :gas_price, help: "Ethereum Gas Price.", labels: []]
  @counter [name: :missing_operator_count, help: "Missing Operators", labels: [:operator]]
  @counter [name: :operator_response_count, help: "Operator Response Count", labels: [:operator]]

  def new_gas_price(gas_price) do
    Gauge.set(
      [name: :gas_price, labels: []],
      gas_price
    )
  end

  def missing_operator(operator) do
    Counter.inc(
      name: :missing_operator_count,
      labels: [operator]
    )
  end

  def operator_response(operator) do
    Counter.inc(
      name: :operator_response_count,
      labels: [operator]
    )
  end

  def initialize_operator_metrics(operator) do
    value =
      Counter.value(
        name: :missing_operator_count,
        labels: [operator]
      )

    if value == :undefined do
      Counter.inc(
        [name: :missing_operator_count, labels: [operator]],
        0
      )

      Counter.inc(
        [name: :operator_response_count, labels: [operator]],
        0
      )
    end
  end
end
