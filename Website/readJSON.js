var JSONobject = JSON.parse('{ "continent": {"europe":[ {"x":0, "y":5}, {"x":10, "y":10} ], "africa":[{"x":0, "y":0}, {"x":10, "y":5} ]} , "age":30, "city":"New York"}');


function getDataset (majorCat, minorCat) {
	return {label: minorCat, data: JSONobject[majorCat][minorCat]};
}

function getData(majorCat) {
	var datasets = [];
	var label;
	for(label in JSONobject[majorCat]) {
		datasets.push(getDataset(majorCat,label));
	}
	return datasets;
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
console.log(getData("continent"));

