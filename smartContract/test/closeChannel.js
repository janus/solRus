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
  getSignBlocks,
  toSolInt256,
  closeChannel,
  closeChannelSp
} = require("./utils.js");

module.exports = async (test, instance) => {
  test("closeChannel happy path no hashlocks", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);
    const hashlocks = "0x";

    await closeChannel(instance, data, data1, signs, hashlocks);

    t.equal(
      (await instance.balanceOf.call(data.address_0)).toString(),
      "24000"
    );
    t.equal(
      (await instance.balanceOf.call(data.address_1)).toString(),
      "20000"
    );

    await revertSnapshot(snapshot);
  });

  test("channel does not exist", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await closeChannel(instance, data, data1, signs, "0x");
    await t.shouldFail(instance.closeChannel("0x" + data1.bad_channel_id));

    await revertSnapshot(snapshot);
  });

  test("channel is not settled", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await createChannel(instance, data);
    await updateState(instance, data1, signs, "0x");

    await t.shouldFail(instance.closeChannel("0x" + data.channel_id));

    await revertSnapshot(snapshot);
  });

  test("channel is already closed", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await closeChannel(instance, data, data1, signs, "0x");
    await t.shouldFail(
      closeChannelWithoutNewChannel(instance, data, data1, signs, "0x")
    );

    await revertSnapshot(snapshot);
  });

  test("hashlocks do not match", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await createChannel(instance, data);

    await t.shouldFail(closeChannel(instance, data, data1, signs, "0x06"));

    await revertSnapshot(snapshot);
  });

  test("bad amount", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(1);

    await createChannel(instance, data);
    await t.shouldFail(closeChannelSp(instance, data, data1, signs, "0x"));

    await revertSnapshot(snapshot);
  });

  test("closeChannel happy path with hashlocks (1 missing preimage)", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(2);
    const data1 = await getSettlingData(2);
    const data2 = await getSpData(0);
    const signs = await getSignBlocks(2);

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

    let mydata = {
      channel_id: data1.channel_id,
      seq_num: data1.seq_num,
      balance_0: data1.balance_0,
      balance_1: data1.balance_1,
      sign_priv_0: data2.update_state_sign_priv_0,
      sign_priv_1: data2.update_state_sign_priv_1,
      sign_start_settling_period: data1.sign_start_settling_period
    };

    await closeChannel(
      instance,
      data,
      mydata,
      mydata,
      `0x${hashlock1}${hashlock2}${hashlock3}`
    );

    t.equal(
      (await instance.balanceOf.call(data.address_0)).toString(),
      "23999"
    );
    t.equal(
      (await instance.balanceOf.call(data.address_1)).toString(),
      "20001"
    );

    await revertSnapshot(snapshot);
  });

  test("closeChannel happy path with lots of hashlocks", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(3);
    const data1 = await getSettlingData(3);
    const data2 = await getSpData(1);
    const signs = await getSignBlocks(2);

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
      channel_id: data1.channel_id,
      seq_num: data1.seq_num,
      balance_0: data1.balance_0,
      balance_1: data1.balance_1,
      sign_priv_0: data2.update_state_sign_priv_0,
      sign_priv_1: data2.update_state_sign_priv_1,
      sign_start_settling_period: data1.sign_start_settling_period
    };

    await instance.submitPreimages(preimages);

    await mineBlocks(1);
    await closeChannel(instance, data, mydata, mydata, hashlocks);

    t.equal(
      (await instance.balanceOf.call(data.address_0)).toString(),
      "24000"
    );
    t.equal(
      (await instance.balanceOf.call(data.address_1)).toString(),
      "20000"
    );

    await revertSnapshot(snapshot);
  });

  test("closeChannelFast happy path no hashlocks", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);
    const signs = await getSignBlocks(2);

    await createChannel(instance, data);

    await instance.closeChannelFast(
      "0x" + data1.channel_id,
      data1.seq_num,
      data1.balance_0,
      data1.balance_1,
      "0x",
      data1.sign_close_chnl_fast_priv_0,
      data1.sign_close_chnl_fast_priv_1
    );

    t.equal(
      (await instance.balanceOf.call(data.address_0)).toString(),
      "24000"
    );
    t.equal(
      (await instance.balanceOf.call(data.address_1)).toString(),
      "20000"
    );

    await revertSnapshot(snapshot);
  });

  test("closeChannelFast nonexistant channel", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await createChannel(instance, data);

    await t.shouldFail(
      instance.closeChannelFast(
        "0x" + data1.bad_channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        "0x",
        data1.sign_close_chnl_fast_priv_0,
        data1.sign_close_chnl_fast_priv_1
      )
    );

    await revertSnapshot(snapshot);
  });

  test("closeChannelFast bad sig", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await createChannel(instance, data);

    await t.shouldFail(
      instance.closeChannelFast(
        "0x" + data1.bad_channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.balance_1,
        "0x",
        data1.sign_close_chnl_fast_priv_0,
        data1.sign_start_settling_period_wrong_prv
      )
    );

    await revertSnapshot(snapshot);
  });

  test("closeChannelFast bad amount", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);
    const data1 = await getSettlingData(1);

    await createChannel(instance, data);

    await t.shouldFail(
      instance.closeChannelFast(
        "0x" + data1.bad_channel_id,
        data1.seq_num,
        data1.balance_0,
        data1.total_balance,
        "0x",
        data1.sign_close_chnl_fast_priv_0,
        data1.sign_close_chnl_fast_priv_1
      )
    );
    await revertSnapshot(snapshot);
  });
};
