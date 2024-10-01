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
      |> TelemetryApi.Utils.clean_list_errors("Error fetching operators metadata")
    end
  end

  
  #Adds operator metadata to received operator.

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

      iex> update_operator(%{field: value})
      {:ok, %Ecto.Changeset{}}

      iex> update_operator(%{field: bad_value})
      {:error, string}

  """
  def update_operator(attrs) do
    with {:ok, address} <- SignatureVerifier.get_address(attrs["version"], attrs["signature"]) do
      address = "0x" <> address
      case Repo.get(Operator, address) do
        nil -> {:error, "Provided address does not correspond to any registered operator"}
        operator -> operator |> Operator.changeset(attrs) |> Repo.insert_or_update()
      end
    end
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
