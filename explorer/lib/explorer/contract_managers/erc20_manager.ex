defmodule ERC20Manager do
  use Ethers.Contract,
  abi_file: "lib/abi/IERC20Metadata.json"

  def symbol(address) do
    ERC20Manager.symbol() |> Ethers.call(to: address)
  end

  def name(address) do
    ERC20Manager.name() |> Ethers.call(to: address)
  end
end
