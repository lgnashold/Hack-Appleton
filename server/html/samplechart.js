var ctx = document.getElementById("canvas").getContext('2d');
var colors = ["red","blue","yellow","purple", "green","orange"];
var myChart;
setFocus("Gender");

function setFocus(category) {
    myChart = new Chart(ctx, {
    type: 'line',
    data: {  
        datasets: getAllDatasets(category, getColors(getMinorCategorys(category).length))
    },
    options: {
        responsive: true,
        title: {
            display: true,
            text: 'Chart.js Line Chart'
        },
        tooltips: {
            mode: 'index',
            intersect: false,
        },
        hover: {
            mode: 'nearest',
            intersect: true
        },
        scales: {
            xAxes: [{
                display: true,
                scaleLabel: {
                    display: true,
                    labelString: 'Time'
                },
                ticks: {
                    beginAtZero: true
                }
            }],
            yAxes: [{
                display: true,
                scaleLabel: {
                    display: true,
                    labelString: 'Purchases'
                },
                ticks: {
                    beginAtZero: true
                }
            }],
	    xAxes: [{
		ticks: {
		    beginAtZero:true
		}
	    }]
		
        }
    }
});
}

function getColors(length) {
    console.log(colors.slice(0, length));
    return colors.slice(0, length);
}


