const copyToClipboardHook = {
	mounted() {
		this.el.addEventListener("click", (e) => {
			e.preventDefault();
			const text = this.el.getAttribute("data-clipboard-text");
			navigator.clipboard
				.writeText(text)
				.then(() => {
					// Optional: Provide feedback to the user
					this.pushEventTo(this.el, "copied", {});
				})
				.catch((err) => {
					console.error("Failed to copy text: ", err);
				});
		});
	}
};

export default copyToClipboardHook;
