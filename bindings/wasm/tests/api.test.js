const { test } = require("node:test");
const assert = require("node:assert");
const { country_code } = require("../pkg-nodejs/iptocc_wasm.js");

const singleCases = [
  ["41.0.0.1",              "ZA"],
  ["2001:4200::1",          "ZA"],
  ["1.0.16.1",              "JP"],
  ["2001:200::1",           "JP"],
  ["8.8.8.8",               "US"],
  ["2001:4860:4860::8888",  "US"],
  ["200.160.0.1",           "BR"],
  ["2001:1280::1",          "BR"],
  ["193.0.6.139",           "NL"],
  ["2001:67c:18::1",        "NL"],
  ["10.0.0.0",              null],
  ["not-an-ip",             null],
];

for (const [address, expected] of singleCases) {
  test(`country_code(${address}) -> ${expected ?? "null"}`, () => {
    assert.strictEqual(country_code(address), expected);
  });
}

const batchCases = [
  {
    name: "mixed_hits_and_miss",
    inputs: ["8.8.8.8", "1.0.16.1", "10.0.0.0"],
    expected: ["US", "JP", null],
  },
  {
    name: "empty",
    inputs: [],
    expected: [],
  },
];

for (const { name, inputs, expected } of batchCases) {
  test(`country_code batch (${name})`, () => {
    assert.deepStrictEqual(country_code(inputs), expected);
  });
}
