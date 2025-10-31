defmodule AggregationModeProof do
  require Logger
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  # Different from proofs.ex (we could use the same but the hashes are constructed different)
  @primary_key {:id, :id, autogenerate: true}
  schema "proofs_agg_mode" do
    field(:agg_proof_id, :binary_id)
    field(:proof_hash, :string)
    field(:index, :integer)

    belongs_to(:aggregated_proof, AggregatedProof,
      define_field: false,
      foreign_key: :agg_proof_id,
      references: :id,
      type: :binary_id
    )

    timestamps()
  end

  def changeset(proof, attrs) do
    proof
    |> cast(attrs, [:agg_proof_id, :proof_hash, :index])
    |> validate_required([:agg_proof_id, :proof_hash, :index])
  end

  def insert_or_update(proof) do
    changeset =
      AggregationModeProof.changeset(%AggregationModeProof{}, proof)

    case(
      Explorer.Repo.get_by(AggregationModeProof,
        agg_proof_id: proof.agg_proof_id,
        proof_hash: proof.proof_hash,
        index: proof.index
      )
    ) do
      nil ->
        Explorer.Repo.insert(changeset)

      existing_proof ->
        "Updating single aggregated proof" |> Logger.debug()

        Ecto.Changeset.change(existing_proof, changeset.changes)
        |> Explorer.Repo.update()
    end
  end

  def get_all_proof_hashes(id) do
    query =
      from(proof in AggregationModeProof,
        select: proof.proof_hash,
        where: proof.agg_proof_id == ^id
      )

    Explorer.Repo.all(query)
  end

  def get_newest_proof_by_hash(hash) do
    query =
      from(proof in AggregationModeProof,
        select: proof,
        where: proof.proof_hash == ^hash,
        order_by: [desc: proof.inserted_at],
        limit: 1
      )

    Explorer.Repo.one(query)
  end
end
