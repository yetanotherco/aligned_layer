defmodule TelemetryApiWeb.TraceController do
  use TelemetryApiWeb, :controller

  alias TelemetryApi.Traces

  action_fallback TelemetryApiWeb.FallbackController

  @doc """
  Create a trace for a NewTask with the given merkle_root
  Method: POST initTaskTrace
  """
  def create_task_trace(conn, %{"merkle_root" => merkle_root}) do
    with {:ok, merkle_root} <- Traces.create_task_trace(merkle_root) do
      conn
      |> put_status(:created)
      |> render(:show, merkle_root: merkle_root)
    end
  end

  @doc """
  Finish a trace for the given merkle_root
  Method: POST finishTaskTrace
  """
  def finish_task_trace(conn, %{"merkle_root" => merkle_root}) do
    with :ok <- Traces.finish_task_trace(merkle_root) do
      conn
      |> put_status(:ok)
      |> render(:show, merkle_root: merkle_root)
    end
  end
end
