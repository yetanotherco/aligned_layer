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
  def string_to_bytes32(string) do
    binary = :erlang.list_to_binary(String.to_charlist(string))

    case byte_size(binary) do
      size when size < 32 -> binary <> :binary.copy(<<0>>, 32 - size)
      32 -> binary
      size when size > 32 -> binary |> binary_part(0, 32)
    end
    
  end
end
