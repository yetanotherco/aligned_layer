defmodule TelemetryApi.Operators do
  @moduledoc """
  The Operators context.
  """

  import Ecto.Query, warn: false
  alias TelemetryApi.Repo

  alias TelemetryApi.Operators.Operator
  alias TelemetryApi.ContractManagers.RegistryCoordinatorManager
  alias TelemetryApi.ContractManagers.OperatorStateRetriever

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
      %Operator{}

      iex> get_operator("non_existent_address")
      nil

  """
  def get_operator(address) do
    Repo.get(Operator, address)
  end

  @doc """
  Fetches all operators.

  ## Examples

      iex> fetch_all_operators()
      {:ok, %Ecto.Changeset{}}

      iex> fetch_all_operators()
      {:error, string}

  """
  def fetch_all_operators() do
    with {:ok, operators} <- OperatorStateRetriever.get_operators() do
        operators = Enum.map(operators, fn op_data -> 
          case Repo.get(Operator, op_data.address) do
            nil -> %Operator{} 
            operator -> operator
          end
          |> Operator.changeset(op_data) 
          |> Repo.insert_or_update()
        end)
        # Check if we failed to store any operator
        case Enum.find(operators, fn {status, _} -> status == :error end) do
          nil -> 
            {:ok, Enum.map(operators, fn {:ok, value} -> value end)}
          
          {:error, _} -> 
            {:error, "Failed to store Operator in database"}
        end
    end
  end
  
  @doc """
  Updates an operator's version.

  ## Examples

      iex> update_operator_version(%{field: value})
      {:ok, %Ecto.Changeset{}}

      iex> update_operator_version(%{field: bad_value})
      {:error, string}

  """
  def update_operator_version(attrs \\ %{}) do
    with {:ok, address} <- SignatureVerifier.get_address(attrs["version"], attrs["signature"]) do
        address = "0x" <> address 
        # We only want to allow changes on version
        changes = %{
          version: attrs["version"]
        }
        case Repo.get(Operator, address) do
          nil -> {:error, "Provided address does not correspond to any registered operator"}
          operator -> operator |> Operator.changeset(changes) |> Repo.insert_or_update()
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
end
