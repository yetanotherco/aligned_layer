defmodule ExplorerWeb.Utils do
  def shorten_block_hash(block_hash) do
    case String.length(block_hash) do
      n when n < 6 -> block_hash
      _ -> "#{String.slice(block_hash, 0, 6)}...#{String.slice(block_hash, -4, 4)}"
    end
  end

  def convert_number_to_shorthand(number) when number >= 1_000_000 do
    "#{div(number, 1_000_000)}M"
  end

  def convert_number_to_shorthand(number) when number >= 10_000 do
    "#{div(number, 10_000)}k"
  end

  def convert_number_to_shorthand(number) when number >= 1_000 do
    "#{div(number, 1_000)}k"
  end

  def convert_number_to_shorthand(number) when number >= 0 do
    "#{number}"
  end

  def convert_number_to_shorthand(_number), do: "Invalid number"
end

defmodule Utils do
  def string_to_bytes32(hex_string) do
    # Remove the '0x' prefix
    hex = case hex_string do
      "0x" <> _ -> String.slice(hex_string, 2..-1//1)
      _ -> raise "Invalid hex string, missing '0x' prefix"
    end

    # Convert the hex string to a binary
    case Base.decode16(hex, case: :mixed) do
      {:ok, binary} -> binary
      _ -> raise "Invalid hex string"
    end
  end

  def get_last_n_items(events, n) when is_list(events) and is_integer(n) and n >= 0 do
    events
    |> Enum.reverse()
    |> Enum.take(n)
    |> Enum.reverse()
  end

end
