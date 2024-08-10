defmodule Quorums do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  schema "quorums" do
    many_to_many :strategies, Strategies, join_through: "quorum_strategies"
    timestamps()
  end

  def process_quorum_changes() do
    our_quorums = get_all_quorums()

    Enum.each(our_quorums, &handle_quorum/1)
  end

  def handle_quorum(%Quorums{} = quorum) do
    strategy_addresses = StakeRegistryManager.get_strategies_of_quorum(quorum.id)

    insert_quorum_if_not_present(quorum)

    Enum.each(strategy_addresses,
      fn strategy_address ->
        QuorumStrategies.insert_quorum_strategy(
          quorum,
          Strategies.get_by_strategy_address(strategy_address)
        )
      end)

    quorum
  end

  def insert_quorum_if_not_present(%Quorums{} = quorum) do
    case get_quorum_by_id(quorum.id) do
      nil ->
        dbg "inserting quorum"
        Explorer.Repo.insert(quorum)
      _ ->
        nil
    end
  end

  def get_quorum_by_id(quorum_id) do
    query = from(q in Quorums,
      where: q.id == ^quorum_id,
      select: q)
    Explorer.Repo.one(query)
  end

  def get_all_quorums() do
    query = from(q in Quorums,
      select: q)
    Explorer.Repo.all(query)
  end
end

defmodule QuorumStrategies do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  schema "quorum_strategies" do
    belongs_to :quorum, Quorum
    belongs_to :strategy, Strategies
    timestamps()
  end

  def changeset(quorum_strategy, attrs) do
    quorum_strategy
    |> cast(attrs, [:quorum_id, :strategy_id])
    |> validate_required([:quorum_id, :strategy_id])
    |> unique_constraint([:quorum_id, :strategy_id])
  end

  def generate_changeset(quorum_id, strategy_id) do
    QuorumStrategies.changeset(%QuorumStrategies{}, Map.from_struct(%QuorumStrategies{quorum_id: quorum_id, strategy_id: strategy_id})) |> tap(&dbg/1)
  end


  def get_quorum_strategy_associations(%Quorums{} = quorum) do
    query = from(qs in "quorum_strategies",
      where: qs.quorum_id == ^quorum.id,
      select: qs.strategy_id)
    Explorer.Repo.all(query)
  end

  def insert_quorum_strategy(%Quorums{} = quorum, %Strategies{} = strategy) do
    existing_strategies = QuorumStrategies.get_quorum_strategy_associations(quorum)

    unless strategy.id in existing_strategies do
      QuorumStrategies.generate_changeset(quorum.id, strategy.id) |> Explorer.Repo.insert() |> dbg
    end
  end
  def insert_quorum_strategy(_any, nil) do
    dbg "trying to insert a nil or errored strategy, skipping"
    nil
  end
end
