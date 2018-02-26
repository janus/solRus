const leftPad = require("left-pad");
const p = require("util").promisify;
const ethUtils = require("ethereumjs-util");
const BN = require("bn.js");
const Web3 = require("web3");

const wb3 = new Web3(new Web3.providers.HttpProvider("http://localhost:3030"));

module.exports = {
  sleep,
  extUint256,
  takeSnapshot,
  getSettlingData,
  getSignBlocks,
  getData,
  getSpData,
  revertSnapshot,
  solSha3,
  sign,
  ecrecover,
  filterLogs,
  mineBlocks,
  createChannel,
  closeChannelWithoutNewChannel,
  updateState,
  startSettlingPeriod,
  toSolUint256,
  toSolInt256,
  closeChannel,
  closeChannelSp
};

function sleep(time) {
  return new Promise(resolve => {
    setTimeout(resolve, time);
  });
}

let snapshotInc = 0;

async function takeSnapshot() {
  let res = await p(web3.currentProvider.sendAsync.bind(web3.currentProvider))({
    jsonrpc: "2.0",
    method: "evm_snapshot",
    id: snapshotInc++
  });
  return res.result;
}

async function revertSnapshot(snapshotId) {
  await p(web3.currentProvider.sendAsync.bind(web3.currentProvider))({
    jsonrpc: "2.0",
    method: "evm_revert",
    params: [snapshotId],
    id: snapshotInc++
  });
}

async function rpcData() {
  let res = await p(wb3.currentProvider.sendAsync.bind(wb3.currentProvider))({
    jsonrpc: "2.0",
    method: "rpc_data",
    id: 2
  });

  return res;
}

async function rpcSigns() {
  let res = await p(wb3.currentProvider.sendAsync.bind(wb3.currentProvider))({
    jsonrpc: "2.0",
    method: "rpc_signs",
    id: 3
  });

  return res;
}

async function rpcUpdate() {
  let res = await p(wb3.currentProvider.sendAsync.bind(wb3.currentProvider))({
    jsonrpc: "2.0",
    method: "rpc_update",
    id: 4
  });

  return res;
}

async function rpcUpdateSp() {
  let res = await p(wb3.currentProvider.sendAsync.bind(wb3.currentProvider))({
    jsonrpc: "2.0",
    method: "rpc_update_sp",
    id: 4
  });

  return res;
}

async function getData(idx) {
  let rst = await rpcData();
  let resData = JSON.parse(rst.result[idx]);
  return resData;
}

async function getSettlingData(idx) {
  let rst = await rpcUpdate();
  let resData = JSON.parse(rst.result[idx]);
  return resData;
}

async function getSpData(idx) {
  let rst = await rpcUpdateSp();
  let resData = JSON.parse(rst.result[idx]);
  return resData;
}

async function getSignBlocks(idx) {
  let rst = await rpcSigns();
  let resData = JSON.parse(rst.result[idx]);
  return resData;
}

async function mineBlock() {
  await p(web3.currentProvider.sendAsync.bind(web3.currentProvider))({
    jsonrpc: "2.0",
    method: "evm_mine",
    id: new Date().getTime()
  });
}

async function mineBlocks(count) {
  let i = 0;
  while (i < count) {
    await mineBlock();
    i++;
  }
}

async function extUint256(instance) {
  let gg = await instance.reUint(1200);
  return gg;
}

function toSolUint256(num) {
  return leftPad(num.toString(16), 64, 0);
}

function toSolInt256(num) {
  return new BN(num).toTwos(256).toString(16, 64);
}

function solSha3(...args) {
  args = args.map(arg => {
    if (typeof arg === "string") {
      if (arg.substring(0, 2) === "0x") {
        return arg.slice(2);
      } else {
        return web3.toHex(arg).slice(2);
      }
    }

    if (typeof arg === "number") {
      return leftPad(arg.toString(16), 64, 0);
    }
  });

  args = args.join("");

  return web3.sha3(args, { encoding: "hex" });
}

function sign(msgHash, privKey) {
  if (typeof msgHash === "string" && msgHash.slice(0, 2) === "0x") {
    msgHash = Buffer.alloc(32, msgHash.slice(2), "hex");
  }
  const sig = ethUtils.ecsign(msgHash, privKey);
  return `0x${sig.r.toString("hex")}${sig.s.toString("hex")}${sig.v.toString(
    16
  )}`;
}

function ecrecover(msg, sig) {
  const r = ethUtils.toBuffer(sig.slice(0, 66));
  const s = ethUtils.toBuffer("0x" + sig.slice(66, 130));
  const v = 27 + parseInt(sig.slice(130, 132));
  const m = ethUtils.toBuffer(msg);
  const pub = ethUtils.ecrecover(m, v, r, s);
  return "0x" + ethUtils.pubToAddress(pub).toString("hex");
}

function filterLogs(logs) {
  return logs.map(log => [log.event, log.args]);
}

async function createChannel(instance, data) {
  await instance.depositToAddress.sendTransaction(data.addr_0, {
    value: 22000
  });
  await instance.depositToAddress.sendTransaction(data.addr_1, {
    value: 22000
  });

  await instance.newChannel(
    "0x" + data.chl_id,
    data.addr_0,
    data.addr_1,
    data.bal_0,
    data.bal_1,
    data.set_period_ln,
    data.sig_0,
    data.sig_1
  );
}

async function updateState(instance, data, hashlocks) {
  await instance.updateState(
    "0x" + data.chl_id,
    data.seq_num,
    data.bal_0,
    data.bal_1,
    hashlocks,
    data.sig_0,
    data.sig_1
  );
}

async function startSettlingPeriod(instance, data) {
  await instance.startSettlingPeriod("0x" + data.chl_id, data.sig_start_stl_p);
}

async function closeChannel(instance, data, data1, hashlocks) {
  await createChannel(instance, data);
  await updateState(instance, data1, hashlocks);
  await startSettlingPeriod(instance, data1);
  await mineBlocks(5);
  await instance.closeChannel("0x" + data.chl_id);
}

async function closeChannelWithoutNewChannel(instance, data1, hashlocks) {
  await updateState(instance, data1, hashlocks);
  await startSettlingPeriod(instance, data1);

  await mineBlocks(5);
  await instance.closeChannel("0x" + data1.chl_id);
}

async function closeChannelSp(instance, data, data1, hashlocks) {
  await instance.newChannel(
    "0x" + data.chl_id,
    data.addr_0,
    data.addr_1,
    data.bal_0,
    data.bogus_amount,
    data.set_period_ln,
    data.sig_0,
    data.sig_1
  );

  await updateState(instance, data1, hashlocks);
  await startSettlingPeriod(instance, data1);
  await mineBlocks(5);
  await instance.closeChannel("0x" + data.chl_id);
}
