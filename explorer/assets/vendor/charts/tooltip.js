const tooltipItem = (title, valueId) => `
  <div class="chart-tooltip-item">
    <p class="chart-tooltip-item-title">${title}</p>
    <p class="chart-tooltip-item-value" id=${valueId}></p>
  </div>
`;

const tooltipComponent = ({ title, isTooltipClickable, items }) => `
  <div class="chart-tooltip-container">
	<div class="chart-tooltip-dot" 
		 style="pointer-events: ${isTooltipClickable ? "auto" : "none"}">
	</div>
	<div class="chart-tooltip-items-container">
		<p class="chart-tooltip-title">${title}</p>
		<div class="chart-tooltip-items">
			${items.map((item) => tooltipItem(item.title, item.id)).join("")}
		</div>
	</div>
  </div>
`;

/**
 * Displays and positions a custom tooltip on the page based on the provided chart context.
 * This function creates the tooltip element if it doesn't exist, updates its content,
 * and positions it correctly based on the chart's canvas and tooltip model.
 *
 * @param {Object} context - The chart.js context, typically passed as `this` within chart hooks.
 * @param {Object} params - An object containing configuration for the tooltip.
 * @param {Object} params.name - A string that serves as an identifier for the tooltip.
 * @param {string} params.title - The title text to display in the tooltip.
 * @param {Array} params.items - An array of items (with ids) to be displayed inside the tooltip.
 * @param {Array} params.onTooltipClick - A callback that receives `tooltipModel` and gets triggered when the tooltip is clicked.
 * @param {Function} params.onTooltipUpdate - A callback function that updates the tooltip values based on the tooltip model.
 *                                           It is called with the `tooltipModel` as an argument and should return an object containing updated values for each tooltip item by their respective `id`.
 *
 * @returns {void} - This function doesn't return any value. It manipulates the DOM directly to show the tooltip.
 *
 * @example
 * alignedTooltip(context, {
 *   name: "my-chart",
 *   title: "Tooltip Title",
 *   items: [{ title: "Cost", id: "cost_id" }, { title: "Timestamp", id: "timestamp_id" }],
 *   onTooltipClick: (tooltipModel) => {
 * 		const dataset = tooltipModel.dataPoints[0].dataset;
		const idx = tooltipModel.dataPoints[0].dataIndex;

 * 		const y = tooltipModel.dataPoints[0].data[idx].y
 * 		window.location.href = `/batches/${y}`
 *   }
 *   onTooltipUpdate: (tooltipModel) => {
 * 	   const cost = tooltipModel.dataPoints[0].raw;
 * 	   const timestamp = tooltipModel.dataPoints[0].label;
 *
 *     return {
 *       cost_id: cost,
 *       timestamp_id: label
 *     };
 *   }
 * });
 */
export const alignedTooltip = (
	context,
	{ name, title, items, onTooltipClick, onTooltipUpdate }
) => {
	const tooltipModel = context.tooltip;
	let tooltipEl = document.getElementById(`chartjs-tooltip-${name}`);
	if (!tooltipEl) {
		tooltipEl = document.createElement("div");
		tooltipEl.style = "transition: opacity 0.3s;";
		tooltipEl.style = "transition: left 0.1s;";
		tooltipEl.id = `chartjs-tooltip-${name}`;
		tooltipEl.innerHTML = tooltipComponent({
			title,
			isTooltipClickable: !!onTooltipClick,
			items,
		});
		document.body.appendChild(tooltipEl);
		tooltipEl.onmouseenter = () => {
			window.isTooltipBeingHovered = true;
		};
		tooltipEl.onmouseleave = () => {
			window.isTooltipBeingHovered = false;
			tooltipEl.style.opacity = 0;
			tooltipEl.style.zIndex = -1;
		};
		// this is needed to maintain responsiveness
		window.addEventListener("resize", () => {
			tooltipEl.remove();
		});
		if (onTooltipClick)
			tooltipEl.querySelector(".chart-tooltip-dot").onclick = () =>
				onTooltipClick(tooltipModel);
	}

	// Hide element if no tooltip
	if (tooltipModel.opacity == 0 && !window.isTooltipBeingHovered) {
		tooltipEl.style.opacity = 0;
		return;
	}

	const values = onTooltipUpdate(tooltipModel);
	items.forEach((item) => {
		tooltipEl.querySelector(`#${item.id}`).textContent = values[item.id];
	});

	const position = context.chart.canvas.getBoundingClientRect();

	tooltipEl.style.opacity = 1;
	tooltipEl.style.zIndex = 100;
	tooltipEl.style.position = "absolute";
	tooltipEl.style.left =
		position.left -
		tooltipEl.offsetWidth / 2 +
		window.scrollX +
		tooltipModel.caretX +
		"px";
	tooltipEl.style.top =
		position.top -
		tooltipEl.offsetHeight +
		window.scrollY +
		tooltipModel.caretY +
		"px";
};
