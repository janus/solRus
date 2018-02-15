const test = require("blue-tape");

const startCal = require("./startCal.js");

//const Stores = artifacts.require("./Store.sol");
const Pays = artifacts.require("./PaymentChannels.sol");

(async () => {
  //const instance = await Stores.deployed();
  const instance = await Pays.deployed();
  startCal(test, instance);
})();
