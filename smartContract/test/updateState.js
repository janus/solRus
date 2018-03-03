// cook mango twist then skin sort option civil have still rather guilt

const test = require("blue-tape");
const p = require("util").promisify;

const {
  filterLogs,
  takeSnapshot,
  revertSnapshot,
  solSha3,
  sign,
  mineBlocks,
  createChannel,
  updateState,
  getSettlingData,
  getSignBlocks,
  getData,
  closeChannel,
  startSettlingPeriod
} = require("./utils.js");

module.exports = async (test, instance) => {
  test("updateState happy path", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);
    const eventLog = instance.allEvents();

    await instance.depositToAddress.sendTransaction(data.address_0, {
      value: 22000
    });
    await instance.depositToAddress.sendTransaction(data.address_1, {
      value: 22000
    });

    await instance.newChannel(
      "0x" + data.channel_id,
      data.address_0,
      data.address_1,
      data.balance_0,
      data.balance_1,
      data.settling_period_length,
      data.sign_priv_0,
      data.sign_priv_1
    );

    await updateState(instance, data1, signs, "0x");

    t.deepEqual(
      JSON.parse(
        JSON.stringify(await instance.channels("0x" + data.channel_id))
      ),
      [
        "0x" + data.channel_id,
        data.address_0,
        data.address_1,
        "30000",
        "17000",
        "13000",
        "0x",
        "1",
        "2",
        false,
        "0",
        false
      ]
    );

    eventLog.stopWatching();

    await revertSnapshot(snapshot);
  });

  test("updateState nonexistant channel", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(3);
    const data1 = await getSettlingData(3);
    const signs = await getSignBlocks(1);

    let hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.bad_channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0,
        signs.sign_priv_1
      )
    );

    await revertSnapshot(snapshot);
  });

  test("channel closed before updateState", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    let hashlocks = "0x";

    await closeChannel(instance, data, data1, signs, hashlocks);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0,
        signs.sign_priv_1
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState low seq #", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    let hashlocks = "0x";
    await createChannel(instance, data);
    await updateState(instance, data1, signs, "0x");

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0_wrong_seq_num,
        signs.sign_priv_1_wrong_seq_num
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (string)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const signs = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0_bad_msg,
        signs.sign_priv_1_bad_msg
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (channelID)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const signs = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0_wrong_id,
        signs.sign_priv_1_wrong_id
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (sequenceNumber)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const signs = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0_wrong_seq_num,
        signs.sign_priv_1_wrong_seq_num
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (balance)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const signs = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0_wrong_balance,
        signs.sign_priv_1_wrong_balance
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (hashlocks)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const signs = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0_bad_hashlocks,
        signs.sign_priv_1_bad_hashlocks
      )
    );

    await revertSnapshot(snapshot);
  });

  test("private key used twice", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const signs = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_0,
        signs.sign_priv_0
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState wrong private key", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const signs = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        hashlocks,
        signs.sign_priv_1,
        signs.sign_priv_1
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateStateWithBounty happy path", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    let hashlocks = "0x";

    await createChannel(instance, data);
    await startSettlingPeriod(instance, data1);

    await instance.updateStateWithBounty(
      "0x" + data1.channel_id,
      data1.seq_num,
      data1.balance_0,
      data1.balance_1,
      hashlocks,
      signs.sign_priv_0,
      signs.sign_priv_1,
      "0x0000000000000000000000000000000000000000000000000000000000000002",
      signs.bounty_sign,
      { from: web3.eth.accounts[0] }
    );

    t.equal((await instance.balanceOf.call(data.address_0)).toString(), "6998");

    const channel = JSON.parse(
      JSON.stringify(await instance.channels("0x" + data.channel_id))
    );

    t.deepEqual(channel, [
      "0x" + data.channel_id,
      data.address_0,
      data.address_1,
      "30000",
      "17000",
      "13000",
      "0x",
      "1",
      "2",
      true,
      channel[10],
      false
    ]);

    await revertSnapshot(snapshot);
  });

  test("updateStateWithBounty settlingPeriod not started", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const signs = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateStateWithBounty(
        "0x" + data.channel_id,
        data1.seq_num,
        data.address_0,
        data.address_1,
        hashlocks,
        signs.sign_priv_0,
        signs.sign_priv_1,
        "0x0000000000000000000000000000000000000000000000000000000000000002",
        signs.bounty_sign,
        { from: web3.eth.accounts[1] }
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateStateWithBounty bad sig", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    const hashlocks = "0x";

    await createChannel(instance, data);
    await startSettlingPeriod(instance, data1);

    await t.shouldFail(
      instance.updateStateWithBounty(
        "0x" + data.channel_id,
        data1.seq_num,
        data.address_0,
        data.address_1,
        hashlocks,
        signs.sign_priv_0,
        signs.sign_priv_1,
        "0x0000000000000000000000000000000000000000000000000000000000000002",
        signs.bounty_sign_bad_msg,
        { from: web3.eth.accounts[2] }
      )
    );

    await revertSnapshot(snapshot);
  });
};
