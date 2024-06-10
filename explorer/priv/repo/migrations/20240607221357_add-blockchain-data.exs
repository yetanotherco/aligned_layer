defmodule :"Elixir.Explorer.Repo.Migrations.Add-blockchain-data" do
  use Ecto.Migration

  def change do
    alter table("batches") do # add new columns
      add :submition_block_number, :integer, null: false, default: 0
      add :submition_transaction_hash, :string, size: 66, null: false, default: ""
      add :submition_timestamp, :utc_datetime
      add :response_block_number, :integer
      add :response_transaction_hash, :string, size: 66
      add :response_timestamp, :utc_datetime
      add :data_pointer, :string, size: 255
    end
  end
end
