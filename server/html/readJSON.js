var JSONobject = JSON.parse('{"Continent":{"Europe":[{"x":1.0,"y":5.0}],"Africa":[],"SouthAmerica":[],"NorthAmerica":[],"Antarctica":[],"Asia":[{"x":3.0,"y":6.0}],"Australia":[]},"Age":{"EighteenToThirty":[{"x":3.0,"y":6.0}],"ThirtyToFifty":[],"ThirteenToEighteen":[],"UnderThirteen":[{"x":1.0,"y":5.0}],"FiftyAndOlder":[]},"Gender":{"Male":[],"Female":[{"x":1.0,"y":5.0}],"Other":[{"x":3.0,"y":6.0}]}}');
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

console.log(getData("Continent"));

