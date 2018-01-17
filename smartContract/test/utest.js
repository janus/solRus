
module.exports = {fetchData, getData, verifySign1, verifySign2};

var netData;

function makeAjaxCall(url, methodType) {
    var promiseObj = new Promise(function (resolve, reject) {
        var xhr = new XMLHttpRequest();
        if (!xhr) {
              console.log('Giving up :( Cannot create an XMLHTTP instance');
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



async function fetchData() {
    var URL = 'http://127.0.0.1:3030/';
    await makeAjaxCall(URL, 'POST').then(processResponse, errorHandler);

}

function processResponse(responseData) {
    var respJson = JSON.parse(responseData);
    var resData = JSON.parse(respJson.result);
    netData = resData;
    
}

async function getData() {
    return netData;
}

function verifySign1(resData, instance) {
    const hex_msg_1 = '0x' + resData.msg_hash1;
    
    //signature 1
    const sign1 = resData.sig1;
    const r1 = '0x' + sign1.slice(0, 64);
    const s1 = '0x' + sign1.slice(64, 128);
    const v1 = sign1.slice(128, 130);
    //console.log(sign1.length);
    const v_decimal_1 = 27 + parseInt(v1) ; // parseInt(v1);

    return instance.recoverAddr.call(hex_msg_1, v_decimal_1, r1,s1);
}


function verifySign2(resData, instance) {
    const hex_msg_2 = '0x' + resData.msg_hash2;
    

    //signature 2
    const sign2 = resData.sig2;
    const r2 = '0x' + sign2.slice(0, 64);
    const s2 = '0x' + sign2.slice(64, 128);
    const v2 = sign2.slice(128, 130);
    const v_decimal_2 = 27 + parseInt(v2);
    

    return instance.recoverAddr.call(hex_msg_2, v_decimal_2, r2,s2);
}



function errorHandler(status) {
    console.log("Bad Network or Spurious Request", status);
}




