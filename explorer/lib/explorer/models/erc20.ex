defmodule ERC20 do
  use Ethers.Contract,
  abi_file: "lib/abi/IERC20Metadata.json"

  def symbol(address) do
    ERC20.symbol() |> Ethers.call(to: address)
  end

  def name(address) do
    "address: #{address}" |> dbg
    ERC20.name() |> Ethers.call(to: address)
  end
end
