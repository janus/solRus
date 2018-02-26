// cook mango twist then skin sort option civil have still rather guilt

const test = require("blue-tape");
const p = require("util").promisify;

const {
  takeSnapshot,
  revertSnapshot,
  solSha3,
  mineBlocks,
  createChannel,
  closeChannelWithoutNewChannel,
  updateState,
  getData,
  getSettlingData,
  getSpData,
  toSolInt256,
  closeChannel,
  closeChannelSp
} = require("./utils.js");

module.exports = async (test, instance) => {
  test("closeChannel happy path no hashlocks", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const hashlocks = "0x";

    await closeChannel(instance, data, data1, hashlocks);

    t.equal((await instance.balanceOf.call(data.addr_0)).toString(), "24000");
    t.equal((await instance.balanceOf.call(data.addr_1)).toString(), "20000");

    await revertSnapshot(snapshot);
  });

  test("channel does not exist", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await closeChannel(instance, data, data1, "0x");
    await t.shouldFail(instance.closeChannel("0x" + data1.chl_id_wg));

    await revertSnapshot(snapshot);
  });

  test("channel is not settled", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await createChannel(instance, data);
    // already existing
    await updateState(instance, data1, "0x");

    await t.shouldFail(instance.closeChannel("0x" + data.chl_id));

    await revertSnapshot(snapshot);
  });

  test("channel is already closed", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await closeChannel(instance, data, data1, "0x");
    await t.shouldFail(closeChannelWithoutNewChannel(instance, data1, "0x"));

    await revertSnapshot(snapshot);
  });

  test("hashlocks do not match", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await createChannel(instance, data);
    // already existing
    await t.shouldFail(closeChannel(instance, data, data1, "0x06"));

    await revertSnapshot(snapshot);
  });

  test("bad amount", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await createChannel(instance, data);
    await t.shouldFail(closeChannelSp(instance, data, data1, "0x"));

    await revertSnapshot(snapshot);
  });

  test("closeChannel happy path with hashlocks (1 missing preimage)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(2);
    const data1 = await getSettlingData(2);
    const data2 = await getSpData(0);

    const channelId =
      "0x1000000000000000000000000000000000000000000000000000000000000000";
    const preimage1 =
      "0x2000000000000000000000000000000000000000000000000000000000000000";
    const preimage2 =
      "0x3000000000000000000000000000000000000000000000000000000000000000";
    const preimage3 =
      "0x4000000000000000000000000000000000000000000000000000000000000000";

    await instance.submitPreimage(solSha3(preimage1), preimage1);
    await instance.submitPreimage(solSha3(preimage2), preimage2);

    // It doesn't matter if the adjustments in the hashlocks exceed the balances
    // in the channel individually as long as they add up to a totalAdjustment
    // that doesn't exceed the balances in the channel
    const hashlock1 = `${solSha3(preimage1).slice(2)}${toSolInt256(-10002)}`;
    const hashlock2 = `${solSha3(preimage2).slice(2)}${toSolInt256(10001)}`;
    const hashlock3 = `${solSha3(preimage3).slice(2)}${toSolInt256(2)}`;

    //console.log(
    //console(`0x${hashlock1}${hashlock2}${hashlock3}`)

    let mydata = {
      chl_id: data1.chl_id,
      seq_num: data1.seq_num,
      bal_0: data1.bal_0,
      bal_1: data1.bal_1,
      sig_0: data2.sig_0,
      sig_1: data2.sig_1,
      sig_start_stl_p: data1.sig_start_stl_p
    };

    await closeChannel(
      instance,
      data,
      mydata,
      `0x${hashlock1}${hashlock2}${hashlock3}`
    );

    t.equal((await instance.balanceOf.call(data.addr_0)).toString(), "23999");
    t.equal((await instance.balanceOf.call(data.addr_1)).toString(), "20001");

    await revertSnapshot(snapshot);
  });

  test("closeChannel happy path with lots of hashlocks", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(3);
    const data1 = await getSettlingData(3);
    const data2 = await getSpData(1);

    let hashlocks = "0x";
    let preimages = "0x";
    let amount = 1;

    for (let i = 0; i < 100; i++) {
      const preimage = solSha3(i);

      preimages = preimages + solSha3(preimage).slice(2) + preimage.slice(2);
      hashlocks = hashlocks + preimage.slice(2) + toSolInt256(amount);

      amount = -amount;
    }

    let mydata = {
      chl_id: data1.chl_id,
      seq_num: data1.seq_num,
      bal_0: data1.bal_0,
      bal_1: data1.bal_1,
      sig_0: data2.sig_0,
      sig_1: data2.sig_1,
      sig_start_stl_p: data1.sig_start_stl_p
    };

    await instance.submitPreimages(preimages);

    await mineBlocks(1);
    await closeChannel(instance, data, mydata, hashlocks);

    t.equal((await instance.balanceOf.call(data.addr_0)).toString(), "24000");
    t.equal((await instance.balanceOf.call(data.addr_1)).toString(), "20000");

    await revertSnapshot(snapshot);
  });

  test("closeChannelFast happy path no hashlocks", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await createChannel(instance, data);

    await instance.closeChannelFast(
      "0x" + data1.chl_id,
      data1.seq_num,
      data1.bal_0,
      data1.bal_1,
      "0x",
      data1.sig_0_cl,
      data1.sig_1_cl
    );

    t.equal((await instance.balanceOf.call(data.addr_0)).toString(), "24000");
    t.equal((await instance.balanceOf.call(data.addr_1)).toString(), "20000");

    await revertSnapshot(snapshot);
  });

  test("closeChannelFast nonexistant channel", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    //await createChannel(instance, data);
    // already existing

    await t.shouldFail(
      instance.closeChannelFast(
        "0x" + data1.chl_id_lt,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        "0x",
        data1.sig_0_cl,
        data1.sig_1_cl
      )
    );

    await revertSnapshot(snapshot);
  });

  test("closeChannelFast bad sig", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await createChannel(instance, data);
    //already created

    await t.shouldFail(
      instance.closeChannelFast(
        "0x" + data1.chl_id,
        data1.seq_num,
        data1.bal_0,
        data1.bal_1,
        "0x",
        data.sig_start_stl_p,
        data.sig_bogus_msg
      )
    );

    await revertSnapshot(snapshot);
  });

  test("closeChannelFast bad amount", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    //console.log("await createChannel(instance, data)");
    await createChannel(instance, data);
    //already created

    await t.shouldFail(
      instance.closeChannelFast(
        "0x" + data1.chl_id,
        data1.seq_num,
        data.bogus_amount,
        data1.bal_1,
        "0x",
        data1.sig_0_cl,
        data1.sig_1_cl
      )
    );
    await revertSnapshot(snapshot);
  });
};
