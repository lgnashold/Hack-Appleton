// Sending a receiving data in JSON format using GET method
//
var xhr = new XMLHttpRequest();
var url = "plots.json";

xhr.setRequestHeader("Content-Type", "application/json");
xhr.onreadystatechange = function () {
   if (xhr.readyState === 4 && xhr.status === 200) {
        var json = JSON.parse(xhr.responseText);
       console.log(json.email + ", " + json.password);
       
    }
};
xhr.open("GET", url, true);
xhr.send();
