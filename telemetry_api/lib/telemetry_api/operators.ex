defmodule TelemetryApi.Operators do
  @moduledoc """
  The Operators context.
  """

  import Ecto.Query, warn: false
  alias TelemetryApi.Repo

  alias TelemetryApi.Operators.Operator
  alias TelemetryApi.RegistryCoordinatorManager

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
  Creates a operator.

  ## Examples

      iex> create_operator(%{field: value})
      {:ok, %Operator{}}

      iex> create_operator(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_operator(attrs \\ %{}) do
    # Get address from the signature
    with {:ok, address} <- SignatureVerifier.get_address(attrs["version"], attrs["signature"]),
      {:ok, is_registered?} <- RegistryCoordinatorManager.is_operator_registered?(address) do
        # Verify operator is registered
        if is_registered? do
          address = "0x" <> address 
          attrs = Map.put(attrs, "address", address)

          # We handle updates here as there is no patch method available at the moment.
          case Repo.get(Operator, address) do
            nil -> %Operator{}
            operator -> operator
          end
          |> Operator.changeset(attrs)
          |> Repo.insert_or_update()
        else
          {:error, "Provided address does not correspond to any registered operator"}
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
