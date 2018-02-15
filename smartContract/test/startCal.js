const test = require("blue-tape");

const {
  fetchData,
  verifySign1,
  verifySign2,
  verifySignedBy,
  verifySignedByBoth,
  verifySignedByOne
} = require("./utest.js");

module.exports = async (test, instance) => {
  test("Verify signature with locally hashed inputs", async t => {
    const data = await fetchData("say_hello", "1");
    // const givenAddr = "0x" + data.address;

    t.equal(await verifySign1(data, instance), true);
  });

  const data = await fetchData("rtn_fingerprinted", "2");
  let sgn0 = data.sig_0;
  let sgn1 = data.sig_1;
  let addr0 = data.address_0;
  let addr1 = data.address_1;
  let fingerprint = "0x" + data.fingerprint;

  test("\nTest signedBy with Rust generated values", async t => {
    //Test signedBy
    t.equal(await verifySignedBy(sgn0, addr0, fingerprint, instance), true);
  });

  test("\nTest signByBoth with Rust generated values", async t => {
    t.equal(
      await verifySignedByBoth(sgn0, addr0, sgn1, addr1, fingerprint, instance),
      true
    );
  });

  test("\nTest signByOne with Rust generated values", async t => {
    addr1 = "0xbc073e9233944b5e729e46d618f0d8edf3d9c34a";

    t.equal(
      await verifySignedByOne(sgn0, addr0, addr1, fingerprint, instance),
      true
    );
  });
};
