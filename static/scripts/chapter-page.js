// all the custom logic related to the "Chapter Page" will go into this script

document.addEventListener("DOMContentLoaded", () => {
	document.querySelectorAll("table").forEach((table) => {
		table.classList.add("uk-table");
		table.classList.add("uk-table-divider");
	});
});
