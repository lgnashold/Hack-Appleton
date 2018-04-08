var JSONobject = JSON.parse('{ "continent": {"europe":[ {"x":10, "y":11}, {"x":10} ], "africa":20} , "age":30, "city":"New York"}');


function getDataset (minorCat) {
	return JSONobject[minorCat];
}

function getMinorCategorys(majorCat) {
	var minorCats = [];
	var label;
	for(label in JSONobject[majorCat]) {
		minorCats.push(label);
	}
	return minorCats;
}

console.log(getMinorCategorys("continent"));