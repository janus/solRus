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
  startSettlingPeriod
} = require("./utils.js");

module.exports = async (test, instance) => {
  test("updateState happy path", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const eventLog = instance.allEvents();

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

    await updateState(instance, data1, "0x");

    t.deepEqual(
      JSON.parse(JSON.stringify(await instance.channels("0x" + data.chl_id))),
      [
        "0x" + data.chl_id,
        data.addr_0,
        data.addr_1,
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
    const sign1 = await getSignBlocks(1);

    let hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.chl_id_wg,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sign_1,
        sign1.sig_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("channel closed before updateState", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const sign1 = await getSignBlocks(1);

    let hashlocks = "0x";
    await createChannel(instance, data);
    await startSettlingPeriod(instance, data1);
    await mineBlocks(5);

    await t.shouldFail(
      instance.updateState(
        "0x" + data.chl_id,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sign_1,
        sign1.sign_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState low seq #", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const sign1 = await getSignBlocks(1);

    let hashlocks = "0x";
    await createChannel(instance, data);
    await updateState(instance, data1, "0x");

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.chl_id_wg,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sig_st_sqn_1,
        sign1.sig_st_sqn_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (string)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const sign1 = await getSignBlocks(0);

    const hashlocks = "0x";

    //await createChannel(instance, data);
    // already existing

    await t.shouldFail(
      instance.updateState(
        "0x" + data.chl_di,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sig_st_derp_1,
        sign1.sig_st_derp_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (channelID)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const sign1 = await getSignBlocks(0);

    const hashlocks = "0x";

    //await createChannel(instance, data);
    // already existing

    await t.shouldFail(
      instance.updateState(
        "0x" + data.chl_di,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sig_st_id_1,
        sign1.sig_st_id_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (sequenceNumber)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const sign1 = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);
    // already existing

    await t.shouldFail(
      instance.updateState(
        "0x" + data.chl_di,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sig_st_sqn_1,
        sign1.sig_st_sqn_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (balance)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const sign1 = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data.chl_di,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sig_st_bl_1,
        sign1.sig_st_bl_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState bad fingerprint (hashlocks)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const sign1 = await getSignBlocks(0);

    const hashlocks = "0x";

    //await createChannel(instance, data);
    // already existing

    await t.shouldFail(
      instance.updateState(
        "0x" + data.chl_di,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sig_st_hl_1,
        sign1.sig_st_hl_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("private key used twice", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const sign1 = await getSignBlocks(0);

    const hashlocks = "0x";

    //await createChannel(instance, data);
    // already existing

    await t.shouldFail(
      instance.updateState(
        "0x" + data.chl_di,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sign_1,
        sign1.sign_1
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateState wrong private key", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);
    const sign1 = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data.chl_di,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sign_1,
        sign1.sign_2
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateStateWithBounty happy path", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const sign1 = await getSignBlocks(1);

    let hashlocks = "0x";

    await createChannel(instance, data);
    await startSettlingPeriod(instance, data1);

    await instance.updateStateWithBounty(
      "0x" + data1.chl_id,
      data1.seq_num,
      data1.bal_0,
      data1.bal_1,
      hashlocks,
      sign1.sign_1,
      sign1.sign_2,
      "0x0000000000000000000000000000000000000000000000000000000000000002",
      sign1.sign_bt,
      { from: web3.eth.accounts[0] }
    );

    t.equal((await instance.balanceOf.call(data.addr_0)).toString(), "6998");

    const channel = JSON.parse(
      JSON.stringify(await instance.channels("0x" + data.chl_id))
    );

    t.deepEqual(channel, [
      "0x" + data.chl_id,
      data.addr_0,
      data.addr_1,
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
    const sign1 = await getSignBlocks(0);

    const hashlocks = "0x";

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateStateWithBounty(
        "0x" + data1.chl_id,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sign_1,
        sign1.sign_2,
        "0x0000000000000000000000000000000000000000000000000000000000000002",
        sign1.sign_bt,
        { from: web3.eth.accounts[1] }
      )
    );

    await revertSnapshot(snapshot);
  });

  test("updateStateWithBounty bad sig", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const sign1 = await getSignBlocks(1);

    const hashlocks = "0x";

    await createChannel(instance, data);
    await startSettlingPeriod(instance, data1);

    await t.shouldFail(
      instance.updateStateWithBounty(
        "0x" + data1.chl_id,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        hashlocks,
        sign1.sign_1,
        sign1.sign_2,
        "0x0000000000000000000000000000000000000000000000000000000000000002",
        sign1.sign_bt_bd,
        { from: web3.eth.accounts[2] }
      )
    );

    await revertSnapshot(snapshot);
  });
};
