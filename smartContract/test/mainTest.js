const test = require("blue-tape");

const startCal = require("./startCal.js");

const Stores = artifacts.require("../contracts/Store.sol");

(async () => {
  const instance = await Stores.deployed();
  startCal(test, instance);
})();
