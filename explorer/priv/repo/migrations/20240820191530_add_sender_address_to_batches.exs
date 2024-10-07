defmodule Explorer.Repo.Migrations.AddSenderAddressToBatches do
  use Ecto.Migration

  def change do
    alter table("batches") do
      add :sender_address, :binary
    end
  end
end
