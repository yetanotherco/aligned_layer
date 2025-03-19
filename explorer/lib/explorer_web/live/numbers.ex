defmodule Numbers do
  def format_number(number) when is_nil(number) or number == :empty do
    nil
  end

  def format_number(number) when is_integer(number) do
    do_format_number(Integer.to_string(number))
  end

  def format_number(number) when is_float(number) do
    number
    |> Float.to_string()
    |> format_number()
  end

  def format_number(number) when is_binary(number) do
    case String.split(number, ".") do
      [integer_part] ->
        case Integer.parse(integer_part) do
          {_int, ""} -> do_format_number(integer_part)
          _ -> raise ArgumentError, "Invalid number string: #{inspect(number)}"
        end

      [integer_part, decimal_part] ->
        case Integer.parse(integer_part) do
          {_int, ""} -> do_format_number(integer_part) <> "." <> decimal_part
          _ -> raise ArgumentError, "Invalid number string: #{inspect(number)}"
        end

      _ ->
        raise ArgumentError, "Invalid number string: #{inspect(number)}"
    end
  end

  defp do_format_number(number_string) do
    number_string
    |> String.reverse()
    |> String.graphemes()
    |> Enum.chunk_every(3)
    |> Enum.join(",")
    |> String.reverse()
  end

  def show_percentage(%Decimal{} = weight) do
    weight
    |> Decimal.mult(100)
    |> Decimal.round(2)
    |> Decimal.to_string(:normal)
    |> Kernel.<>("%")
  end

  def show_percentage(weight) when is_float(weight) do
    weight
    |> Decimal.from_float()
    |> show_percentage()
  end

  def show_percentage(weight) when is_binary(weight) do
    weight
    |> Decimal.new()
    |> show_percentage()
  end

  def show_percentage(weight) when is_integer(weight) do
    weight
    |> Decimal.new()
    |> show_percentage()
  end
end
