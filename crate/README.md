# iptocc

[![crates.io](https://img.shields.io/crates/v/iptocc.svg?logo=rust&label=crates.io&style=for-the-badge)](https://crates.io/crates/iptocc)
[![CI](https://img.shields.io/github/actions/workflow/status/roniemartinez/IPToCC/ci.yml?branch=master&label=CI&logo=github%20actions&style=for-the-badge)](https://github.com/roniemartinez/IPToCC/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?logo=rust&style=for-the-badge)](https://www.rust-lang.org/)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=for-the-badge)

Fast, offline IPv4/IPv6 to ISO 3166-1 alpha-2 country code lookup.

Python and WASM bindings built on this crate live in the [same repository](https://github.com/roniemartinez/IPToCC).

> [!NOTE]
> Country codes reflect the country **assigned** by a **Regional Internet Registry (RIR)** to each IP block, not where the block is being used. RIR data agrees with MaxMind for **~95%** of IPv4 addresses and has minimal discrepancies for IPv6 ([Zander, 2012](https://figshare.swinburne.edu.au/articles/report/On_the_accuracy_of_IP_geolocation_based_on_IP_allocation_data/26254751)).

## Features

- Offline lookup, no API keys, no network calls
- IPv4 and IPv6 in one call
- Generic `country_code<T: IpAddress>(input)` accepts `&str`, `String`, `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`
- Batch `country_codes` takes any iterable of the above
- Lookup data embedded via `include_bytes!`; no runtime file I/O
- Nanosecond-scale lookups: ~1.3 ns typed, ~6 ns string. See [BENCHMARK.md](../BENCHMARK.md).
- `iptocc` CLI installed with `cargo install iptocc`
- Data refreshed nightly from the five Regional Internet Registries (RIRs)

## Install

```bash
cargo add iptocc
```

## Usage

```rust
use iptocc::{country_code, country_codes};

assert_eq!(country_code("8.8.8.8"), Some("US"));
assert_eq!(country_code("2001:4860:4860::8888"), Some("US"));
assert_eq!(country_code("10.0.0.0"), None);

let codes = country_codes(["8.8.8.8", "1.0.16.1"]);
assert_eq!(codes, vec![Some("US"), Some("JP")]);
```

Typed inputs are also accepted and skip the string parsing step:

```rust
use std::net::Ipv4Addr;
use iptocc::country_code;

assert_eq!(country_code(Ipv4Addr::new(8, 8, 8, 8)), Some("US"));
```

A CLI is installed by `cargo install iptocc`:

```bash
$ iptocc 8.8.8.8 1.0.16.1
8.8.8.8 US
1.0.16.1 JP
```
