defmodule ExplorerWeb.TaskHTML do
  @moduledoc """
  This module contains pages rendered by TaskController.

  See the `task_html` directory for all templates available.
  """
  use ExplorerWeb, :html

  embed_templates "task_html/*"
end
