const findClosestIndex = (target, values) => {
	let closestIndex = 0;
	let smallestDiff = Math.abs(values[0] - target);
	for (let i = 1; i < values.length; i++) {
		const diff = Math.abs(values[i] - target);
		if (diff < smallestDiff) {
			closestIndex = i;
			smallestDiff = diff;
		}
	}
	return closestIndex;
};

/**
 * A callback function to customize y-axis tick labels by showing only the zero, minimum and maximum data values.
 *
 * @param {Object} data - The chart data object containing datasets and their values.
 * @param {Function} renderText - A function to format and render text for the tick labels.
 * @returns {Function} - A callback function for Chart.js tick customization.
 *
 * The returned function compares the current tick index with the indices of the values closest
 * to the minimum and maximum data points, and displays these values formatted using the
 * `renderText` function.
 *
 * @example
 * options.scales.y.ticks.callback = yTickCallbackShowMinAndMaxValues(data, (val) => `${val} USD`);
 */
export const yTickCallbackShowMinAndMaxValues =
	(data, renderText) => (_value, index, values) => {
		if (index === 0) return renderText(0);

		const dataY = data.datasets[0].data.map((point) => parseFloat(point.y));
		const sortedData = dataY.sort((a, b) => b - a);
		const min = sortedData[0];
		const max = sortedData[sortedData.length - 1];
		const valsData = values.map((item) => item.value);
		const idxClosestToMin = findClosestIndex(min, valsData);
		const idxClosestToMax = findClosestIndex(max, valsData);

		if (index == idxClosestToMin) return renderText(min);
		if (index == idxClosestToMax) return renderText(max);
		return "";
	};
