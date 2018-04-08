var ctx = document.getElementById("canvas").getContext('2d');
var colors = ["red","blue","yellow","purple", "green","orange"];
var myChart;
setFocus("Gender");

function setFocus(category) {
    myChart = new Chart(ctx,{type: 'line',
            data: {
                datasets: getAllDatasets(category, getColors(getMinorCategorys(category).length))
            },
            options: {
                responsive: true,
                title: {
                    display: true,
                    text: category + " vs time"
                },
                scales: {
                    xAxes: [{
                        type:"linear",
                        title:"Time",
                        scaleLabel: {
                            display: true,
                            labelString: "Time"
                        }
                        
                    }],
                    yAxes: [{
                        ticks: {
                            min:0
                        },
                        scaleLabel: {
                            display: true,
                            labelString: "Purchases made by demographic"
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