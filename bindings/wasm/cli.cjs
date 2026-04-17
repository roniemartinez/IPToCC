#!/usr/bin/env node
const { Command } = require("commander");
const { country_code } = require("./pkg-nodejs/iptocc_wasm.js");
const pkg = require("./package.json");

const program = new Command();
program
  .name("iptocc")
  .description("Fast, offline IP-to-country lookup.")
  .version(pkg.version)
  .argument("[addresses...]", "IPv4 or IPv6 addresses to look up")
  .action((addresses) => {
    if (!addresses || addresses.length === 0) {
      program.help();
      return;
    }
    if (addresses.length === 1) {
      const cc = country_code(addresses[0]);
      if (cc === null || cc === undefined) {
        process.exit(1);
      }
      process.stdout.write(`${cc}\n`);
      return;
    }
    const results = country_code(addresses);
    for (let i = 0; i < addresses.length; i++) {
      process.stdout.write(`${addresses[i]} ${results[i] ?? "-"}\n`);
    }
  });

program.parse();
