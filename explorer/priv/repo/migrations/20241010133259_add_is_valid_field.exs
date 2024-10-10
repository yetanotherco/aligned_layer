defmodule Explorer.Repo.Migrations.AddIsValidField do
  use Ecto.Migration

  def change do
    alter table("batches") do
      add :is_valid, :boolean, default: true
    end
  end
end
