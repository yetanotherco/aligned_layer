const themeCookieKey = "theme";

const isDark = () => {
	const theme = document.cookie
		.split("; ")
		.find((row) => row.startsWith(`${themeCookieKey}=`))
		?.split("=")[1];
	return theme == "dark";
};

const setThemeCookie = (theme) => {
	document.cookie = `${themeCookieKey}=${theme}; path=/; max-age=31536000; SameSite=Strict;`; // would expire in a yer
};

const setupThemeToggle = () => {
	toggleVisibility = (dark) => {
		const themeToggleDarkIcon = document.getElementById(
			"theme-toggle-dark-icon"
		);
		const themeToggleLightIcon = document.getElementById(
			"theme-toggle-light-icon"
		);
		if (themeToggleDarkIcon == null || themeToggleLightIcon == null) return;
		const show = dark ? themeToggleDarkIcon : themeToggleLightIcon;
		const hide = dark ? themeToggleLightIcon : themeToggleDarkIcon;
		show.classList.remove("hidden", "text-transparent");
		hide.classList.add("hidden", "text-transparent");
		if (dark) {
			document.documentElement.classList.add("dark");
			setThemeCookie("dark");
		} else {
			document.documentElement.classList.remove("dark");
			setThemeCookie("light");
		}
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
	updated() {},
};

export default darkModeHook;
