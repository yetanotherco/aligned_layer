defmodule ExplorerWeb.Hooks do
  def on_mount(:add_host, _params, _session, socket) do
    %URI{host: host} = Phoenix.LiveView.get_connect_info(socket, :uri)

    socket = Phoenix.Component.assign(socket, host: host)

    {:cont, socket}
  end

  def on_mount(:add_theme, _params, session, socket) do
    socket = Phoenix.Component.assign(socket, theme: session["theme"])

    {:cont, socket}
  end
end
