const test = require("blue-tape");

const {fetchData, verifySign1, verifySign2} = require("./utest.js");

module.exports = async (test, instance) => {
  test("Verify signature with locally hashed inputs", async t => {
      const data = await fetchData();
      const givenAddr = '0x' + data.address;
      
      t.equal(await verifySign1(data, instance), true);

  });
};

