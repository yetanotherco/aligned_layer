defmodule TelemetryApiWeb.Router do
  use TelemetryApiWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/api", TelemetryApiWeb do
    pipe_through :api

    get "/operators", OperatorController, :index
    get "/operators/:id", OperatorController, :show
    post "/operators", OperatorController, :create_or_update
    post "/initTaskTrace", TraceController, :create_task_trace
    post "/operatorResponse", TraceController, :register_operator_response
    post "/quorumReached", TraceController, :quorum_reached
    post "/taskError", TraceController, :task_error
    post "/aggregatorTaskSetGasPrice", TraceController, :aggregator_task_set_gas_price
    post "/aggregatorTaskSent", TraceController, :aggregator_task_sent
    post "/finishTaskTrace", TraceController, :finish_task_trace

    post "/initBatcherTaskTrace", TraceController, :create_batcher_task_trace
    post "/batcherTaskUploadedToS3", TraceController, :batcher_task_uploaded_to_s3
    post "/batcherTaskStarted", TraceController, :batcher_task_started
    post "/batcherTaskSent", TraceController, :batcher_task_sent
    post "/batcherTaskCreationFailed", TraceController, :batcher_task_creation_failed
  end

  scope "/versions", TelemetryApiWeb do
    pipe_through :api

    get "/", OperatorController, :index_public
    get "/:id", OperatorController, :show_public
    post "/", OperatorController, :create_or_update
  end

  # Enable LiveDashboard in development
  # if Application.compile_env(:telemetry_api, :dev_routes) do
  #   # If you want to use the LiveDashboard in production, you should put
  #   # it behind authentication and allow only admins to access it.
  #   # If your application does not have an admins-only section yet,
  #   # you can use Plug.BasicAuth to set up some basic authentication
  #   # as long as you are also using SSL (which you should anyway).
  #   import Phoenix.LiveDashboard.Router

  #   scope "/dev" do
  #     pipe_through [:fetch_session, :protect_from_forgery]

  #     live_dashboard "/dashboard", metrics: TelemetryApiWeb.Telemetry
  #   end
  # end
end
