defmodule AggregatedProofs do
  require Logger
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  # Different from proofs.ex (we could use the same but the hashes are constructed differently)
  @primary_key {:id, :integer, autogenerate: true}
  schema "proofs_agg_mode" do
    field(:aggregated_proof_number, :integer)
    field(:proof_hash, :string)
  end

  def changeset_proof(proof, attrs) do
    proofs
    |> cast(attrs, [:aggregated_proof_number, :proof_hash])
    |> validate_required([:aggregated_proof_number, :proof_hash])
  end

  @primary_key {:number, :integer, autogenerate: false}
  schema "aggregated_proofs" do
    field(:merkle_root, :string)
    field(:status, :integer)
    field(:tx_hash, :string)
    field(:blob_versioned_hash, :string)
    field(:blob_data, :binary)
    field(:number_of_proofs, :integer)
    field(:block_number, :integer)
    field(:tx_timestamp, :utc_datetime)

    timestamps()
  end

  @doc """
  Creates a changeset based on the given `attrs`.
  """
  def changeset(aggregated_proof, attrs) do
    aggregated_proof
    |> cast(attrs, [
      :number,
      :merkle_root,
      :status,
      :tx_hash,
      :blob_versioned_hash,
      :blob_data,
      :number_of_proofs,
      :block_number,
      :tx_timestamp
    ])
    |> validate_required([
      :number,
      :merkle_root,
      :status,
      :tx_hash,
      :blob_versioned_hash,
      :number_of_proofs,
      :block_number,
      :tx_timestamp
    ])
    |> unique_constraint(:number)
  end

  def insert_or_update(agg_proof) do
    changeset = AggregatedProofs.changeset(%AggregatedProofs{}, Map.from_struct(agg_proof))

    case Explorer.Repo.get_by(AggregatedProofs, number: agg_proof.number) do
      nil ->
        Explorer.Repo.insert(changeset)

      existing_agg_proof ->
        "Updating aggregated proof" |> Logger.debug()

        Ecto.Changeset.change(existing_agg_proof, changeset.changes)
        |> Explorer.Repo.update()
    end
  end

  def insert_proof(proof) do
    changeset = changeset_proof(proof)
    Explorer.Repo.insert(changeset)
  end
end
