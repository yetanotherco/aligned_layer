defmodule Batches do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  @primary_key {:merkle_root, :string, autogenerate: false}
  schema "batches" do
    field :amount_of_proofs, :integer
    field :is_verified, :boolean
    field :submission_block_number, :integer
    field :submission_transaction_hash, :string
    field :submission_timestamp, :utc_datetime
    field :response_block_number, :integer
    field :response_transaction_hash, :string
    field :response_timestamp, :utc_datetime
    field :data_pointer, :string
    field :proof_hashes, {:array, :string}

    timestamps()
  end

  @doc false
  def changeset(new_batch, updates) do
    new_batch
    |> cast(updates, [:merkle_root, :amount_of_proofs, :is_verified, :submission_block_number, :submission_transaction_hash, :submission_timestamp, :response_block_number, :response_transaction_hash, :response_timestamp, :data_pointer, :proof_hashes])
    |> validate_required([:merkle_root, :amount_of_proofs, :is_verified, :submission_block_number, :submission_transaction_hash, :proof_hashes])
    |> validate_format(:merkle_root, ~r/0x[a-fA-F0-9]{64}/)
    |> unique_constraint(:merkle_root)
    |> validate_number(:amount_of_proofs, greater_than: 0)
    |> validate_inclusion(:is_verified, [true, false])
    |> validate_number(:submission_block_number, greater_than: 0)
    |> validate_format(:submission_transaction_hash, ~r/0x[a-fA-F0-9]{64}/)
    |> validate_number(:response_block_number, greater_than: 0)
    |> validate_format(:response_transaction_hash, ~r/0x[a-fA-F0-9]{64}/)
  end

  def cast_to_batches(%BatchDB{} = batch_db) do
    %Batches{
      merkle_root: batch_db.merkle_root,
      amount_of_proofs: batch_db.amount_of_proofs,
      is_verified: batch_db.is_verified,
      submission_block_number: batch_db.submission_block_number,
      submission_transaction_hash: batch_db.submission_transaction_hash,
      submission_timestamp: batch_db.submission_timestamp,
      response_block_number: batch_db.response_block_number,
      response_transaction_hash: batch_db.response_transaction_hash,
      response_timestamp: batch_db.response_timestamp,
      data_pointer: batch_db.data_pointer,
      proof_hashes: batch_db.proof_hashes
    }
  end

  def generate_changeset(%BatchDB{} = batch_db) do
    Batches.changeset(%Batches{}, Map.from_struct(Batches.cast_to_batches(batch_db)))
  end

  def get_batch(%{merkle_root: merkle_root}) do
    query = from(b in Batches,
    where: b.merkle_root == ^merkle_root,
    select: b)

    Explorer.Repo.one(query)
  end

  def get_latest_batches(%{amount: amount}) do
    query = from(b in Batches,
      order_by: [desc: b.submission_block_number],
      limit: ^amount,
      select: b)

    Explorer.Repo.all(query)
  end

  def get_paginated_batches(%{page: page, page_size: page_size}) do
    query = from(b in Batches,
      order_by: [desc: b.submission_block_number],
      limit: ^page_size,
      offset: ^((page - 1) * page_size),
      select: b)

    Explorer.Repo.all(query)
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

  def get_unverified_batches() do
    threshold_datetime = DateTime.utc_now() |> DateTime.add(-43200, :second) # 12 hours ago

    query = from(b in Batches,
    where: b.is_verified == false and b.submission_timestamp > ^threshold_datetime,
    select: b)

    Explorer.Repo.all(query)
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

  def get_proof_info(%{merkle_root: merkle_root}) do
    query = from(b in Batches,
      where: b.merkle_root == ^merkle_root,
      select: [b.amount_of_proofs, b.proof_hashes])

    case Explorer.Repo.one(query) do
      nil -> nil
      result -> result
    end
  end

  def insert_or_update(changeset) do
    merkle_root = changeset.changes.merkle_root
    case Explorer.Repo.get(Batches, merkle_root) do
      nil ->
        "New Batch, inserting to DB:" |> IO.puts()
        case Explorer.Repo.insert(changeset) do
          {:ok, _} ->
            "Batch inserted successfully" |> IO.puts()
            {:ok, :empty}

          {:error, changeset} ->
            "Batch insert failed #{changeset}" |> IO.puts()
            {:error, changeset}
        end
      existing_batch ->
        try do
          if existing_batch.is_verified != changeset.changes.is_verified
            or existing_batch.amount_of_proofs != changeset.changes.amount_of_proofs  # rewrites if it was writen with DB's default
            or existing_batch.data_pointer != changeset.changes.data_pointer          # rewrites if it was writen with DB's default
            or existing_batch.submission_block_number != changeset.changes.submission_block_number          # reorg may change submission_block_number
            or existing_batch.submission_transaction_hash != changeset.changes.submission_transaction_hash  # reorg may change submission_tx_hash
            or (Map.has_key?(changeset.changes, :block_number)
              and  existing_batch.response_block_number != changeset.changes.response_block_number)         # reorg may change response_block_number
            or (Map.has_key?(changeset.changes, :response_transaction_hash)
              and existing_batch.response_transaction_hash != changeset.changes.response_transaction_hash)  # reorg may change response_tx_hash
          do
            "Batch values have changed, updating in DB" |> IO.puts()
            updated_changeset = Ecto.Changeset.change(existing_batch, changeset.changes)
            case Explorer.Repo.update(updated_changeset) do
              {:ok, _} ->
                "Batch updated successfully" |> IO.puts()
                {:ok, :empty}

              {:error, changeset} ->
                "Batch update failed #{changeset}" |> IO.puts()
                {:error, changeset}
            end
          end
        rescue
          error ->
            IO.inspect("Error updating batch in DB: #{inspect(error)}")
            raise error
        end
    end
  end

end
