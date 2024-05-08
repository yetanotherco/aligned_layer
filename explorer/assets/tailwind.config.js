// See the Tailwind configuration guide for advanced usage
// https://tailwindcss.com/docs/configuration

const plugin = require("tailwindcss/plugin");
const fs = require("fs");
const path = require("path");

module.exports = {
	darkMode: ["class"],
	content: [
		"./js/**/*.js",
		"../lib/explorer_web.ex",
		"../lib/explorer_web/**/*.*ex"
	],
	theme: {
		extend: {
			colors: {
				border: "hsl(var(--border))",
				input: "hsl(var(--input))",
				ring: "hsl(var(--ring))",
				background: "hsl(var(--background))",
				foreground: "hsl(var(--foreground))",
				primary: {
					DEFAULT: "hsl(var(--primary))",
					foreground: "hsl(var(--primary-foreground))"
				},
				secondary: {
					DEFAULT: "hsl(var(--secondary))",
					foreground: "hsl(var(--secondary-foreground))"
				},
				destructive: {
					DEFAULT: "hsl(var(--destructive))",
					foreground: "hsl(var(--destructive-foreground))"
				},
				muted: {
					DEFAULT: "hsl(var(--muted))",
					foreground: "hsl(var(--muted-foreground))"
				},
				accent: {
					DEFAULT: "hsl(var(--accent))",
					foreground: "hsl(var(--accent-foreground))"
				},
				popover: {
					DEFAULT: "hsl(var(--popover))",
					foreground: "hsl(var(--popover-foreground))"
				},
				card: {
					DEFAULT: "hsl(var(--card))",
					foreground: "hsl(var(--card-foreground))"
				}
			},
			borderRadius: {
				lg: "var(--radius)",
				md: "calc(var(--radius) - 2px)",
				sm: "calc(var(--radius) - 4px)"
			}
		}
	},
	plugins: [
		require("@tailwindcss/forms"),
		// Allows prefixing tailwind classes with LiveView classes to add rules
		// only when LiveView classes are applied, for example:
		//
		//     <div class="phx-click-loading:animate-ping">
		//
		plugin(({ addVariant }) =>
			addVariant("phx-no-feedback", [
				".phx-no-feedback&",
				".phx-no-feedback &"
			])
		),
		plugin(({ addVariant }) =>
			addVariant("phx-click-loading", [
				".phx-click-loading&",
				".phx-click-loading &"
			])
		),
		plugin(({ addVariant }) =>
			addVariant("phx-submit-loading", [
				".phx-submit-loading&",
				".phx-submit-loading &"
			])
		),
		plugin(({ addVariant }) =>
			addVariant("phx-change-loading", [
				".phx-change-loading&",
				".phx-change-loading &"
			])
		),

		// Embeds Heroicons (https://heroicons.com) into your app.css bundle
		// See your `CoreComponents.icon/1` for more information.
		//
		plugin(function ({ matchComponents, theme }) {
			let iconsDir = path.join(
				__dirname,
				"../deps/heroicons/optimized"
			);
			let values = {};
			let icons = [
				["", "/24/outline"],
				["-solid", "/24/solid"],
				["-mini", "/20/solid"],
				["-micro", "/16/solid"]
			];
			icons.forEach(([suffix, dir]) => {
				fs.readdirSync(path.join(iconsDir, dir)).forEach(
					(file) => {
						let name =
							path.basename(file, ".svg") + suffix;
						values[name] = {
							name,
							fullPath: path.join(iconsDir, dir, file)
						};
					}
				);
			});
			matchComponents(
				{
					hero: ({ name, fullPath }) => {
						let content = fs
							.readFileSync(fullPath)
							.toString()
							.replace(/\r?\n|\r/g, "");
						let size = theme("spacing.6");
						if (name.endsWith("-mini")) {
							size = theme("spacing.5");
						} else if (name.endsWith("-micro")) {
							size = theme("spacing.4");
						}
						return {
							[`--hero-${name}`]: `url('data:image/svg+xml;utf8,${content}')`,
							"-webkit-mask": `var(--hero-${name})`,
							mask: `var(--hero-${name})`,
							"mask-repeat": "no-repeat",
							"background-color": "currentColor",
							"vertical-align": "middle",
							display: "inline-block",
							width: size,
							height: size
						};
					}
				},
				{ values }
			);
		})
	]
};
