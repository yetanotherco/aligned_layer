defmodule TelemetryApi.Operators do
  @moduledoc """
  The Operators context.
  """

  import Ecto.Query, warn: false
  alias TelemetryApi.Repo

  alias TelemetryApi.Operators.Operator
  alias TelemetryApi.ContractManagers.OperatorStateRetriever
  alias TelemetryApi.ContractManagers.DelegationManager

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
  Gets a single operator by id or address.

  ## Examples

      iex> get_operator(%{id: some_id})
      {:ok, %Operator{}}

      iex> get_operator(%{address: some_address})
      {:ok, %Operator{}}

      iex> get_operator(%{address: non_existent_address})
      {:error, :not_found, "Operator not found for address: non_existent_address"}
  """
  def get_operator(%{address: address}) do
    case Repo.get(Operator, address) do
      nil ->
        IO.inspect("Operator not found for address: #{address}")
        {:error, :not_found, "Operator not found for address: #{address}"}

      operator ->
        {:ok, operator}
    end
  end

  def get_operator(%{id: id}) do
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
      # Construct tuple {%Operator{}, op_data}
      operators = Enum.map(operators, fn op_data ->
        {Repo.get(Operator, op_data.address), op_data}
      end)

      # Filter operators already stored on db and those that are new
      #TODO: We actually don't need to add the %Operator{} here, we could do it just before the merge
      new_operators = Enum.filter(operators, fn {op, _} -> is_nil(op) end)
        |> Enum.map(fn {_, data} -> {%Operator{}, data} end)
      old_operators = Enum.filter(operators, fn {op, _} -> not is_nil(op) end)

      # Fetch metadata for new operators
      new_operators = Enum.map(new_operators, fn {op, op_data} ->
        case add_operator_metadata(op_data) do
          {:ok, data} -> {:ok, {op, data}}
          {:error, msg} -> {:error, msg}
        end
      end)
      # Filter status ok and map to {op, op_data}
        |> Enum.filter(fn {status, _} -> status == :ok end)
        |> Enum.map(fn {_, data} -> data end)

      # Merge both lists
      operators = (new_operators ++ old_operators)

      # Insert in db
      Enum.map(operators, fn {op, op_data} ->
        Operator.changeset(op, op_data) |> Repo.insert_or_update()
      end)
      end
    :ok
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
  Updates an operator.

  ## Examples

      iex> update_operator(some_version, some_signature, %{field: value})
      {:ok, %Ecto.Changeset{}}

      iex> update_operator(some_version, invalid_signature, %{field:  value})
      {:error, "Some status", "Some message"}

  """
  def update_operator(version, signature, changes) do
    with {:ok, address} <- SignatureVerifier.recover_address(version, signature) do
      address = "0x" <> address
      case Repo.get(Operator, address) do
        nil ->
          {:error, :bad_request,
           "Provided address does not correspond to any registered operator"}

        operator ->
          update_operator(operator, changes)
      end
    end
  end

  @doc """
  Updates an operator.

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
  Checks if an operator is registered.

  ## Examples

      iex> is_registered?(%Operator{status: "REGISTERED"})
      true

      iex> is_registered?(%Operator{status: "DEREGISTERED"})
      false

  """
  def is_registered?(operator) do
    operator.status == "REGISTERED"
  end
end
