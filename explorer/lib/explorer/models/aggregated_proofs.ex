defmodule AggregatedProofs do
  require Logger
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:merkle_root, :string, autogenerate: false}
  schema "aggregated_proofs" do
    field(:blob_versioned_hash, :string)
    field(:block_number, :integer)
    field(:tx_hash, :string)
    field(:number_of_proofs, :integer)

    has_many(:proofs_agg_mode, AggregationModeProof,
      foreign_key: :merkle_root,
      references: :merkle_root
    )

    timestamps()
  end

  @doc """
  Creates a changeset based on the given `attrs`.
  """
  def changeset(aggregated_proof, attrs) do
    aggregated_proof
    |> cast(attrs, [
      :merkle_root,
      :blob_versioned_hash,
      :block_number,
      :tx_hash,
      :number_of_proofs
    ])
    |> validate_required([
      :merkle_root,
      :blob_versioned_hash,
      :block_number,
      :tx_hash,
      :number_of_proofs
    ])
    |> unique_constraint(:merkle_root)
  end

  def insert_or_update(agg_proof) do
    changeset = AggregatedProofs.changeset(%AggregatedProofs{}, agg_proof)

    case Explorer.Repo.get_by(AggregatedProofs, merkle_root: agg_proof.merkle_root) do
      nil ->
        Explorer.Repo.insert(changeset)

      existing_agg_proof ->
        "Updating aggregated proof" |> Logger.debug()

        Ecto.Changeset.change(existing_agg_proof, changeset.changes)
        |> Explorer.Repo.update()
    end
  end
end
