//var Store = artifacts.require("./Store.sol");
var PaymentChannels = artifacts.require("./PaymentChannels.sol");
//var MetaCoin = artifacts.require("./MetaCoin.sol");

module.exports = function(deployer) {
  deployer.deploy(PaymentChannels);
  //deployer.link(ConvertLib, MetaCoin);
  //deployer.deploy(MetaCoin);
};
