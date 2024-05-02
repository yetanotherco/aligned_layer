defmodule ExplorerWeb.HomeLive.Index do
  use ExplorerWeb, :live_view

  def handle_event("search_task", %{"task" => task_params}, socket) do
    task_id = Map.get(task_params, "id")

    {:noreply, redirect(socket, to: "/task/#{task_id}")}
  end

  def render(assigns) do
    ~H"""
    <div class="flex flex-col items-center justify-center w-full">
      <div class="text-center text-gray-500 dark:text-gray-400 mb-8">
        <h1 class="text-3xl font-bold tracking-tighter">Aligned Explorer</h1>
      </div>
      <form phx-submit="search_task" class="flex items-center w-full max-w-md gap-2">
        <input
          class="flex h-10 w-full ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed flex-1 rounded-md border border-gray-200 bg-white px-4 py-2 text-sm font-medium shadow-sm transition-colors hover:bg-gray-100 hover:text-gray-900 focus:outline-none focus:ring-1 focus:ring-gray-950 disabled:pointer-events-none disabled:opacity-50 dark:border-gray-800 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus:ring-gray-300"
          type="search"
          placeholder="Search operator task..."
          name="task[id]"
        />
        <button
          type="submit"
          class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10 rounded-full"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="w-5 h-5 stroke-gray-500 dark:stroke-gray-400"
          >
            <circle cx="11" cy="11" r="8"></circle>
            <path d="m21 21-4.3-4.3"></path>
          </svg>
          <span class="sr-only">Search</span>
        </button>
      </form>
    </div>
    """
  end
end
