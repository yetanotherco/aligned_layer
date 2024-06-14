defmodule Batches do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  @primary_key {:merkle_root, :string, autogenerate: false}
  schema "batches" do
    field :amount_of_proofs, :integer
    field :is_verified, :boolean

    timestamps()
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

  def cast_to_batches(%BatchDB{} = batch_db) do
    %Batches{
      merkle_root: batch_db.batch_merkle_root,
      amount_of_proofs: batch_db.amount_of_proofs,
      is_verified: batch_db.is_verified
    }
  end

  def get_amount_of_submitted_proofs() do
    case Explorer.Repo.aggregate(Batches, :sum, :amount_of_proofs) do
      nil -> 0
      result -> result
    end
  end

  def get_amount_of_verified_proofs() do
    query = from(b in Batches,
      where: b.is_verified == true,
      select: sum(b.amount_of_proofs))

    case Explorer.Repo.one(query) do
      nil -> 0
      result -> result
    end
  end

  def get_amount_of_verified_batches() do
    query = from(b in Batches,
      where: b.is_verified == true,
      select: count(b.merkle_root))

    case Explorer.Repo.one(query) do
      nil -> 0
      result -> result
    end
  end
end
