defmodule AggregatedProofs do
  require Logger
  use Ecto.Schema
  import Ecto.Query
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  schema "aggregated_proofs" do
    field(:merkle_root, :string)
    field(:blob_versioned_hash, :string)
    field(:block_number, :integer)
    field(:block_timestamp, :utc_datetime)
    field(:tx_hash, :string)
    field(:number_of_proofs, :integer)

    has_many(:proofs_agg_mode, AggregationModeProof,
      foreign_key: :agg_proof_id,
      references: :id
    )

    timestamps()
  end

  @doc """
  Creates a changeset based on the given `attrs`.
  """
  def changeset(aggregated_proof, attrs) do
    aggregated_proof
    |> cast(attrs, [
      :id,
      :merkle_root,
      :blob_versioned_hash,
      :block_number,
      :block_timestamp,
      :tx_hash,
      :number_of_proofs
    ])
    |> validate_required([
      :merkle_root,
      :blob_versioned_hash,
      :block_number,
      :block_timestamp,
      :tx_hash,
      :number_of_proofs
    ])
    |> unique_constraint(:id)
  end

  def insert_or_update(agg_proof) do
    changeset = AggregatedProofs.changeset(%AggregatedProofs{}, agg_proof)

    case Explorer.Repo.get_by(AggregatedProofs, block_number: agg_proof.block_number) do
      nil ->
        Explorer.Repo.insert(changeset)

      existing_agg_proof ->
        "Updating aggregated proof" |> Logger.debug()

        Ecto.Changeset.change(existing_agg_proof, changeset.changes)
        |> Explorer.Repo.update()
    end
  end

  def get_aggregated_proof_by_id(id) do
    Explorer.Repo.get_by(AggregatedProofs, id: id)
  end

  def get_paginated_proofs(%{page: page, page_size: size}) do
    query =
      from(proof in AggregatedProofs,
        order_by: [desc: proof.block_number],
        limit: ^size,
        offset: ^((page - 1) * size),
        select: proof
      )

    Explorer.Repo.all(query)
  end

  def get_last_page(page_size) do
    total_proofs = Explorer.Repo.aggregate(AggregatedProofs, :count, :id)
    last_page = div(total_proofs, page_size)
    if rem(total_proofs, page_size) > 0, do: last_page + 1, else: last_page
  end

  def get_number_of_agg_proofs() do
    Explorer.Repo.aggregate(AggregatedProofs, :count, :id)
  end
end
