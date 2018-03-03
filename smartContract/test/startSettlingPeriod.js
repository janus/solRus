// cook mango twist then skin sort option civil have still rather guilt

const test = require("blue-tape");
const p = require("util").promisify;

const {
  filterLogs,
  takeSnapshot,
  revertSnapshot,
  getData,
  getSettlingData,
  getSpData,
  createChannel,
  updateState,
  getSignBlocks,
  startSettlingPeriod
} = require("./utils.js");

module.exports = async (test, instance) => {
  test("startSettlingPeriod nonexistant channel", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.bad_channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        "0x",
        signs.sign_priv_0,
        signs.sign_priv_1
      )
    );

    await revertSnapshot(snapshot);
  });

  test("startSettlingPeriod already started", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await createChannel(instance, data);
    await updateState(instance, data1, signs, "0x");
    await startSettlingPeriod(instance, data1);
    await t.shouldFail(startSettlingPeriod(instance, data));

    await revertSnapshot(snapshot);
  });

  test("startSettlingPeriod bad sig", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await createChannel(instance, data);
    await updateState(instance, data1, signs, "0x");

    await t.shouldFail(
      instance.startSettlingPeriod("0x" + data1.channel_id, data1.wrong_sign)
    );

    await revertSnapshot(snapshot);
  });

  test("startSettlingPeriod wrong private key", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await createChannel(instance, data);
    await updateState(instance, data1, signs, "0x");

    await t.shouldFail(
      instance.startSettlingPeriod("0x" + data1.channel_id, data1.wrong_sign)
    );

    await revertSnapshot(snapshot);
  });
};
