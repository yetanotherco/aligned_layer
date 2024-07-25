// You can include dependencies in two ways.
//
// The simplest option is to put them in assets/vendor and
// import them using relative paths:
//
//     import "../vendor/some-package.js"
//
// Alternatively, you can `npm install some-package --prefix assets` and import
// them using a path starting with the package name:
//
//     import "some-package"
//

import "phoenix_html";
import { Socket } from "phoenix";
import { LiveSocket } from "phoenix_live_view";
import topbar from "../vendor/topbar";

import darkModeHook from "../vendor/dark_mode";
import searchFocusHook from "../vendor/search_focus";
import tooltipHook from "../vendor/tooltip";

let Hooks = {};
Hooks.DarkThemeToggle = darkModeHook;
Hooks.SearchFocus = searchFocusHook;
Hooks.TooltipHook = tooltipHook;

let csrfToken = document
	.querySelector("meta[name='csrf-token']")
	.getAttribute("content");

let liveSocket = new LiveSocket("/live", Socket, {
	params: { _csrf_token: csrfToken },
	hooks: Hooks
});

// Show progress bar on live navigation and form submits
topbar.config({
	barColors: { 0: "#9AE497" },
	shadowColor: "rgba(0, 0, 0, .3)"
});
window.addEventListener("phx:page-loading-start", (_info) =>
	topbar.show(50)
);
window.addEventListener("phx:page-loading-stop", (_info) =>
	topbar.hide()
);

// connect if there are any LiveViews on the page
liveSocket.connect();

// expose liveSocket on window for web console debug logs and latency simulation:
// >> liveSocket.enableDebug()
// >> liveSocket.enableLatencySim(1000)  // enabled for duration of browser session
// >> liveSocket.disableLatencySim()
window.liveSocket = liveSocket;
