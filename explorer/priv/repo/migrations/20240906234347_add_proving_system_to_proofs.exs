defmodule Explorer.Repo.Migrations.AddProvingSystemToProofs do
  use Ecto.Migration

  def change do
    alter table("proofs") do
      add :proving_system, :binary
    end
  end
end
