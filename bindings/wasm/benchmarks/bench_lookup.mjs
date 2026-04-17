import { bench, group, run } from "mitata";
import { country_code } from "../pkg-nodejs/iptocc_wasm.js";

const V4 = [
  ["afrinic",      "41.0.0.1"],
  ["apnic",        "1.0.16.1"],
  ["arin",         "8.8.8.8"],
  ["lacnic",       "200.160.0.1"],
  ["ripencc",      "193.0.6.139"],
  ["miss_private", "10.0.0.0"],
];

const V6 = [
  ["afrinic",       "2001:4200::1"],
  ["apnic",         "2001:200::1"],
  ["arin",          "2001:4860:4860::8888"],
  ["lacnic",        "2001:1280::1"],
  ["ripencc",       "2001:67c:18::1"],
  ["miss_loopback", "::1"],
];

group("v4", () => {
  for (const [name, addr] of V4) {
    bench(name, () => country_code(addr));
  }
});

group("v6", () => {
  for (const [name, addr] of V6) {
    bench(name, () => country_code(addr));
  }
});

await run();
