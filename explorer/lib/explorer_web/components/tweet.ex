defmodule TweetComponent do
  use ExplorerWeb, :live_component

  attr :text, :string, required: true
  attr :class, :string, default: nil

  @impl true
  def render(assigns) do
    ~H"""
    <div>
      <a
        href={"https://twitter.com/intent/tweet?text=#{@text}"}
        target="_blank"
        class="twitter-share-button"
        data-size="large"
        data-related="alignedlayer"
        data-dnt="true"
        data-show-count="truefalse"
        data-url=" "
      >
        <button class={[
          "bg-black text-neutral-50 hover:bg-neutral-700 px-3 pb-0.5",
          "font-bold text-sm leading-[18px] tracking-wider rounded-full",
          @class
        ]}>
          <span class="text-xl font-normal">𝕏</span> <span class="tracking-tight">Post</span>
        </button>
      </a>
      <script async src="https://platform.twitter.com/widgets.js" charset="utf-8" />
    </div>
    """
  end
end
