defmodule ReleasesHelper do
  require Logger

  def get_latest_release do
    case do_fetch_latest_release() do
      {:ok, tag} ->
        tag

      {:error, reason} ->
        Logger.error("Failed to fetch latest release: #{reason}")
        nil
    end
  end

  defp do_fetch_latest_release do
    with :ok <- fetch_tags(),
         {:ok, tag} <- get_latest_tag() do
      {:ok, tag}
    end
  end

  defp fetch_tags do
    case System.cmd("git", ["fetch", "--tags"]) do
      {_, 0} -> :ok
      {error, _} -> {:error, "Failed to fetch tags: #{error}"}
    end
  end

  defp get_latest_tag do
    case System.cmd("git", ["describe", "--tags", "--abbrev=0"]) do
      {tag, 0} -> {:ok, String.trim(tag)}
      {_, _} -> {:error, "No tags found or not a git repository"}
    end
  end
end
