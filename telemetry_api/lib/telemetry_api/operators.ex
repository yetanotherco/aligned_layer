defmodule TelemetryApi.Operators do
  @moduledoc """
  The Operators context.
  """

  import Ecto.Query, warn: false
  alias TelemetryApi.Repo

  alias TelemetryApi.Operators.Operator

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
  Gets a single operator by id.

  Raises `Ecto.NoResultsError` if the Operator does not exist.

  ## Examples

      iex> get_operator_by_id!(123)
      %Operator{}

      iex> get_operator_by_id!(456)
      ** (Ecto.NoResultsError)

  """
  def get_operator_by_id!(id), do: Repo.get!(Operator, id)

  @doc """
  Gets an operator by address.

  ## Examples

      iex> get_operator(%{field: value})
      %Operator{}

      iex> get_operator(%{field: value})
      nil

  """
  def get_operator(attrs \\ %{}) do
    address = SignatureVerifier.get_address(attrs["version"], attrs["signature"])
    query = from o in Operator, where: o.address == ^address
    Repo.one(query)
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
    address = SignatureVerifier.get_address(attrs["version"], attrs["signature"])
    attrs = Map.put(attrs, "address", address)

    %Operator{}
    |> Operator.changeset(attrs)
    |> Repo.insert()
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
