defmodule Explorer.Repo.Migrations.AddTokensTable do
  use Ecto.Migration

  def change do
    create table("strategies", primary_key: false) do
      add :id, :bigserial, primary_key: true
      add :strategy_address, :binary, null: false
      add :token_address, :binary, null: false
      add :name, :string, null: false
      add :symbol, :string, null: false
      add :total_staked, :decimal, precision: 22, scale: 0

      timestamps()
    end

    create unique_index("strategies", [:strategy_address], name: :strategy_address_index)
    create unique_index("strategies", [:token_address], name: :tokens_address_index)
  end
end
