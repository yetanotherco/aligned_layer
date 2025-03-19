import "phoenix_html";
import { Socket } from "phoenix";
import { LiveSocket } from "phoenix_live_view";
import topbar from "../vendor/topbar";

import darkModeHook from "../vendor/dark_mode";
import searchFocusHook from "../vendor/search_focus";
import tooltipHook from "../vendor/tooltip";
import copyToClipboardHook from "../vendor/clipboard";
import chartHook from "../vendor/charts";

let hooks = {};
hooks.DarkThemeToggle = darkModeHook;
hooks.SearchFocus = searchFocusHook;
hooks.TooltipHook = tooltipHook;
hooks.CopyToClipboard = copyToClipboardHook;
hooks.ChartHook = chartHook;

let csrfToken = document
	.querySelector("meta[name='csrf-token']")
	.getAttribute("content");

let liveSocket = new LiveSocket("/live", Socket, {
	params: { _csrf_token: csrfToken },
	hooks: hooks,
});

topbar.config({
	barColors: { 0: "#18FF7F" },
	shadowColor: "rgba(0, 0, 0, .3)"
});
window.addEventListener("phx:page-loading-start", (_info) => topbar.show(50));
window.addEventListener("phx:page-loading-stop", (_info) => topbar.hide());

liveSocket.connect();

// expose liveSocket on window for web console debug logs and latency simulation:
// >> liveSocket.enableDebug()
// >> liveSocket.enableLatencySim(1000)  // enabled for duration of browser session
// >> liveSocket.disableLatencySim()
window.liveSocket = liveSocket;
