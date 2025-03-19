defmodule ReleasesHelper do
  require Logger

  def get_latest_release do
    System.get_env("LATEST_RELEASE")
  end
end
