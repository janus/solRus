
module.exports = { fetchData, returnVal };

var returnVal; 

function makeAjaxCall(url, methodType) {
    var promiseObj = new Promise(function (resolve, reject) {
    var xhr = new XMLHttpRequest();
    if (!xhr) {
	      alert('Giving up :( Cannot create an XMLHTTP instance');
	      return false;
    }
    xhr.open(methodType, url);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(JSON.stringify({jsonrpc: "2.0", id: "1",  method: "say_hello"}));
    xhr.onreadystatechange = function() {
    if (xhr.readyState === 4) {
         if (xhr.status === 200) {
            resolve(xhr.responseText);
         } else {
            reject(xhr.status);
         }
    } else {
         console.log("xhr processing going on");
        }
    }
   
    console.log("request sent succesfully");
        });

    return promiseObj;
}


function fetchData() {
    var URL = 'http://127.0.0.1:3030/';
    makeAjaxCall(URL, 'POST').then(processResponse, errorHandler);

}

function processResponse(responseData) {
    var respJson = JSON.parse(responseData);
    var resData = JSON.parse(respJson.result);
    returnVal = resData;
    alert(resData);
}


function errorHandler(status) {
    alert("Bad Network or Spurious Request", status);
}






