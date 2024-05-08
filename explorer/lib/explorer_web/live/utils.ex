defmodule ExplorerWeb.Utils do
  def shorten_block_hash(block_hash) do
    case String.length(block_hash) do
      n when n < 6 -> block_hash
      _ -> "#{String.slice(block_hash, 0, 6)}...#{String.slice(block_hash, -4, 4)}"
    end
  end
end
