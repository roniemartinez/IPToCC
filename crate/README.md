# iptocc

Fast, offline IPv4/IPv6 to ISO 3166-1 alpha-2 country code lookup for Rust.

## Features

- Offline lookup, no API keys, no network calls
- IPv4 and IPv6 in one call
- Single-address `country_code` and batch `country_codes` functions
- Lookup data embedded via `include_bytes!`; no runtime file I/O
- `iptocc` CLI binary installed alongside the library
- Database refreshed nightly from the five Regional Internet Registries

## Install

```bash
cargo add iptocc
```

## Usage

```rust
use iptocc::{country_code, country_codes};

let cc = country_code("8.8.8.8");
assert_eq!(cc, Some("US"));

let codes = country_codes(["8.8.8.8", "1.0.16.1"]);
assert_eq!(codes, vec![Some("US"), Some("JP")]);
```

A CLI is installed by `cargo install iptocc`:

```bash
$ iptocc 8.8.8.8 1.0.16.1
8.8.8.8 US
1.0.16.1 JP
```
