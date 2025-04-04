defmodule AggregationModeProof do
  require Logger
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  # Different from proofs.ex (we could use the same but the hashes are constructed different)
  @primary_key {:id, :id, autogenerate: true}
  schema "proofs_agg_mode" do
    field(:aggregated_proof_number, :integer)
    field(:proof_hash, :string)
  end

  def changeset(proof, attrs) do
    proof
    |> cast(attrs, [:aggregated_proof_number, :proof_hash])
    |> validate_required([:aggregated_proof_number, :proof_hash])
  end

  def insert_proof(proof) do
    changeset =
      AggregationModeProof.changeset(%AggregationModeProof{}, Map.from_struct(proof))

    Explorer.Repo.insert(changeset)
  end
end
