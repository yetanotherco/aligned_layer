defmodule ExplorerWeb.TestController do
  use ExplorerWeb, :controller

  def test(conn, _params) do
    # import MyERC20Token

    # get_erc20_name() |> elem(1) |> IO.puts()

    render(conn, :test, message: MyERC20Token.get_erc20_name() |> elem(1))
  end
end
