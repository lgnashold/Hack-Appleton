//Major Categories define graphs, (i.e. Gender, Continent, Age)
//Minor Categories are the categories within the major demographics, (i.e. Female, Male, Other)

var JSONobject = JSON.parse('{"Gender":{"Male":[],"Other":[{"x":14.0,"y":3.0},{"x":15.0,"y":3.0},{"x":16.0,"y":8.0},{"x":17.0,"y":3.0}],"Female":[{"x":14.0,"y":4.0},{"x":15.0,"y":9.0},{"x":16.0,"y":10.0},{"x":17.0,"y":10.0}]},"Age":{"ThirteenToEighteen":[{"x":15.0,"y":1.0},{"x":16.0,"y":5.0},{"x":17.0,"y":1.0}],"EighteenToThirty":[{"x":14.0,"y":1.0},{"x":15.0,"y":3.0},{"x":16.0,"y":2.0},{"x":17.0,"y":6.0}],"ThirtyToFifty":[{"x":14.0,"y":1.0},{"x":15.0,"y":1.0},{"x":16.0,"y":4.0},{"x":17.0,"y":1.0}],"FiftyAndOlder":[{"x":14.0,"y":5.0},{"x":15.0,"y":6.0},{"x":16.0,"y":3.0},{"x":17.0,"y":4.0}],"UnderThirteen":[{"x":15.0,"y":1.0},{"x":16.0,"y":4.0},{"x":17.0,"y":1.0}]},"Continent":{"NorthAmerica":[{"x":14.0,"y":2.0},{"x":15.0,"y":2.0},{"x":17.0,"y":1.0}],"Africa":[{"x":14.0,"y":1.0},{"x":15.0,"y":2.0},{"x":16.0,"y":3.0},{"x":17.0,"y":3.0}],"Asia":[{"x":14.0,"y":1.0},{"x":16.0,"y":4.0},{"x":17.0,"y":1.0}],"Europe":[{"x":14.0,"y":1.0},{"x":15.0,"y":1.0},{"x":16.0,"y":2.0},{"x":17.0,"y":1.0}],"SouthAmerica":[{"x":14.0,"y":2.0},{"x":15.0,"y":5.0},{"x":16.0,"y":2.0},{"x":17.0,"y":5.0}],"Australia":[{"x":15.0,"y":1.0},{"x":16.0,"y":4.0}],"Antarctica":[{"x":15.0,"y":1.0},{"x":16.0,"y":3.0},{"x":17.0,"y":2.0}]}}');


//Returns one dataset object that has several graphical componenets
//data connects to the JSONobject and indexes to a specific category and subcategoy
function getDataset (majorCat, minorCat, color) {
	return {
		label: minorCat, data: JSONobject[majorCat][minorCat],
		fill:false,borderWidth:5,showLines:false,borderColor:color
	};
}

//Returns a list of datasets in a Major Category (i.e. returning all the data for the Gender category)
//majorCat is the major category to get the datasets from
//colorList is all the possible colors, colors are assigned to each dataset sequentially

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


//Returns a list of all major categories in the JSON
function getMajorCategories() {
	var majorCats = [];
	var label;
	for(label in JSONobject) {
		majorCats.push(label);
	}
	return majorCats;
}

//Returns a list of all minor categories under a major category
function getMinorCategories(majorCat) {
	var minorCats = [];
	var label;
	for(label in JSONobject[majorCat]) {
		minorCats.push(label);
	}
	return minorCats;
}
