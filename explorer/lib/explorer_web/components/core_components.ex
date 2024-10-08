defmodule ExplorerWeb.CoreComponents do
  @moduledoc """
  Provides core UI components.

  At first glance, this module may seem daunting, but its goal is to provide
  core building blocks for your application, such as modals, tables, and
  forms. The components consist mostly of markup and are well-documented
  with doc strings and declarative assigns. You may customize and style
  them in any way you want, based on your application growth and needs.

  The default components use Tailwind CSS, a utility-first CSS framework.
  See the [Tailwind CSS documentation](https://tailwindcss.com) to learn
  how to customize them or feel free to swap in another framework altogether.

  Icons are provided by [heroicons](https://heroicons.com). See `icon/1` for usage.
  """
  use Phoenix.Component

  alias Phoenix.LiveView.JS
  import ExplorerWeb.Gettext
  import Tails, only: [classes: 1]

  @doc """
  Renders a modal.

  ## Examples

      <.modal id="confirm-modal">
        This is a modal.
      </.modal>

  JS commands may be passed to the `:on_cancel` to configure
  the closing/cancel event, for example:

      <.modal id="confirm" on_cancel={JS.navigate(~p"/posts")}>
        This is another modal.
      </.modal>

  """
  attr :id, :string, required: true
  attr :show, :boolean, default: false
  attr :on_cancel, JS, default: %JS{}
  slot :inner_block, required: true

  def modal(assigns) do
    ~H"""
    <div
      id={@id}
      phx-mounted={@show && show_modal(@id)}
      phx-remove={hide_modal(@id)}
      data-cancel={JS.exec(@on_cancel, "phx-remove")}
      class="relative z-50 hidden"
    >
      <div id={"#{@id}-bg"} class="bg-zinc-50/90 fixed inset-0 transition-opacity" aria-hidden="true" />
      <div
        class="fixed inset-0 overflow-y-auto"
        aria-labelledby={"#{@id}-title"}
        aria-describedby={"#{@id}-description"}
        role="dialog"
        aria-modal="true"
        tabindex="0"
      >
        <div class="flex min-h-full items-center justify-center">
          <div class="w-full max-w-3xl p-4 sm:p-6 lg:py-8">
            <.focus_wrap
              id={"#{@id}-container"}
              phx-window-keydown={JS.exec("data-cancel", to: "##{@id}")}
              phx-key="escape"
              phx-click-away={JS.exec("data-cancel", to: "##{@id}")}
              class="shadow-zinc-700/10 ring-zinc-700/10 relative hidden rounded-2xl bg-white p-14 shadow-lg ring-1 transition"
            >
              <div class="absolute top-6 right-5">
                <button
                  phx-click={JS.exec("data-cancel", to: "##{@id}")}
                  type="button"
                  class="-m-3 flex-none p-3 opacity-20 hover:opacity-40"
                  aria-label={gettext("close")}
                >
                  <.icon name="hero-x-mark-solid" class="h-5 w-5" />
                </button>
              </div>
              <div id={"#{@id}-content"}>
                <%= render_slot(@inner_block) %>
              </div>
            </.focus_wrap>
          </div>
        </div>
      </div>
    </div>
    """
  end

  @doc """
  Renders flash notices.

  ## Examples

      <.flash kind={:info} flash={@flash} />
      <.flash kind={:info} phx-mounted={show("#flash")}>Welcome Back!</.flash>
  """
  attr :id, :string, doc: "the optional id of flash container"
  attr :flash, :map, default: %{}, doc: "the map of flash messages to display"
  attr :title, :string, default: nil
  attr :kind, :atom, values: [:info, :error], doc: "used for styling and flash lookup"
  attr :rest, :global, doc: "the arbitrary HTML attributes to add to the flash container"
  attr :delay, :boolean, default: false, doc: "optional 3s delay for the flash message"

  slot :inner_block, doc: "the optional inner block that renders the flash message"

  def flash(assigns) do
    assigns = assign_new(assigns, :id, fn -> "flash-#{assigns.kind}" end)

    ~H"""
    <div
      :if={msg = render_slot(@inner_block) || Phoenix.Flash.get(@flash, @kind)}
      id={@id}
      phx-click={JS.push("lv:clear-flash", value: %{key: @kind}) |> hide("##{@id}")}
      role="alert"
      class={
        classes([
          "fixed bottom-5 right-2 mr-2 w-80 sm:w-96 z-50 rounded-lg p-3 ring-1",
          "fixed bottom-5 right-2 mr-2 w-80 sm:w-96 z-50 rounded-lg p-3 ring-1",
          @kind == :info && "bg-emerald-50 text-emerald-800 ring-emerald-500 fill-cyan-900",
          @kind == :error && "bg-rose-50 text-rose-900 shadow-md ring-rose-500 fill-rose-900",
          @delay && "delay-&lsqb;3s&rsqb;"
        ])
      }
      {@rest}
    >
      <p :if={@title} class="flex items-center gap-1.5 text-sm font-semibold leading-6">
        <.icon :if={@kind == :info} name="hero-information-circle-mini" class="h-4 w-4" />
        <.icon :if={@kind == :error} name="hero-exclamation-circle-mini" class="h-4 w-4" />
        <%= @title %>
      </p>
      <p class="mt-2 text-sm leading-5 break-all"><%= msg %></p>
      <button type="button" class="group absolute top-1 right-1 p-2" aria-label={gettext("close")}>
        <.icon name="hero-x-mark-solid" class="h-5 w-5 opacity-40 group-hover:opacity-70" />
      </button>
    </div>
    """
  end

  @doc """
  Shows the flash group with standard titles and content.

  ## Examples

      <.flash_group flash={@flash} />
  """
  attr :flash, :map, required: true, doc: "the map of flash messages"
  attr :id, :string, default: "flash-group", doc: "the optional id of flash container"

  def flash_group(assigns) do
    ~H"""
    <div id={@id}>
      <.flash kind={:info} title={gettext("Success!")} flash={@flash} />
      <.flash kind={:error} title={gettext("Error!")} flash={@flash} />
      <.flash
        id="client-error"
        kind={:error}
        title={gettext("We can't find the internet")}
        phx-disconnected={show(".phx-client-error #client-error")}
        phx-connected={hide("#client-error")}
        delay
        hidden
      >
        <%= gettext("Attempting to reconnect") %>
        <.icon name="hero-arrow-path" class="ml-1 h-3 w-3 animate-spin" />
      </.flash>

      <.flash
        id="server-error"
        kind={:error}
        title={gettext("Something went wrong!")}
        phx-disconnected={show(".phx-server-error #server-error")}
        phx-connected={hide("#server-error")}
        hidden
      >
        <%= gettext("Hang in there while we get back on track") %>
        <.icon name="hero-arrow-path" class="ml-1 h-3 w-3 animate-spin" />
      </.flash>
    </div>
    """
  end

  @doc """
  Renders a simple form.

  ## Examples

      <.simple_form for={@form} phx-change="validate" phx-submit="save">
        <.input field={@form[:email]} label="Email"/>
        <.input field={@form[:username]} label="Username" />
        <:actions>
          <.button>Save</.button>
        </:actions>
      </.simple_form>
  """
  attr :for, :any, required: true, doc: "the datastructure for the form"
  attr :as, :any, default: nil, doc: "the server side parameter to collect all input under"

  attr :rest, :global,
    include: ~w(autocomplete name rel action enctype method novalidate target multipart),
    doc: "the arbitrary HTML attributes to apply to the form tag"

  slot :inner_block, required: true
  slot :actions, doc: "the slot for form actions, such as a submit button"

  def simple_form(assigns) do
    ~H"""
    <.form :let={f} for={@for} as={@as} {@rest}>
      <div class="mt-10 space-y-8 bg-white">
        <%= render_slot(@inner_block, f) %>
        <div :for={action <- @actions} class="mt-2 flex items-center justify-between gap-6">
          <%= render_slot(action, f) %>
        </div>
      </div>
    </.form>
    """
  end

  @doc """
  Renders a button. To add an icon just search for the icon name in the https://heroicons.com/ and pass it as the icon attribute.

  ## Examples

      <.button>Send!</.button>
      <.button phx-click="go" class="ml-2">Send!</.button>
  """
  attr :type, :string, default: nil
  attr :class, :string, default: nil
  attr :rest, :global, include: ~w(disabled form name value)
  attr :icon, :string, default: nil
  attr :icon_class, :string, default: nil

  slot :inner_block, required: true

  def button(assigns) do
    ~H"""
    <button
      type={@type}
      class={[
        "phx-submit-loading:opacity-75 rounded-lg bg-card hover:bg-muted py-2 px-3",
        "text-sm font-semibold leading-6 text-foregound active:text-foregound/80",
        "border border-foreground/20 inline-flex items-center gap-1.5",
        @class
      ]}
      {@rest}
    >
      <.icon :if={@icon != nil} name={"hero-#{@icon}"} class={"size-4 stroke-inherit #{@icon_class}"} />
      <%= render_slot(@inner_block) %>
    </button>
    """
  end

  @doc """
  Root background component.
  """
  slot :inner_block, default: nil

  def root_background(assigns) do
    ~H"""
    <main class="px-4 sm:px-6 lg:px-8 pt-20 pb-8 selection:bg-accent/80 selection:text-accent-foreground/80 min-h-dvh">
      <%= render_slot(@inner_block) %>
    </main>
    """
  end

  @doc """
    Renders a card background.
  """
  attr :class, :string, default: nil
  slot :inner_block, default: nil

  def card_background(assigns) do
    ~H"""
    <div class={classes(["bg-card border border-foreground/20 rounded-2xl p-4", @class])}>
      <%= render_slot(@inner_block) %>
    </div>
    """
  end

  @doc """
    Renders a heading to use before entering a card.
  """
  attr :class, :string, default: nil
  slot :inner_block, default: nil

  def card_preheding(assigns) do
    ~H"""
    <h1 class={
      classes([
        "text-4xl sm:text-5xl font-bold font-foreground text-left py-2",
        @class
      ])
    }>
      <%= render_slot(@inner_block) %>
    </h1>
    """
  end

  @doc """
  Renders a card with a title and content.
  """
  attr :class, :string, default: nil
  attr :title, :string, default: nil
  attr :inner_class, :string, default: nil

  slot :inner_block, default: nil

  def card(assigns) do
    ~H"""
    <.card_background class={@class}>
      <h2 class="font-medium text-muted-foreground capitalize">
        <%= @title %>
      </h2>
      <span class={classes(["text-4xl font-bold slashed-zero", @inner_class])}>
        <%= render_slot(@inner_block) %>
      </span>
    </.card_background>
    """
  end

  @doc """
  Renders a card with a link and title that has a hyperlink icon and underline on hover.
  """
  attr :class, :string, default: nil
  attr :inner_class, :string, default: nil
  attr :title, :string, default: nil
  attr :href, :string, default: nil
  attr :rest, :global, include: ~w(href target navigate)
  attr :icon, :string, default: "hero-arrow-top-right-on-square-solid"

  slot :inner_block, default: nil

  def card_link(assigns) do
    ~H"""
    <.link target="_blank" href={@href} class="group" {@rest}>
      <.card_background class={@class}>
        <h2 class="font-medium text-muted-foreground capitalize group-hover:underline truncate">
          <%= @title %>
          <.icon name={@icon} class="size-4 mb-1" />
        </h2>
        <span class={classes(["text-4xl font-bold slashed-zero", @inner_class])}>
          <%= render_slot(@inner_block) %>
        </span>
      </.card_background>
    </.link>
    """
  end

  @doc """
    Renders an arrow icon.
  """
  attr :class, :string, default: nil

  def right_arrow(assigns) do
    ~H"""
    <.icon
      name="hero-arrow-right-solid"
      class="size-4 stroke-foreground group-hover:stroke-foreground/80 -translate-x-1 group-hover:translate-x-0 duration-150 transition-all"
    />
    """
  end

  @doc """
    Renders an anchor tag.
  """
  attr :class, :string, default: nil
  attr :rest, :global, include: ~w(href target)
  slot :inner_block, default: nil

  def a(assigns) do
    ~H"""
    <.link
      class={
        classes([
          "underline underline-offset-4 space-x-0.5 font-medium hover:text-foreground/80",
          @class
        ])
      }
      {@rest}
    >
      <%= render_slot(@inner_block) %>
      <.icon name="hero-arrow-top-right-on-square-solid" class="size-4 mb-1" />
    </.link>
    """
  end

  @doc """
    Renders a badge component.
  """
  attr :class, :string, default: nil
  attr :variant, :string, default: "accent"
  slot :inner_block, default: nil

  def badge(assigns) do
    ~H"""
    <span class={
      classes([
        "px-3 py-1 rounded-full font-semibold",
        case @variant do
          "accent" ->
            "text-accent-foreground bg-accent group-hover:bg-accent/80"

          "primary" ->
            "text-primary-foreground bg-primary group-hover:bg-primary/80"

          "secondary" ->
            "text-secondary-foreground bg-secondary group-hover:bg-secondary/80"

          "destructive" ->
            "text-destructive-foreground bg-destructive group-hover:bg-destructive/80"

          "foreground" ->
            "text-background bg-foreground group-hover:bg-foreground/80"

          "outline" ->
            "text-foreground/80 border border-foreground/20 group-hover:bg-muted-foreground/20"

          "card" ->
            "text-card-foreground bg-card group-hover:bg-card/80"

          _ ->
            "text-accent-foreground bg-accent group-hover:bg-accent/80"
        end,
        @class
      ])
    }>
      <%= render_slot(@inner_block) %>
    </span>
    """
  end

  @doc """
    Renders a dynamic badge compoent.
  """
  attr :class, :string, default: nil
  attr :status, :boolean, default: true
  attr :falsy_text, :string, default: "Pending"
  attr :truthy_text, :string, default: "Verified"
  slot :inner_block, default: nil

  def dynamic_badge(assigns) do
    ~H"""
    <.badge
      variant={
        case @status do
          true -> "accent"
          false -> "foreground"
        end
      }
      class={
        classes([
          @class
        ])
      }
    >
      <%= case @status do
        true -> @truthy_text
        false -> @falsy_text
      end %>
      <%= render_slot(@inner_block) %>
    </.badge>
    """
  end

  @doc """
  Renders an input with label and error messages.

  A `Phoenix.HTML.FormField` may be passed as argument,
  which is used to retrieve the input name, id, and values.
  Otherwise all attributes may be passed explicitly.

  ## Types

  This function accepts all HTML input types, considering that:

    * You may also set `type="select"` to render a `<select>` tag

    * `type="checkbox"` is used exclusively to render boolean values

    * For live file uploads, see `Phoenix.Component.live_file_input/1`

  See https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input
  for more information. Unsupported types, such as hidden and radio,
  are best written directly in your templates.

  ## Examples

      <.input field={@form[:email]} type="email" />
      <.input name="my-input" errors={["oh no!"]} />
  """
  attr :id, :any, default: nil
  attr :name, :any
  attr :label, :string, default: nil
  attr :value, :any

  attr :type, :string,
    default: "text",
    values: ~w(checkbox color date datetime-local email file month number password
               range search select tel text textarea time url week)

  attr :field, Phoenix.HTML.FormField,
    doc: "a form field struct retrieved from the form, for example: @form[:email]"

  attr :errors, :list, default: []
  attr :checked, :boolean, doc: "the checked flag for checkbox inputs"
  attr :prompt, :string, default: nil, doc: "the prompt for select inputs"
  attr :options, :list, doc: "the options to pass to Phoenix.HTML.Form.options_for_select/2"
  attr :multiple, :boolean, default: false, doc: "the multiple flag for select inputs"

  attr :rest, :global,
    include: ~w(accept autocomplete capture cols disabled form list max maxlength min minlength
                multiple pattern placeholder readonly required rows size step)

  slot :inner_block

  def input(%{field: %Phoenix.HTML.FormField{} = field} = assigns) do
    assigns
    |> assign(field: nil, id: assigns.id || field.id)
    |> assign(:errors, Enum.map(field.errors, &translate_error(&1)))
    |> assign_new(:name, fn -> if assigns.multiple, do: field.name <> "[]", else: field.name end)
    |> assign_new(:value, fn -> field.value end)
    |> input()
  end

  def input(%{type: "checkbox"} = assigns) do
    assigns =
      assign_new(assigns, :checked, fn ->
        Phoenix.HTML.Form.normalize_value("checkbox", assigns[:value])
      end)

    ~H"""
    <div phx-feedback-for={@name}>
      <label class="flex items-center gap-4 text-sm leading-6 text-zinc-600">
        <input type="hidden" name={@name} value="false" />
        <input
          type="checkbox"
          id={@id}
          name={@name}
          value="true"
          checked={@checked}
          class="rounded border-zinc-300 text-zinc-900 focus:ring-0"
          {@rest}
        />
        <%= @label %>
      </label>
      <.error :for={msg <- @errors}><%= msg %></.error>
    </div>
    """
  end

  def input(%{type: "select"} = assigns) do
    ~H"""
    <div phx-feedback-for={@name}>
      <.label for={@id}><%= @label %></.label>
      <select
        id={@id}
        name={@name}
        class="mt-2 block w-full rounded-md border border-gray-300 bg-white shadow-sm focus:border-zinc-400 focus:ring-0 sm:text-sm"
        multiple={@multiple}
        {@rest}
      >
        <option :if={@prompt} value=""><%= @prompt %></option>
        <%= Phoenix.HTML.Form.options_for_select(@options, @value) %>
      </select>
      <.error :for={msg <- @errors}><%= msg %></.error>
    </div>
    """
  end

  def input(%{type: "textarea"} = assigns) do
    ~H"""
    <div phx-feedback-for={@name}>
      <.label for={@id}><%= @label %></.label>
      <textarea
        id={@id}
        name={@name}
        class={
          classes([
            "mt-2 block w-full rounded-lg text-zinc-900 focus:ring-0 sm:text-sm sm:leading-6",
            "min-h-[6rem] phx-no-feedback:border-zinc-300 phx-no-feedback:focus:border-zinc-400",
            @errors == [] && "border-zinc-300 focus:border-zinc-400",
            @errors != [] && "border-rose-400 focus:border-rose-400"
          ])
        }
        {@rest}
      ><%= Phoenix.HTML.Form.normalize_value("textarea", @value) %></textarea>
      <.error :for={msg <- @errors}><%= msg %></.error>
    </div>
    """
  end

  # All other inputs text, datetime-local, url, password, etc. are handled here...
  def input(assigns) do
    ~H"""
    <div phx-feedback-for={@name}>
      <.label for={@id}><%= @label %></.label>
      <input
        type={@type}
        name={@name}
        id={@id}
        value={Phoenix.HTML.Form.normalize_value(@type, @value)}
        class={
          classes([
            "mt-2 block w-full rounded-lg text-zinc-900 focus:ring-0 sm:text-sm sm:leading-6",
            "phx-no-feedback:border-zinc-300 phx-no-feedback:focus:border-zinc-400",
            @errors == [] && "border-zinc-300 focus:border-zinc-400",
            @errors != [] && "border-rose-400 focus:border-rose-400"
          ])
        }
        {@rest}
      />
      <.error :for={msg <- @errors}><%= msg %></.error>
    </div>
    """
  end

  @doc """
  Renders a label.
  """
  attr :for, :string, default: nil
  slot :inner_block, required: true

  def label(assigns) do
    ~H"""
    <label for={@for} class="block text-sm font-semibold leading-6 text-zinc-800">
      <%= render_slot(@inner_block) %>
    </label>
    """
  end

  @doc """
  Generates a generic error message.
  """
  slot :inner_block, required: true

  def error(assigns) do
    ~H"""
    <p class="mt-3 flex gap-3 text-sm leading-6 text-rose-600 phx-no-feedback:hidden">
      <.icon name="hero-exclamation-circle-mini" class="mt-0.5 h-5 w-5 flex-none" />
      <%= render_slot(@inner_block) %>
    </p>
    """
  end

  @doc """
  Renders a header with title.
  """
  attr :class, :string, default: nil

  slot :inner_block, required: true
  slot :subtitle
  slot :actions

  def header(assigns) do
    ~H"""
    <header class={[@actions != [] && "flex items-center justify-between gap-6", @class]}>
      <div>
        <h1 class="text-lg font-semibold leading-8 text-zinc-800">
          <%= render_slot(@inner_block) %>
        </h1>
        <p :if={@subtitle != []} class="mt-2 text-sm leading-6 text-zinc-600">
          <%= render_slot(@subtitle) %>
        </p>
      </div>
      <div class="flex-none"><%= render_slot(@actions) %></div>
    </header>
    """
  end

  @doc ~S"""
  Renders a table with custom styling.

  ## Examples

      <.table id="users" rows={@users}>
        <:col :let={user} label="id"><%= user.id %></:col>
        <:col :let={user} label="username"><%= user.username %></:col>
      </.table>
  """
  attr :id, :string, required: true
  attr :rows, :list, required: true
  attr :row_id, :any, default: nil, doc: "the function for generating the row id"
  attr :row_click, :any, default: nil, doc: "the function for handling phx-click on each row"

  attr :row_item, :any,
    default: &Function.identity/1,
    doc: "the function for mapping each row before calling the :col and :action slots"

  slot :col, required: true do
    attr :label, :string
    attr :class, :string
  end

  slot :action, doc: "the slot for showing user actions in the last table column"

  def table(assigns) do
    assigns =
      with %{rows: %Phoenix.LiveView.LiveStream{}} <- assigns do
        assign(assigns, row_id: assigns.row_id || fn {id, _item} -> id end)
      end

    ~H"""
    <.card_background class="overflow-x-auto">
      <table class="table-auto border-collapse w-full">
        <thead>
          <tr class="text-muted-foreground font-normal truncate">
            <th
              :for={{col, i} <- Enum.with_index(@col)}
              class={classes(["pr-4", i == 0 && "text-left", i != 0 && "text-center"])}
            >
              <%= col[:label] %>
            </th>
            <th :if={@action != []} class="p-0 pb-4">
              <span class="sr-only"><%= gettext("Actions") %></span>
            </th>
          </tr>
        </thead>
        <tbody id={@id} phx-update={match?(%Phoenix.LiveView.LiveStream{}, @rows) && "stream"}>
          <tr
            :for={row <- @rows}
            id={@row_id && @row_id.(row)}
            class="gap-y-2 [&>td]:pt-3 animate-in fade-in-0 duration-700 truncate"
          >
            <td
              :for={{col, _i} <- Enum.with_index(@col)}
              phx-click={@row_click && @row_click.(row)}
              class={classes(["p-0", @row_click && "hover:cursor-pointer"])}
            >
              <div class={
                classes([
                  "group block normal-case font-medium text-base min-w-28",
                  col[:class] != nil && col[:class],
                  col[:class] == nil && "text-center font-semibold"
                ])
              }>
                <%= render_slot(col, @row_item.(row)) %>
              </div>
            </td>
            <td :if={@action != []} class="w-14 p-0">
              <div class="whitespace-nowrap py-4 text-right text-sm font-medium">
                <span
                  :for={action <- @action}
                  class="ml-4 font-semibold leading-6 text-muted-foreground"
                >
                  <%= render_slot(action, @row_item.(row)) %>
                </span>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </.card_background>
    """
  end

  @doc """
  Renders an empty card background.

  ## Examples

      <.empty_card_background text="No users found" />

  """
  attr :class, :string, default: nil
  attr :inner_text_class, :string, default: nil
  attr :text, :string, default: nil
  slot :inner_block

  def empty_card_background(assigns) do
    ~H"""
    <.card_background class={
      classes([
        "overflow-x-auto min-h-[38.45rem] flex flex-col items-center justify-center gap-2",
        @class
      ])
    }>
      <p
        :if={@text != nil}
        class={
          classes([
            "text-lg text-muted-foreground",
            @inner_text_class
          ])
        }
      >
        <%= @text %>
      </p>
      <%= render_slot(@inner_block) %>
    </.card_background>
    """
  end

  @doc """
  Renders a data list.

  ## Examples

      <.list>
        <:item title="Title"><%= @post.title %></:item>
        <:item title="Views"><%= @post.views %></:item>
      </.list>
  """
  slot :item, required: true do
    attr :title, :string, required: true
  end

  def list(assigns) do
    ~H"""
    <div class="mt-14">
      <dl class="-my-4 divide-y divide-zinc-100">
        <div :for={item <- @item} class="flex gap-4 py-4 text-sm leading-6 sm:gap-8">
          <dt class="w-1/4 flex-none text-zinc-500"><%= item.title %></dt>
          <dd class="text-zinc-700"><%= render_slot(item) %></dd>
        </div>
      </dl>
    </div>
    """
  end

  @doc """
  Renders a [Heroicon](https://heroicons.com).

  Heroicons come in three styles – outline, solid, and mini.
  By default, the outline style is used, but solid and mini may
  be applied by using the `-solid` and `-mini` suffix.

  You can customize the size and colors of the icons by setting
  width, height, and background color classes.

  Icons are extracted from the `deps/heroicons` directory and bundled within
  your compiled app.css by the plugin in your `assets/tailwind.config.js`.

  ## Examples

      <.icon name="hero-x-mark-solid" />
      <.icon name="hero-arrow-path" class="ml-1 w-3 h-3 animate-spin" />
  """
  attr :name, :string, required: true
  attr :class, :string, default: nil

  def icon(%{name: "hero-" <> _} = assigns) do
    ~H"""
    <span class={classes([@name, @class])} />
    """
  end

  ## JS Commands

  def show(js \\ %JS{}, selector) do
    JS.show(js,
      to: selector,
      transition:
        {"transition-all transform ease-out duration-300",
         "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95",
         "opacity-100 translate-y-0 sm:scale-100"}
    )
  end

  def hide(js \\ %JS{}, selector) do
    JS.hide(js,
      to: selector,
      time: 200,
      transition:
        {"transition-all transform ease-in duration-200",
         "opacity-100 translate-y-0 sm:scale-100",
         "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"}
    )
  end

  def show_modal(js \\ %JS{}, id) when is_binary(id) do
    js
    |> JS.show(to: "##{id}")
    |> JS.show(
      to: "##{id}-bg",
      transition: {"transition-all transform ease-out duration-300", "opacity-0", "opacity-100"}
    )
    |> show("##{id}-container")
    |> JS.add_class("overflow-hidden", to: "body")
    |> JS.focus_first(to: "##{id}-content")
  end

  def hide_modal(js \\ %JS{}, id) do
    js
    |> JS.hide(
      to: "##{id}-bg",
      transition: {"transition-all transform ease-in duration-200", "opacity-100", "opacity-0"}
    )
    |> hide("##{id}-container")
    |> JS.hide(to: "##{id}", transition: {"block", "block", "hidden"})
    |> JS.remove_class("overflow-hidden", to: "body")
    |> JS.pop_focus()
  end

  @doc """
  Translates an error message using gettext.
  """
  def translate_error({msg, opts}) do
    # When using gettext, we typically pass the strings we want
    # to translate as a static argument:
    #
    #     # Translate the number of files with plural rules
    #     dngettext("errors", "1 file", "%{count} files", count)
    #
    # However the error messages in our forms and APIs are generated
    # dynamically, so we need to translate them by calling Gettext
    # with our gettext backend as first argument. Translations are
    # available in the errors.po file (as we use the "errors" domain).
    if count = opts[:count] do
      Gettext.dngettext(ExplorerWeb.Gettext, "errors", msg, msg, count, opts)
    else
      Gettext.dgettext(ExplorerWeb.Gettext, "errors", msg, opts)
    end
  end

  @doc """
  Translates the errors for a field from a keyword list of errors.
  """
  def translate_errors(errors, field) when is_list(errors) do
    for {^field, {msg, opts}} <- errors, do: translate_error({msg, opts})
  end

  @doc """
  Tooltip component.

  ## Example
      <.tooltip>
        <p>Hover over me</p>
      </.tooltip>

  """
  attr :class, :string, default: nil
  slot :inner_block, required: true

  def tooltip(assigns) do
    ~H"""
    <span
      id={Utils.random_id("tt")}
      class={
        classes([
          "tooltip",
          "animate-in fade-in slide-in-from-bottom duration-50",
          "px-2.5 py-1 text-sm text-foreground bg-popover border border-muted-foreground/30 rounded-full shadow-sm drop-shadow-sm",
          @class
        ])
      }
      role="tooltip"
      phx-hook="TooltipHook"
    >
      <%= render_slot(@inner_block) %>
    </span>
    """
  end

  @doc """
  Divider component.

  ## Example
      <.divider />

  """
  attr :class, :string, default: nil

  def divider(assigns) do
    ~H"""
    <hr class={classes(["border-t rounded-full border-muted-foreground/40 my-1.5", @class])} />
    """
  end
end
