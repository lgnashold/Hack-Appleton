//Edits the HTML of the Analytics page to update the second navbar to have an element for each major category
//This creates the "Gender Age Continent" navbar currently, but would dynamically update if the JSON included more categories

var categories = getMajorCategories();

var output = "";

for(var i = 0; i < categories.length; i++) {
	output += "<li><a onclick=\"setFocus('" + categories[i] + "')\">" + categories[i] + "</a></li>";
}

document.getElementById("ca").innerHTML = output;