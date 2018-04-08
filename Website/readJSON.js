var JSONobject = JSON.parse('{"Gender":{"Male":[],"Other":[{"x":14.0,"y":3.0},{"x":15.0,"y":3.0},{"x":16.0,"y":8.0},{"x":17.0,"y":3.0}],"Female":[{"x":14.0,"y":4.0},{"x":15.0,"y":9.0},{"x":16.0,"y":10.0},{"x":17.0,"y":10.0}]},"Age":{"ThirteenToEighteen":[{"x":15.0,"y":1.0},{"x":16.0,"y":5.0},{"x":17.0,"y":1.0}],"EighteenToThirty":[{"x":14.0,"y":1.0},{"x":15.0,"y":3.0},{"x":16.0,"y":2.0},{"x":17.0,"y":6.0}],"ThirtyToFifty":[{"x":14.0,"y":1.0},{"x":15.0,"y":1.0},{"x":16.0,"y":4.0},{"x":17.0,"y":1.0}],"FiftyAndOlder":[{"x":14.0,"y":5.0},{"x":15.0,"y":6.0},{"x":16.0,"y":3.0},{"x":17.0,"y":4.0}],"UnderThirteen":[{"x":15.0,"y":1.0},{"x":16.0,"y":4.0},{"x":17.0,"y":1.0}]},"Continent":{"NorthAmerica":[{"x":14.0,"y":2.0},{"x":15.0,"y":2.0},{"x":17.0,"y":1.0}],"Africa":[{"x":14.0,"y":1.0},{"x":15.0,"y":2.0},{"x":16.0,"y":3.0},{"x":17.0,"y":3.0}],"Asia":[{"x":14.0,"y":1.0},{"x":16.0,"y":4.0},{"x":17.0,"y":1.0}],"Europe":[{"x":14.0,"y":1.0},{"x":15.0,"y":1.0},{"x":16.0,"y":2.0},{"x":17.0,"y":1.0}],"SouthAmerica":[{"x":14.0,"y":2.0},{"x":15.0,"y":5.0},{"x":16.0,"y":2.0},{"x":17.0,"y":5.0}],"Australia":[{"x":15.0,"y":1.0},{"x":16.0,"y":4.0}],"Antarctica":[{"x":15.0,"y":1.0},{"x":16.0,"y":3.0},{"x":17.0,"y":2.0}]}}');
function getDataset (majorCat, minorCat, color) {
	return {
		label: minorCat, data: JSONobject[majorCat][minorCat],
		fill:false,borderWidth:5,borderColor:color
	};
}

function getAllDatasets(majorCat, colorList) {
	var datasets = [];
	var label;
	var count = 0;
	for(label in JSONobject[majorCat]) {
		datasets.push(getDataset(majorCat,label,colorList[count]));
		count++;
	}
	return datasets;
}

function getMajorCategorys() {
	var majorCats = [];
	var label;
	for(label in JSONobject) {
		majorCats.push(label);
	}
	return majorCats;
}

function getMinorCategorys(majorCat) {
	var minorCats = [];
	var label;
	for(label in JSONobject[majorCat]) {
		minorCats.push(label);
	}
	return minorCats;
}

console.log(getMajorCategorys());

