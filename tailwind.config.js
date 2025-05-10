import franken from "franken-ui/shadcn-ui/preset-quick";

/** @type {import('tailwindcss').Config} */
export default {
	presets: [
		franken({
			customPalette: {
				".uk-theme-raesan": {
					"--background": "347 2% 95%",
					"--foreground": "347 2% 1%",
					"--card": "347 2% 90%",
					"--card-foreground": "347 2% 10%",
					"--popover": "347 2% 95%",
					"--popover-foreground": "347 95% 1%",
					"--primary": "347 15% 23%",
					"--primary-foreground": "0 0% 100%",
					"--secondary": "347 10% 70%",
					"--secondary-foreground": "0 0% 0%",
					"--muted": "309 10% 85%",
					"--muted-foreground": "347 2% 35%",
					"--accent": "309 10% 80%",
					"--accent-foreground": "347 2% 10%",
					"--destructive": "0 50% 30%",
					"--destructive-foreground": "347 2% 90%",
					"--border": "347 20% 50%",
					"--input": "347 20% 18%",
					"--ring": "347 15% 23%",
					"--radius": "1rem"
				},
				".dark.uk-theme-raesan": {
					"--background": "347 10% 5%",
					"--foreground": "347 2% 90%",
					"--card": "347 2% 1%",
					"--card-foreground": "347 2% 90%",
					"--popover": "347 10% 5%",
					"--popover-foreground": "347 2% 90%",
					"--primary": "347 15% 23%",
					"--primary-foreground": "0 0% 100%",
					"--secondary": "347 10% 10%",
					"--secondary-foreground": "0 0% 100%",
					"--muted": "309 10% 15%",
					"--muted-foreground": "347 2% 60%",
					"--accent": "309 10% 15%",
					"--accent-foreground": "347 2% 90%",
					"--destructive": "0 50% 30%",
					"--destructive-foreground": "347 2% 90%",
					"--border": "347 20% 18%",
					"--input": "347 20% 18%",
					"--ring": "347 15% 23%",
					"--radius": "1em"
				}
			}
		})
	],
	content: ["./src/server/web/**/*.rs"],
	safelist: [
		{
			pattern: /^uk-/
		},
		"ProseMirror",
		"ProseMirror-focused",
		"tiptap",
		"mr-2",
		"mt-2",
		"opacity-50"
	],
	theme: {
		extend: {}
	},
	plugins: []
};
