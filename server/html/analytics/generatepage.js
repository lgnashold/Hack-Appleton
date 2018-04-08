var categories = getMajorCategorys();

var output = "";

for(var i = 0; i < categories.length; i++) {
	output += "<li><a onclick=\"setFocus('" + categories[i] + "')\">" + categories[i] + "</a></li>";
}

document.getElementById("ca").innerHTML = output;