defmodule ExplorerWeb.ChartComponents do
  use Phoenix.Component

  attr(:id, :string, required: true)
  attr(:chart_type, :string, required: true)
  attr(:chart_data, :string, required: true)
  attr(:chart_options, :string, required: true)

  defp basic_chart(assigns) do
    ~H"""
    <div class="relative h-full w-full">
      <canvas
        id={@id}
        phx-hook="ChartHook"
        data-chart-type={@chart_type}
        data-chart-data={@chart_data}
        data-chart-options={@chart_options}
        class="!w-full !h-full"
      >
      </canvas>
    </div>
    """
  end

  @doc """
  Renders a bar chart with aligned style.
  ## Examples
    <.bar_chart
      id="exchanges"
      points={%{x: [1, 2, 3, 4], y: ["01-01-2024", "01-02-2024", "01-03-2024", "01-04-2024"]},}
      show_ticks={%{x: true, y: true}}
      extra_data={%{merkle_roots: [0x1, 0x2, 0x3, 0x4]}}
    />
    !Note:
    - id is used to reference the chart on javascript to apply custom styles, configurations, tooltip, that are possible only via javascript
    - points: nil values are automatically ignored and not displayed
    - extra_data: any other data you might want to retrieve via javascript later
  """
  attr(:id, :string, required: true)
  attr(:points, :map, required: true)
  attr(:extra_data, :map, default: %{})
  attr(:show_ticks, :map, default: %{x: true, y: true})

  def bar_chart(assigns) do
    ~H"""
    <.basic_chart
      id={@id}
      chart_type="bar"
      chart_data={
        Jason.encode!(%{
          labels: @points,
          datasets: [
            Map.merge(
              %{
                data: @points,
                fill: false,
                tension: 0.1,
                backgroundColor: "rgba(24, 255, 128, 0.3)",
                hoverBackgroundColor: "rgba(24, 255, 128, 0.5)",
                borderColor: "rgb(24, 255, 127)",
                borderWidth: 1.5,
                borderRadius: 4
              },
              @extra_data
            )
          ]
        })
      }
      chart_options={
        Jason.encode!(%{
          animation: false,
          responsive: false,
          maintainAspectRatio: false,
          interaction: %{
            mode: "index",
            intersect: false
          },
          plugins: %{
            legend: %{
              display: false
            }
          },
          elements: %{
            point: %{
              pointStyle: false
            }
          },
          scales: %{
            x: %{
              bounds: "data",
              offset: false,
              ticks: %{
                display: @show_ticks.x,
                autoSkip: false,
                maxRotation: 0,
                font: %{
                  weight: "700"
                }
              },
              grid: %{
                display: false
              },
              border: %{
                display: false
              }
            },
            y: %{
              offset: false,
              beginAtZero: true,
              ticks: %{
                display: @show_ticks.y,
                autoSkip: false,
                maxRotation: 0,
                font: %{
                  weight: "700"
                }
              },
              grid: %{
                display: false
              },
              border: %{
                display: false
              }
            }
          }
        })
      }
    />
    """
  end

  @doc """
  Renders a linear chart with aligned style.
  ## Examples
    <.line_chart
      id="exchanges"
      points={%{x: [1, 2, 3, 4], y: ["01-01-2024", "01-02-2024", "01-03-2024", "01-04-2024"]},}
      show_ticks={%{x: true, y: true}}
      extra_data={%{merkle_roots: [0x1, 0x2, 0x3, 0x4]}}
    />
    !Note:
    - id is used to reference the chart on javascript to apply custom styles, configurations, tooltip, that are possible only via javascript
    - points: nil values are automatically ignored and not displayed
    - extra_data: any other data you might want to retrieve via javascript later
  """
  attr(:id, :string, required: true)
  attr(:points, :map, required: true)
  attr(:extra_data, :map, default: %{})
  attr(:show_ticks, :map, default: %{x: true, y: true})

  def line_chart(assigns) do
    ~H"""
    <.basic_chart
      id={@id}
      chart_type="line"
      chart_data={
        Jason.encode!(%{
          labels: @points,
          datasets: [
            Map.merge(
              %{data: @points, borderColor: "rgb(24, 255, 127)", fill: false, tension: 0.1},
              @extra_data
            )
          ]
        })
      }
      chart_options={
        Jason.encode!(%{
          animation: false,
          responsive: false,
          maintainAspectRatio: false,
          interaction: %{
            mode: "index",
            intersect: false
          },
          plugins: %{
            legend: %{
              display: false
            }
          },
          elements: %{
            point: %{
              pointStyle: false
            }
          },
          scales: %{
            x: %{
              type: "linear",
              bounds: "data",
              offset: false,
              ticks: %{
                display: @show_ticks.x,
                autoSkip: false,
                maxRotation: 0,
                font: %{
                  weight: "700"
                }
              },
              grid: %{
                display: false
              },
              border: %{
                display: false
              }
            },
            y: %{
              type: "linear",
              offset: false,
              ticks: %{
                display: @show_ticks.y,
                autoSkip: false,
                maxRotation: 0,
                font: %{
                  weight: "700"
                }
              },
              grid: %{
                display: false
              },
              border: %{
                display: false
              }
            }
          }
        })
      }
    />
    """
  end
end
