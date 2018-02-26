// cook mango twist then skin sort option civil have still rather guilt

const test = require("blue-tape");
const p = require("util").promisify;

const {
  takeSnapshot,
  revertSnapshot,
  getSettlingData,
  getData
} = require("./utils.js");

module.exports = async (test, instance) => {
  test("newChannel happy path", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);
    const data1 = await getSettlingData(0);

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

    //let b = await instance.balanceOf(data.address_0);
    //console.log(b);
    t.equal((await instance.balanceOf.call(data.addr_0)).c[0], 7000);
    t.equal((await instance.balanceOf.call(data.addr_1)).c[0], 7000);

    t.deepEqual(
      JSON.parse(JSON.stringify(await instance.channels("0x" + data.chl_id))),
      [
        "0x1000000000000000000000000000000000000000000000000000000000000000",
        data.addr_0,
        data.addr_1,
        "30000",
        "15000",
        "15000",
        "0x",
        "0",
        "2",
        false,
        "0",
        false
      ]
    );

    await revertSnapshot(snapshot);
  });

  test("newChannel bad sig", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);

    //await instance.depositToAddress.sendTransaction(data.address_0, {value: 22000});
    //await instance.depositToAddress.sendTransaction(data.address_1, {value: 22000});

    await t.shouldFail(
      instance.newChannel(
        "0x" + data.chl_id,
        data.addr_0,
        data.addr_1,
        data.bal_0,
        data.bal_1,
        data.set_period_ln,
        data.sig_0,
        data.bogus_sign
      )
    );

    await revertSnapshot(snapshot);
  });

  test("newChannel bad amount", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(1);

    //await instance.depositToAddress.sendTransaction(data.address_0, {value: 22000});
    //await instance.depositToAddress.sendTransaction(data.address_1, {value: 22000});

    await t.shouldFail(
      instance.newChannel(
        "0x" + data.chl_id,
        data.addr_0,
        data.addr_1,
        data.bal_0,
        data.bogus_amount,
        data.set_period_ln,
        data.sig_0,
        data.sig_1
      )
    );

    //await t.shouldFail(createChannel(instance, channelId, 6, 130, 2));
    await revertSnapshot(snapshot);
  });

  test("newChannel already exists", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);

    await t.shouldFail(
      instance.newChannel(
        "0x" + data.chl_id,
        data.addr_0,
        data.addr_1,
        data.bal_0,
        data.bal_1,
        data.set_period_ln,
        data.sig_0,
        data.sig_1
      )
    );

    await revertSnapshot(snapshot);
  });

  test("newChannel wrong public key", async t => {
    const snapshot = await takeSnapshot();
    const data = await getData(0);

    //await instance.depositToAddress.sendTransaction(data.address_0, {value: 22000});
    //await instance.depositToAddress.sendTransaction(data.address_1, {value: 22000});

    await t.shouldFail(
      instance.newChannel(
        "0x" + data.chl_id,
        data.addr_0,
        data.bogus_addr,
        data.bal_0,
        data.bal_1,
        data.set_period_ln,
        data.sig_0,
        data.sig_1
      )
    );

    await revertSnapshot(snapshot);
  });
};
