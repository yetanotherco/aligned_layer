const localStorageKey = "theme";

const isDark = () => {
	if (localStorage.getItem(localStorageKey) === "dark") return true;
	if (localStorage.getItem(localStorageKey) === "light")
		return false;
	return window.matchMedia("(prefers-color-scheme: dark)").matches;
};

const setupThemeToggle = () => {
	toggleVisibility = (dark) => {
		const themeToggleDarkIcon = document.getElementById(
			"theme-toggle-dark-icon"
		);
		const themeToggleLightIcon = document.getElementById(
			"theme-toggle-light-icon"
		);
		if (
			themeToggleDarkIcon == null ||
			themeToggleLightIcon == null
		)
			return;
		const show = dark
			? themeToggleDarkIcon
			: themeToggleLightIcon;
		const hide = dark
			? themeToggleLightIcon
			: themeToggleDarkIcon;
		show.classList.remove("hidden", "text-transparent");
		hide.classList.add("hidden", "text-transparent");
		if (dark) {
			document.documentElement.classList.add("dark");
		} else {
			document.documentElement.classList.remove("dark");
		}
		try {
			localStorage.setItem(
				localStorageKey,
				dark ? "dark" : "light"
			);
		} catch (_err) {}
	};
	toggleVisibility(isDark());
	document
		.getElementById("theme-toggle")
		.addEventListener("click", function () {
			toggleVisibility(!isDark());
		});
};

const darkModeHook = {
	mounted() {
		setupThemeToggle();
	},
	updated() {}
};

export default darkModeHook;
