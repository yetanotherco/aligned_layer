defmodule ERC20InterfaceManager do
  use Ethers.Contract,
  abi_file: "lib/abi/IERC20Metadata.json"

  def symbol("0x") do
    {:ok, "ETH"}
  end
  def symbol(address) do
    ERC20InterfaceManager.symbol() |> Ethers.call(to: address)
  end

  def name("0x") do
    {:ok, "Native ETH"}
  end
  def name(address) do
    ERC20InterfaceManager.name() |> Ethers.call(to: address)
  end
end
