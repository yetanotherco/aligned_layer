defmodule ExplorerWeb.Plugs do
  import Plug.Conn

  def init(default), do: default

  def load_theme_cookie_in_session(conn, _opts) do
    # Default to dark if not present
    theme = Map.get(conn.cookies, "theme", "dark")

    conn
    |> put_session(:theme, theme)
  end
end
