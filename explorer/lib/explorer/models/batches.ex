defmodule Batches do
  require Logger
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
    field :fee_per_proof, :integer
    field :sender_address, :binary
    field :max_aggregator_fee, :decimal
    field :is_valid, :boolean, default: true

    timestamps()
  end

  @doc false
  def changeset(new_batch, updates) do
    new_batch
    |> cast(updates, [:merkle_root, :amount_of_proofs, :is_verified, :submission_block_number, :submission_transaction_hash, :submission_timestamp, :response_block_number, :response_transaction_hash, :response_timestamp, :data_pointer, :fee_per_proof, :sender_address, :max_aggregator_fee, :is_valid])
    |> validate_required([:merkle_root, :amount_of_proofs, :is_verified, :submission_block_number, :submission_transaction_hash, :fee_per_proof, :sender_address, :is_valid])
    |> validate_format(:merkle_root, ~r/0x[a-fA-F0-9]{64}/)
    |> unique_constraint(:merkle_root)
    |> validate_number(:amount_of_proofs, greater_than: 0)
    |> validate_inclusion(:is_verified, [true, false])
    |> validate_number(:submission_block_number, greater_than: 0)
    |> validate_format(:submission_transaction_hash, ~r/0x[a-fA-F0-9]{64}/)
    |> validate_number(:response_block_number, greater_than: 0)
    |> validate_format(:response_transaction_hash, ~r/0x[a-fA-F0-9]{64}/)
    |> validate_number(:max_aggregator_fee, greater_than: 0)
    |> validate_number(:fee_per_proof, greater_than_or_equal_to: 0)
    |> validate_inclusion(:is_valid, [true, false])
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
      fee_per_proof: batch_db.fee_per_proof,
      sender_address: batch_db.sender_address,
      max_aggregator_fee: batch_db.max_aggregator_fee,
      is_valid: batch_db.is_valid
    }
  end

  # returns changeset for both Batches and Proofs table
  def generate_changesets(%BatchDB{} = batch_db) do
    batches_changeset = Batches.changeset(%Batches{}, Map.from_struct(Batches.cast_to_batches(batch_db)))
    proofs = Proofs.cast_to_proofs(batch_db)

    {batches_changeset, proofs}
  end

  def get_batch(%{merkle_root: merkle_root}) do
    query = from(b in Batches,
    where: b.merkle_root == ^merkle_root,
    select: b)

    Explorer.Repo.one(query)
  end

  def get_latest_verified_batch() do
    query = from(b in Batches,
      order_by: [desc: b.submission_block_number],
      limit: 1,
      where: b.is_verified,
      select: b)

    Explorer.Repo.one(query)
  end

  def get_latest_batches(%{amount: amount} = %{order_by: :desc}) do
    query = from(b in Batches,
      order_by: [desc: b.submission_block_number],
      limit: ^amount,
      select: b)

    Explorer.Repo.all(query)
  end 
  
  def get_latest_batches(%{amount: amount} = %{order_by: :asc}) do
    query = from(b in Batches,
      order_by: [asc: b.submission_block_number],
      limit: ^amount,
      select: b)

    Explorer.Repo.all(query)
  end
  
  def get_paginated_batches(%{page: page, page_size: page_size}) do
    query =
      from(b in Batches,
        order_by: [desc: b.submission_block_number],
        limit: ^page_size,
        offset: ^((page - 1) * page_size),
        select: b
      )

    Explorer.Repo.all(query)
  end

  def get_last_page(page_size) do
    total_batches = Explorer.Repo.aggregate(Batches, :count, :merkle_root)
    last_page = div(total_batches, page_size)
    if rem(total_batches, page_size) > 0, do: last_page + 1, else: last_page
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
    where: b.is_valid == true and b.is_verified == false and b.submission_timestamp > ^threshold_datetime,
    select: b)

    Explorer.Repo.all(query)
  end

  def get_amount_of_submitted_proofs() do
    case Explorer.Repo.aggregate(Batches, :sum, :amount_of_proofs) do
      nil -> 0
      result -> result
    end
  end
  
  def get_avg_fee_per_proof() do
    query =
      from(b in Batches,
        where: b.is_verified == true,
        select: sum(b.fee_per_proof * b.amount_of_proofs) / sum(b.amount_of_proofs)
      )

    case Explorer.Repo.one(query) do
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

  # The following query was built by reading:
  # - https://hexdocs.pm/ecto/Ecto.Query.html#module-fragments
  # - https://www.postgresql.org/docs/9.1/functions-datetime.html#FUNCTIONS-DATETIME-TABLE
  # - https://stackoverflow.com/questions/43288914/how-to-use-date-trunc-with-timezone-in-ecto-fragment-statement
  # - https://glennjon.es/2016/09/24/elixir-ecto-how-to-group-and-count-records-by-week.html
  def get_daily_verified_batches_summary() do
    submission_constant_cost = Utils.constant_batch_submission_gas_cost()
    additional_cost_per_proof = Utils.additional_batch_submission_gas_cost_per_proof()

    query = Batches
      |> where([b], b.is_verified == true)
      |> group_by([b], fragment("DATE_TRUNC('day', ?)", b.response_timestamp))
      |> select([b], %{
        date: fragment("DATE_TRUNC('day', ?)::date", b.response_timestamp),
        amount_of_proofs: sum(b.amount_of_proofs),
        gas_cost: sum(fragment("? + ? * ?", ^submission_constant_cost, ^additional_cost_per_proof, b.amount_of_proofs))
      })
      |> order_by([b], fragment("DATE_TRUNC('day', ?)", b.response_timestamp))

    Explorer.Repo.all(query)
  end

  def insert_or_update(batch_changeset, proofs) do
    merkle_root = batch_changeset.changes.merkle_root
    stored_proofs = Proofs.get_proofs_from_batch(%{merkle_root: merkle_root})
    case Explorer.Repo.get(Batches, merkle_root) do
      nil ->
        multi = Ecto.Multi.new()
          |> Ecto.Multi.insert(:insert_batch, batch_changeset)
          |> Ecto.Multi.insert_all(:insert_all, Proofs, proofs)

          case Explorer.Repo.transaction(multi) do
            {:ok, _} ->
              Logger.debug("Batch inserted successfully")
              {:ok, :success}

            {:error, _failed_operation, failed_changeset, _reason} ->
              Logger.error("Error inserting batch: #{inspect(failed_changeset.errors)}")
              {:error, failed_changeset}
          end

      existing_batch ->
        try do
          if existing_batch.is_verified != batch_changeset.changes.is_verified
            or existing_batch.amount_of_proofs != batch_changeset.changes.amount_of_proofs  # rewrites if it was writen with DB's default
            or existing_batch.data_pointer != batch_changeset.changes.data_pointer          # rewrites if it was writen with DB's default
            or existing_batch.submission_block_number != batch_changeset.changes.submission_block_number          # reorg may change submission_block_number
            or existing_batch.submission_transaction_hash != batch_changeset.changes.submission_transaction_hash  # reorg may change submission_tx_hash
            or (Map.has_key?(batch_changeset.changes, :block_number)
              and  existing_batch.response_block_number != batch_changeset.changes.response_block_number)         # reorg may change response_block_number
            or (Map.has_key?(batch_changeset.changes, :response_transaction_hash)
              and existing_batch.response_transaction_hash != batch_changeset.changes.response_transaction_hash)  # reorg may change response_tx_hash
            or stored_proofs == nil and proofs != %{}                 # no proofs registered in DB, but some received
          do
            "Batch values have changed, updating in DB" |> Logger.debug()
            updated_changeset = Ecto.Changeset.change(existing_batch, batch_changeset.changes) # no changes in proofs table

            multi =
              Ecto.Multi.new()
              |> Ecto.Multi.update(:update_batch, updated_changeset)
              |> (fn m -> if stored_proofs == nil and proofs != %{}, do: Ecto.Multi.insert_all(m, :insert_proofs, Proofs, proofs), else: m end).()

            case Explorer.Repo.transaction(multi) do
              {:ok, _} ->
                "Batch updated and new proofs inserted successfully" |> Logger.debug()
                {:ok, :empty}
              {:error, _, changeset, _} ->
                "Error: #{inspect(changeset.errors)}" |> Logger.error()
                {:error, changeset}
            end

          end
        rescue
          error ->
            "Error updating batch in DB: #{inspect(error)}" |> Logger.alert()
        end
    end
  end
end
