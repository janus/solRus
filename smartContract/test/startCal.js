const test = require("blue-tape");

const {fetchData, getData, verifySign} = require("./utest.js");

module.exports = async (test, instance) => {
  test("start Address verification", async t => {
      await fetchData();
      const data = await getData();
      
      const accountAddress = '0x' + data.pubkey;
      t.equal(await verifySign(data, instance), accountAddress);
      

  });
};






