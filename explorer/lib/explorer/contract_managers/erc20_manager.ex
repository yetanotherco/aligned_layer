defmodule ERC20Manager do
  use Ethers.Contract,
  abi_file: "lib/abi/IERC20Metadata.json"

  def symbol("0x") do
    {:ok, "ETH"}
  end
  def symbol(address) do
    ERC20Manager.symbol() |> Ethers.call(to: address)
  end

  def name("0x") do
    {:ok, "Native ETH"}
  end
  def name(address) do
    ERC20Manager.name() |> Ethers.call(to: address)
  end
end
