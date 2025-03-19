defmodule TelemetryApiWeb.TraceController do
  use TelemetryApiWeb, :controller

  alias TelemetryApi.Traces

  action_fallback(TelemetryApiWeb.FallbackController)

  @doc """
  Create a trace for a NewTask with the given merkle_root
  Method: POST initTaskTrace
  """
  def create_task_trace(conn, %{"merkle_root" => merkle_root}) do
    with :ok <- Traces.create_task_trace(merkle_root) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Register a batcher task sent to Ethereum in the trace of the given merkle_root
  Method: POST batcherTaskSent
  """
  def batcher_task_sent(conn, %{"merkle_root" => merkle_root, "tx_hash" => tx_hash}) do
    with :ok <- Traces.batcher_task_sent(merkle_root, tx_hash) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Register a batcher task uploaded to S3 in the trace of the given merkle_root
  Method: POST batcherTaskUploadedToS3
  """
  def batcher_task_uploaded_to_s3(conn, %{"merkle_root" => merkle_root}) do
    with :ok <- Traces.batcher_task_uploaded_to_s3(merkle_root) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Register a batcher task started in the trace of the given merkle_root
  Method: POST batcherTaskStarted
  """
  def batcher_task_started(conn, %{
        "merkle_root" => merkle_root,
        "fee_per_proof" => fee_per_proof,
        "num_proofs_in_batch" => total_proofs
      }) do
    with :ok <- Traces.batcher_task_started(merkle_root, fee_per_proof, total_proofs) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Create a trace for a new batcher task with the given merkle_root
  Method: POST initBatcherTaskTrace
  """
  def create_batcher_task_trace(conn, %{
        "merkle_root" => merkle_root
      }) do
    with :ok <- Traces.create_batcher_task_trace(merkle_root) do
      conn
      |> put_status(:created)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Register a batcher task creation error in the trace of the given merkle_root
  Method: POST batcherTaskCreationFailed
  """
  def batcher_task_creation_failed(conn, %{"merkle_root" => merkle_root, "error" => error}) do
    with :ok <- Traces.batcher_task_creation_failed(merkle_root, error) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Register an operator response in the trace of the given merkle_root
  Method: POST operatorResponse
  """
  def register_operator_response(conn, %{
        "merkle_root" => merkle_root,
        "operator_id" => operator_id
      }) do
    with :ok <- Traces.register_operator_response(merkle_root, operator_id) do
      conn
      |> put_status(:ok)
      |> render(:show_operator, operator_id: operator_id)
    end
  end

  @doc """
  Registers a reached quorum in the trace of the given merkle_root
  Method: POST quorumReached
  """
  def quorum_reached(conn, %{"merkle_root" => merkle_root}) do
    with :ok <- Traces.quorum_reached(merkle_root) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Registers an error in the trace of the given merkle_root
  Method: POST taskError
  """
  def task_error(conn, %{"merkle_root" => merkle_root, "error" => error}) do
    with :ok <- Traces.task_error(merkle_root, error) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Registers a gas price in the trace of the given merkle_root
  Method: POST aggregatorTaskSetGasPrice
  """
  def aggregator_task_set_gas_price(conn, %{
        "merkle_root" => merkle_root,
        "gas_price" => gas_price
      }) do
    with :ok <- Traces.aggregator_task_set_gas_price(merkle_root, gas_price) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end

  @doc """
  Register a task sent, from the aggregator, to Ethereum in the trace of the given merkle_root
  Method: POST aggregatorTaskSent
  """
  def aggregator_task_sent(conn, %{"merkle_root" => merkle_root, "tx_hash" => tx_hash, "effective_gas_price" => effective_gas_price}) do
    with :ok <- Traces.aggregator_task_sent(merkle_root, tx_hash, effective_gas_price) do
      conn
      |> put_status(:ok)
      |> render(:show_merkle, merkle_root: merkle_root)
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
      |> render(:show_merkle, merkle_root: merkle_root)
    end
  end
end
