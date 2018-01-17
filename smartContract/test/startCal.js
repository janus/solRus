const test = require("blue-tape");

const {fetchData, getData, verifySign1, verifySign2} = require("./utest.js");

module.exports = async (test, instance) => {
  test("start Address verification", async t => {
      await fetchData();
      const data = await getData();
      
      const givenAddr = '0x' + data.address;
      
      t.equal(await verifySign1(data, instance), givenAddr);
      
      t.equal(await verifySign2(data, instance), givenAddr);
      

  });
};






