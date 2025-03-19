defmodule ExplorerWeb.DataController do
  use ExplorerWeb, :controller

  def verified_batches_summary(conn, _params) do
    batches_summary =
      Batches.get_daily_verified_batches_summary()

    render(conn, :show, %{
      batches: batches_summary,
    })
  end
end
