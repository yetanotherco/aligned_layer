defmodule AggregatedProofs do
  require Logger
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  @primary_key {:number, :integer, autogenerate: false}
  schema "aggregated_proofs" do
    field(:merkle_root, :string)
    field(:status, :integer)
    field(:tx_hash, :string)
    field(:blob_versioned_hash, :string)
    field(:number_of_proofs, :integer)
    field(:block_number, :integer)

    has_many(:proofs_agg_mode, AggregationModeProof)

    timestamps()
  end

  @doc """
  Creates a changeset based on the given `attrs`.
  """
  def changeset(aggregated_proof, attrs) do
    aggregated_proof
    |> cast(attrs, [
      :number,
      :status,
      :merkle_root,
      :blob_versioned_hash,
      :block_number,
      :tx_hash,
      :number_of_proofs
    ])
    |> validate_required([
      :number,
      :status,
      :merkle_root,
      :blob_versioned_hash,
      :block_number,
      :tx_hash,
      :number_of_proofs
    ])
    |> unique_constraint(:number)
  end

  def insert_or_update(agg_proof) do
    changeset = AggregatedProofs.changeset(%AggregatedProofs{}, agg_proof)

    case Explorer.Repo.get_by(AggregatedProofs, number: agg_proof.number) do
      nil ->
        Explorer.Repo.insert(changeset)

      existing_agg_proof ->
        "Updating aggregated proof" |> Logger.debug()

        Ecto.Changeset.change(existing_agg_proof, changeset.changes)
        |> Explorer.Repo.update()
    end
  end
end
