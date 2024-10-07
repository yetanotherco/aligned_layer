const searchFocusHook = {
	mounted() {
		window.addEventListener("keydown", (e) => {
			// Check if the pressed key is 'K' and if Command (Mac) or Control (Windows) is also pressed
			if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
				e.preventDefault(); // Prevent default browser behavior
				this.el.focus(); // Focus the search input
			}
		});
	},
	destroyed() {
		window.removeEventListener("keydown", this.handleKeyDown);
	}
};

export default searchFocusHook;
