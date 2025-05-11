window.MathJax = {
	options: {
		enableMenu: false,
		ignoreHtmlClass: "tex2jax_ignore",
		processHtmlClass: "tex2jax_process"
	},
	tex: {
		inlineMath: [
			["$", "$"],
			["$$", "$$"]
		],
		displayMath: [["$$$", "$$$"]]
	},
	loader: {
		load: ["input/asciimath", "[tex]/noerrors", "ui/lazy"]
	},
	chtml: { mtextInheritFont: true, minScale: 1, exFactor: 1 }
};
