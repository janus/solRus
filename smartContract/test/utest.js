const p = require("util").promisify;
//const BN = require("bn.js");
//const bn = require("bignumber.js");
//const leftPad = require("left-pad");

const Web3 = require('web3');

const web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:3030"));


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
    console.log(resData);
    netData = resData;
    
}

async function getData() {
    return netData;
}

function verifySign1(resData, instance) {
    const  lenNum1 = resData.num1.length;
    const num1 = resData.num1.substr(1,lenNum1 - 2);
    const  lenNum2 = resData.num2.length;
    const num2 = resData.num2.substr(1,lenNum2 - 2);
      
    //var hash = web3.sha3(toSolInt256(num1)+ toSolInt256(num2));
    //var hash = web3.sha3(web3.toHex(1), {encoding: 'hex'});
    
    const address = '0x' + resData.address;
    //signature 1
    const sign1 = resData.sig;
    const r1 = '0x' + sign1.slice(0, 64);
    const s1 = '0x' + sign1.slice(64, 128);
    const v1 = sign1.slice(128, 130);
    //console.log(sign1.length);
    const v_decimal_1 = 27 + parseInt(v1) ;

    return instance.isSigned.call(address, num1, num2, v_decimal_1, r1,s1);
}


function verifySign2(resData, instance) {
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

/*
async function takeSnapshot() {
  let res = await p(web3.currentProvider.sendAsync.bind(web3.currentProvider))({
    jsonrpc: "2.0",
    method: "evm_snapshot",
    id: snapshotInc++
  });
  return res.result;
}

function toSolUint256(num) {
  return leftPad(num.toString(16), 64, 0);
}

function toSolInt256(num) {
  return new BN(num).toTwos(256).toString(16, 64);
}
*/