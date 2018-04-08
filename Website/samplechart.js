var ctx = document.getElementById("canvas").getContext('2d');
var myChart = new Chart(ctx, {
    type: 'line',
    data: {  
        datasets: getData("continent")
    },
    options: {
        scales: {
            yAxes: [{
                ticks: {
                    beginAtZero:true
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



