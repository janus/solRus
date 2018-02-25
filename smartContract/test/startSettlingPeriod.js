// cook mango twist then skin sort option civil have still rather guilt

const test = require("blue-tape");
const p = require("util").promisify;

const {
  filterLogs,
  takeSnapshot,
  revertSnapshot,
  getData,
  getSettlingData,
  createChannel,
  updateState,
  startSettlingPeriod
} = require("./utils.js");

module.exports = async (test, instance) => {
  test("startSettlingPeriod nonexistant channel", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(14);
    const data1 = await getSettlingData(14);

    await createChannel(instance, data);

    await t.shouldFail(
      instance.updateState(
        "0x" + data1.chl_id_wg,
        "0x" + data1.seq_num,
        "0x" + data1.bal_0,
        "0x" + data1.bal_1,
        "0x",
        data1.sig_0,
        data1.sig_1
      )
    );

    await revertSnapshot(snapshot);
  });

  test("startSettlingPeriod already started", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(11);
    const data1 = await getSettlingData(11);

    await createChannel(instance, data);
    await updateState(instance, data1, "0x");
    await startSettlingPeriod(instance, data1);
    await t.shouldFail(startSettlingPeriod(instance, data1));

    await revertSnapshot(snapshot);
  });

  test("startSettlingPeriod bad sig", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(12);
    const data1 = await getSettlingData(12);

    await createChannel(instance, data);
    await updateState(instance, data1, "0x");

    await t.shouldFail(
      instance.startSettlingPeriod("0x" + data1.chl_id, data1.sig_bogus_msg)
    );

    await revertSnapshot(snapshot);
  });

  test("startSettlingPeriod wrong private key", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(13);
    const data1 = await getSettlingData(13);

    await createChannel(instance, data);
    await updateState(instance, data1, "0x");

    await t.shouldFail(
      instance.startSettlingPeriod("0x" + data1.chl_id, data1.sig_bogus_msg)
    );

    await revertSnapshot(snapshot);
  });
};
