defmodule Tokens do
  use Ecto.Schema
  import Ecto.Changeset
  # import Ecto.Query

  schema "tokens" do
    field :name, :string
    field :symbol, :string
    field :address, :binary
    field :total_staked, :decimal

    timestamps()
  end

  @doc false
  def changeset(token, attrs) do
    token
    |> cast(attrs, [:name, :symbol, :address, :total_staked])
    |> validate_required([:name, :symbol, :address, :total_staked])
  end

  def add_token(%Tokens{name: name, symbol: symbol, address: address, total_staked: total_staked} = new_token) do
    new_token
    |> Tokens.changeset(%{name: name, symbol: symbol, address: address, total_staked: total_staked})
    |> Explorer.Repo.insert()
  end

end
