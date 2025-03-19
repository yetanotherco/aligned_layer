defmodule EthConverter do
  use HTTPoison.Base

  @wei_per_eth 1_000_000_000_000_000_000
  @cache_ttl :timer.minutes(5)

  def wei_to_eth(wei, decimal_places \\ 18)

  def wei_to_eth(wei, _decimal_places) when is_nil(wei) do
    nil
  end

  def wei_to_eth(%Decimal{} = wei, decimal_places) do
    wei
    |> Decimal.div(Decimal.new(@wei_per_eth))
    |> Decimal.round(decimal_places)
    |> Decimal.to_string(:normal)
  end

  def wei_to_eth(wei, decimal_places) when is_integer(wei) do
    wei
    |> Decimal.new()
    |> Decimal.div(Decimal.new(@wei_per_eth))
    |> Decimal.round(decimal_places)
    |> Decimal.to_string(:normal)
  end

  def wei_to_eth(wei, decimal_places) when is_binary(wei) do
    wei
    |> String.to_integer()
    |> wei_to_eth(decimal_places)
  end

  def wei_to_usd(wei, decimal_places \\ 0)

  def wei_to_usd(nil, _), do: {:error, "nil value"}

  def wei_to_usd(wei, decimal_places) do
    with eth_amount <- wei_to_eth(wei, 18),
         {:ok, eth_price} <- get_eth_price_usd() do
      usd_value =
        Decimal.mult(Decimal.new(to_string(eth_amount)), Decimal.new(to_string(eth_price)))

      {:ok, Decimal.round(usd_value, decimal_places) |> Decimal.to_string(:normal)}
    else
      {:error, reason} -> {:error, reason}
    end
  end

  def wei_to_usd_sf(wei, significant_figures \\ 3)
  def wei_to_usd_sf(nil, _), do: {:error, "nil value"}

  # rounds to significant figures, instead of decimal places
  def wei_to_usd_sf(wei, significant_figures) do
    with eth_amount <- wei_to_eth(wei, 18),
         {:ok, eth_price} <- get_eth_price_usd() do
      usd_value =
        Decimal.mult(Decimal.new(to_string(eth_amount)), Decimal.new(to_string(eth_price)))

      rounded_value = round_to_sf(usd_value, significant_figures)

      {:ok, Decimal.to_string(rounded_value, :normal)}
    else
      {:error, reason} -> {:error, reason}
    end
  end

  defp round_to_sf(%Decimal{coef: 0} = _value, _sf), do: Decimal.new("0")

  defp round_to_sf(value, significant_figures) do
    # Convert the value to a float and calculate the magnitude
    value_float = Decimal.to_float(value)
    magnitude = :math.log10(abs(value_float)) |> floor()

    # Calculate the factor to multiply by for shifting the decimal point
    factor = :math.pow(10, significant_figures - 1 - magnitude)

    # Round, then shift back
    rounded_value = value_float
                    |> Kernel.*(factor)
                    |> round()
                    |> Kernel./(factor)

    Decimal.new(Float.to_string(rounded_value))
  end

  def multiply_eth_by_usd(eth, usd_price) do
    eth_float = to_float(eth)
    usd_float = to_float(usd_price)

    result = eth_float * usd_float

    :erlang.float_to_binary(result, decimals: 5)
  end

  defp to_float(value) when is_binary(value), do: String.to_float(value)
  defp to_float(value) when is_float(value), do: value
  defp to_float(value) when is_integer(value), do: value / 1
  defp to_float(_), do: 0.0

  def wei_to_eth_decimal(wei) when is_integer(wei) do
    wei_to_eth_decimal(Integer.to_string(wei))
  end

  def wei_to_eth_decimal(wei) when is_binary(wei) do
    {:ok, Decimal.div(Decimal.new(wei), Decimal.new(@wei_per_eth))}
  end

  @base_url "https://api.coingecko.com/api/v3"

  def get_eth_price_usd do
    Cachex.get(:eth_price_cache, :eth_price)
    |> case do
      {:ok, nil} ->
        fetch_and_cache_eth_price()

      {:ok, price} ->
        {:ok, price}

      {:error, reason} ->
        {:error, reason}
    end
  end

  defp fetch_and_cache_eth_price do
    case get("/simple/price?ids=ethereum&vs_currencies=usd") do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        with {:ok, price} <- parse_response(body) do
          Cachex.put(:eth_price_cache, :eth_price, price, ttl: @cache_ttl)
          {:ok, price}
        end

      {:ok, %HTTPoison.Response{status_code: status_code}} ->
        {:error, "Request failed with status code: #{status_code}"}

      {:error, %HTTPoison.Error{reason: reason}} ->
        {:error, "HTTP request failed: #{reason}"}
    end
  end

  def parse_response(body) do
    case Jason.decode(body) do
      {:ok, %{"ethereum" => %{"usd" => price}}} ->
        {:ok, price}

      _ ->
        {:error, "Failed to parse response"}
    end
  end

  def process_url(url) do
    @base_url <> url
  end

  def process_response_body(body) do
    body
  end
end
