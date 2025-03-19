import { yTickCallbackShowMinAndMaxValues } from "./helpers";
import { alignedTooltip } from "./tooltip";

export const costPerProofCustomOptions = (options, data) => {
	// show only 0, min and max values
	options.scales.y.ticks.callback = yTickCallbackShowMinAndMaxValues(
		data,
		(val) => `${val} USD`
	);

	// show age min, mean and max age in x axis
	options.scales.x.ticks.callback = (_value, index, values) => {
		const age = data.datasets[0].age;
		if (index === 0) return age[0];
		if (index === Math.floor((age.length - 1) / 2))
			return age[Math.floor((age.length - 1) / 2)];
		if (index === values.length - 1) return age[age.length - 1];
		return "";
	};

	options.plugins.tooltip.external = (context) =>
		alignedTooltip(context, {
			name: "cost-per-proof",
			title: "Cost per proof",
			items: [
				{ title: "Fee per proof", id: "cost" },
				{ title: "Age", id: "age" },
				{ title: "Merkle root", id: "merkle_root" },
				{ title: "Block number", id: "block_number" },
				{ title: "Amount of proofs", id: "amount_of_proofs" },
			],
			onTooltipClick: (tooltipModel) => {
				const dataset = tooltipModel.dataPoints[0].dataset;
				const idx = tooltipModel.dataPoints[0].dataIndex;
				const merkleRootHash = dataset.merkle_root[idx];
				window.location.href = `/batches/${merkleRootHash}`;
			},
			onTooltipUpdate: (tooltipModel) => {
				const dataset = tooltipModel.dataPoints[0].dataset;
				const idx = tooltipModel.dataPoints[0].dataIndex;

				const cost = `${dataset.data[idx].y} USD`;
				const age = dataset.age[idx];
				const merkleRootHash = dataset.merkle_root[idx];
				const merkle_root = `${merkleRootHash.slice(
					0,
					6
				)}...${merkleRootHash.slice(merkleRootHash.length - 4)}`;
				const block_number = dataset.data[idx].x;
				const amount_of_proofs = dataset.amount_of_proofs[idx];

				return {
					cost,
					age,
					merkle_root,
					block_number,
					amount_of_proofs,
				};
			},
		});
};
