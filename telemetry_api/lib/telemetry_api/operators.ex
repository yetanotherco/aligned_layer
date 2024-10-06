defmodule TelemetryApi.Operators do
  @moduledoc """
  The Operators context.
  """

  import Ecto.Query, warn: false
  alias TelemetryApi.Repo

  alias TelemetryApi.Operators.Operator
  alias TelemetryApi.ContractManagers.OperatorStateRetriever
  alias TelemetryApi.ContractManagers.DelegationManager

  @active 1

  @doc """
  Returns the list of operators.

  ## Examples

      iex> list_operators()
      [%Operator{}, ...]

  """
  def list_operators do
    Repo.all(Operator)
  end

  @doc """
  Gets a single operator.

  ## Examples

      iex> get_operator("some_address"})
      {:ok, %Operator{}}

      iex> get_operator("non_existent_address")
      {:error, :not_found, "Operator not found for address: non_existent_address"}
  """
  def get_operator(address) do
    case Repo.get(Operator, address) do
      nil ->
        IO.inspect("Operator not found for address: #{address}")
        {:error, :not_found, "Operator not found for address: #{address}"}

      operator ->
        {:ok, operator}
    end
  end

  @doc """
  Get a single operator by operator id.

  ## Examples

      iex> get_operator_by_id("some_id")
      {:ok, %Operator{}}

      iex> get_operator_by_id("non_existent_id")
      {:error, :not_found, "Operator not found for id: non_existent_id"}
  """
  def get_operator_by_id(id) do
    query = from(o in Operator, where: o.id == ^id)

    case Repo.one(query) do
      nil -> {:error, :not_found, "Operator not found for id: {id}"}
      operator -> {:ok, operator}
    end
  end

  @doc """
  - Fetches the state of all operators from the RegistryCoordinator ({address, id, stake}).
  - Fetches the metadata of all operators from the DelegationManager.
  - Stores all data in the database.

  ## Examples

      iex> fetch_all_operators()
      {:ok, %Ecto.Changeset{}}

      iex> fetch_all_operators()
      {:error, string}

  """
  def fetch_all_operators() do
    with {:ok, operators} <- OperatorStateRetriever.get_operators() do
      Enum.map(operators, fn op_data ->
        with {:ok, full_operator_data} <- add_operator_metadata(op_data) do
          case Repo.get(Operator, op_data.address) do
            nil -> %Operator{}
            operator -> operator
          end
          |> Operator.changeset(full_operator_data)
          |> Repo.insert_or_update()
        end
      end)
      |> TelemetryApi.Utils.check_list_status("Error fetching operators metadata")
    end
  end

  # Adds operator metadata to received operator.

  ### Examples

  #    iex> add_operator_metadata(operator)
  #    {:ok, operator_with_metadata}
  #
  #    iex> add_operator_metadata(operator)
  #    {:error, string}
  #
  defp add_operator_metadata(op_data) do
    with {:ok, url} <- DelegationManager.get_operator_url(op_data.address),
         {:ok, metadata} <- TelemetryApi.Utils.fetch_json_data(url) do
      operator = %{
        id: op_data.id,
        address: op_data.address,
        stake: op_data.stake,
        name: Map.get(metadata, "name")
      }

      {:ok, operator}
    end
  end

  @doc """
  Updates an operator's version.

  ## Examples

      iex> update_operator_version(%{field: value})
      {:ok, %Ecto.Changeset{}}

      iex> update_operator_version(%{field: bad_value})
      {:error, "Some status", "Some message"}

  """
  def update_operator_version(%{"version" => version, "signature" => signature}) do
    with {:ok, address} <- SignatureVerifier.recover_address(version, signature) do
      address = "0x" <> address
      # We only want to allow changes on version
      changes = %{
        version: version
      }

      case Repo.get(Operator, address) do
        nil ->
          {:error, :bad_request,
           "Provided address does not correspond to any registered operator"}

        operator ->
          operator |> Operator.changeset(changes) |> Repo.insert_or_update()
      end
    end
  end

  @doc """
  Updates a operator.

  ## Examples

      iex> update_operator(operator, %{field: new_value})
      {:ok, %Operator{}}

      iex> update_operator(operator, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_operator(%Operator{} = operator, attrs) do
    operator
    |> Operator.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a operator.

  ## Examples

      iex> delete_operator(operator)
      {:ok, %Operator{}}

      iex> delete_operator(operator)
      {:error, %Ecto.Changeset{}}

  """
  def delete_operator(%Operator{} = operator) do
    Repo.delete(operator)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking operator changes.

  ## Examples

      iex> change_operator(operator)
      %Ecto.Changeset{data: %Operator{}}

  """
  def change_operator(%Operator{} = operator, attrs \\ %{}) do
    Operator.changeset(operator, attrs)
  end

  @doc """
  Checks if an operator is active.

  ## Examples

      iex> is_active?(%Operator{status: 1})
      true

      iex> is_active?(%Operator{status: 0})
      false

  """
  def is_active?(operator) do
    operator.status == @active
  end
end
