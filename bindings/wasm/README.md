# @roniemartinez/iptocc

[![npm](https://img.shields.io/npm/v/@roniemartinez/iptocc.svg?logo=npm&label=npm&style=for-the-badge)](https://www.npmjs.com/package/@roniemartinez/iptocc)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=for-the-badge)

WASM bindings for the [`iptocc`](https://crates.io/crates/iptocc) Rust crate. Offline IPv4 and IPv6 to ISO 3166-1 alpha-2 country code lookup.

> **Note:** Country codes reflect the country **assigned** by a **Regional Internet Registry (RIR)** to each IP block, not where the block is being used. RIR data agrees with MaxMind for **~95%** of IPv4 addresses and has minimal discrepancies for IPv6 ([Zander, 2012](https://figshare.swinburne.edu.au/articles/report/On_the_accuracy_of_IP_geolocation_based_on_IP_allocation_data/26254751)).

## Features

- Offline lookup, no API keys, no network calls
- IPv4 and IPv6 in one call
- Single string OR batch (array of strings)
- Multi-target output: Node, bundlers, browser, Deno, no-modules
- `iptocc` CLI installed with `npm install -g @roniemartinez/iptocc` (or use `npx`)
- Data refreshed nightly from the five Regional Internet Registries

## Install

```bash
npm install @roniemartinez/iptocc
```

## Usage

```javascript
const { country_code } = require("@roniemartinez/iptocc");

country_code("8.8.8.8");                  // "US"
country_code(["8.8.8.8", "1.0.16.1"]);    // ["US", "JP"]
```

Per-target entry points: `@roniemartinez/iptocc/{nodejs,bundler,web,deno,no-modules}`.

Run the CLI without installing via `npx`:

```bash
$ npx @roniemartinez/iptocc 8.8.8.8 1.0.16.1
8.8.8.8 US
1.0.16.1 JP
```

Or install the package and the `iptocc` bin is on your `PATH`.

## See also

- [iptocc](https://crates.io/crates/iptocc) — the Rust crate this binding is built on
- [iptocc on PyPI](https://pypi.org/project/iptocc/) — Python bindings
- [GitHub repo](https://github.com/roniemartinez/IPToCC)
