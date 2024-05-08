defmodule ExplorerWeb.Tasks.Index do
  use ExplorerWeb, :live_view

  def mount(params, session, socket) do
    ExplorerWeb.Tasks.Controller.mount(params, session, socket)
  end

  def render(assigns) do
    ~H"""
      <div class="divTable">
          <div class="divRow">
              <div class="divCell">Header 1</div>
              <div class="divCell">Header 2</div>
              <div class="divCell">Header 3</div>
          </div>
          <div class="divRow">
              <div class="divCell">Row 1 Col 1</div>
              <div class="divCell">Row 1 Col 2</div>
              <div class="divCell">Row 1 Col 3</div>
          </div>
          <div class="divRow">
              <div class="divCell">Row 2 Col 1</div>
              <div class="divCell">Row 2 Col 2</div>
              <div class="divCell">Row 2 Col 3</div>
          </div>
      </div>
    """
  end
end
