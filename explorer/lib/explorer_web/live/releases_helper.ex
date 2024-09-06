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
    case System.cmd("git", ["rev-list", "--tags", "--max-count=1"]) do
      {sha, 0} ->
        sha = String.trim(sha)
        case System.cmd("git", ["describe", "--tags", sha]) do
          {tag, 0} -> {:ok, String.trim(tag)}
          {_, _} -> {:error, "Failed to describe tag"}
        end
      {_, _} -> {:error, "No tags found or not a git repository"}
    end
  end
end
