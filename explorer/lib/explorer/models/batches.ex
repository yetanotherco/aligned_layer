defmodule Batches do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:merkle_root, :string, autogenerate: false}
  schema "batches" do
    field :amount_of_proofs, :integer
    field :is_verified, :boolean

    # timestamps()
  end

  @doc false
  def changeset(new_batch, updates) do
    new_batch
    |> cast(updates, [:merkle_root, :amount_of_proofs, :is_verified])
    |> validate_required([:merkle_root, :amount_of_proofs, :is_verified])
    |> validate_number(:amount_of_proofs, greater_than: 0)
    |> validate_inclusion(:is_verified, [true, false])
    |> validate_format(:merkle_root, ~r/0x[a-fA-F0-9]{64}/)
    |> unique_constraint(:merkle_root)
  end
end
