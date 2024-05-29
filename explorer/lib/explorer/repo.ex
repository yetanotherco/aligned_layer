defmodule Explorer.Repo do

  use Ecto.Repo,
    otp_app: :explorer,
    adapter: Ecto.Adapters.Postgres

end
