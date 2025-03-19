import {
	computePosition,
	autoUpdate,
	offset,
	shift,
	flip
} from "@floating-ui/dom";

// A class to manage the tooltip lifecycle
class Tooltip {
	showEvents = ["mouseenter", "focus"];
	hideEvents = ["mouseleave", "blur"];
	$parent;
	$tooltip;
	cleanup;

	constructor($tooltip) {
		this.$tooltip = $tooltip;
		this.$parent = $tooltip.parentElement;
		this.cleanup = null;
		this.setupEventListeners();
		this.setupFloatingUI();
	}

	setupEventListeners() {
		this.showEvents.forEach((event) => {
			this.$parent.addEventListener(
				event,
				this.show.bind(this)
			);
		});

		this.hideEvents.forEach((event) => {
			this.$parent.addEventListener(
				event,
				this.hide.bind(this)
			);
		});
	}

	setupFloatingUI() {
		this.cleanup = autoUpdate(this.$parent, this.$tooltip, () => {
			computePosition(this.$parent, this.$tooltip, {
				placement: "bottom",
				middleware: [offset(5), flip(), shift({ padding: 5 })]
			}).then(({ x, y }) => {
				Object.assign(this.$tooltip.style, {
					left: `${x}px`,
					top: `${y}px`
				});
			});
		});
	}

	show() {
		this.$tooltip.setAttribute("data-show", "");
	}

	hide() {
		this.$tooltip.removeAttribute("data-show");
	}

	destroy() {
		this.showEvents.forEach((event) => {
			this.$parent.removeEventListener(event, this.show);
		});

		this.hideEvents.forEach((event) => {
			this.$parent.removeEventListener(event, this.hide);
		});

		if (this.cleanup) {
			this.cleanup();
		}
	}
}

const tooltipHook = {
	mounted() {
		this.el.tooltip = new Tooltip(this.el);
	},
	updated() {
		this.el.tooltip?.update();
	},
	destroyed() {
		this.el.tooltip?.destroy();
	}
};

export default tooltipHook;
