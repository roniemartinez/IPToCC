# IPToCC

[![CI](https://img.shields.io/github/actions/workflow/status/roniemartinez/IPToCC/ci.yml?branch=master&label=CI&logo=github%20actions&style=for-the-badge)](https://github.com/roniemartinez/IPToCC/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/iptocc.svg?logo=rust&label=crates.io&style=for-the-badge)](https://crates.io/crates/iptocc)
[![PyPI](https://img.shields.io/pypi/v/iptocc.svg?logo=pypi&logoColor=white&label=PyPI&style=for-the-badge)](https://pypi.org/project/iptocc/)
[![npm](https://img.shields.io/npm/v/@roniemartinez/iptocc.svg?logo=npm&label=npm&style=for-the-badge)](https://www.npmjs.com/package/@roniemartinez/iptocc)
[![Python](https://img.shields.io/pypi/pyversions/iptocc.svg?logo=python&logoColor=white&label=Python&style=for-the-badge)](https://pypi.org/project/iptocc/)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?logo=rust&style=for-the-badge)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=for-the-badge)](#license)
[![All Contributors](https://img.shields.io/github/all-contributors/roniemartinez/IPToCC?label=all%20contributors&style=for-the-badge)](#contributors-)

Fast, offline IPv4/IPv6 to ISO 3166-1 alpha-2 country code lookup. One Rust core with Python, WebAssembly, and CLI bindings.

> [!IMPORTANT]
> **iptocc 3.0 is a complete Rust rewrite.** Previous versions (2.x and earlier) were pure-Python on top of pandas. The 3.x line ships a Rust core with PyO3 Python bindings, wasm-bindgen WASM bindings, and a standalone `iptocc` crate. The Python public API stays compatible for the common case (see [Migrating from 2.x](#migrating-from-2x)).

## Features

- Offline lookup, no external API calls.
- IPv4 and IPv6 in a single call.
- Accepts a single address or a batch of addresses.
- Lookup data embedded in the binary; no runtime file I/O after startup.
- Sub-microsecond per-call latency on native Rust; low hundreds of nanoseconds through Python or WebAssembly.
- `iptocc` CLI installed by all three ecosystems.
- Database refreshed nightly from the five Regional Internet Registries.

## Install

### Python (3.10 or newer)

```bash
pip install iptocc
```

### Rust

```bash
cargo add iptocc
```

### Node, browser, Deno, bundlers

```bash
npm install @roniemartinez/iptocc
```

## Usage

### Python

```python
from iptocc import country_code

# Single lookup
country_code("8.8.8.8")                # "US"
country_code("2001:4860:4860::8888")   # "US"
country_code("10.0.0.0")               # None

# Batch lookup (any iterable of strings)
country_code(["8.8.8.8", "1.0.16.1", "10.0.0.0"])
# ["US", "JP", None]
```

### Rust

```rust
use iptocc::{country_code, country_codes};

let cc = country_code("8.8.8.8");
assert_eq!(cc, Some("US"));

let codes = country_codes(["8.8.8.8", "1.0.16.1", "10.0.0.0"]);
assert_eq!(codes, vec![Some("US"), Some("JP"), None]);
```

### Node

```javascript
const { country_code } = require("@roniemartinez/iptocc");

country_code("8.8.8.8");                     // "US"
country_code(["8.8.8.8", "1.0.16.1"]);       // ["US", "JP"]
```

### CLI

```bash
$ iptocc 8.8.8.8
US

$ iptocc 8.8.8.8 1.0.16.1 10.0.0.0 193.0.6.139
8.8.8.8 US
1.0.16.1 JP
10.0.0.0 -
193.0.6.139 NL
```

## Comparison with other Python IP-to-country libraries

### Features

| Feature | iptocc 2.x (legacy) | [ip_to_country](https://github.com/jamesdolan/ip_to_country) | iptocc 3.x (this) |
|---|:-:|:-:|:-:|
| IPv4 | ✅ | ✅ | ✅ |
| IPv6 | ✅ |    | ✅ |
| Offline lookup | ✅ | ✅ | ✅ |
| Batch API |    |    | ✅ |
| CLI |    |    | ✅ |
| Python | ✅ | ✅ | ✅ |
| Rust |    |    | ✅ |
| WASM |    |    | ✅ |
| Nightly data refresh |    | ✅ | ✅ |

### Performance (Python)

Measured on an Apple M3 Pro across the same 1,000 unique IPv4 addresses, one-shot wall-clock timing via `time.perf_counter()`. Lookup-only; one-time data load is excluded. Lower is better.

| Library | 1 lookup | 1,000 lookups (total) | Relative to ip_to_country |
|---|---:|---:|---:|
| [ip_to_country](https://github.com/jamesdolan/ip_to_country) (`bisect` over `array.array`) | ~1.2 us | ~1.2 ms (loop; no batch API) | 1x (baseline) |
| iptocc 2.1.2 (legacy, pandas DataFrame filter) | ~78 ms | ~78 s (loop; no batch API) | ~65,000x slower |
| **iptocc 3.x (Rust + PyO3)** | **~100 ns** | **~31 us (batch)** | **~38x faster** |

So iptocc 3.x is roughly **~38x** faster than ip_to_country on a 1,000-address workload, while adding IPv6, a batch API, a CLI, and Rust/WASM bindings. (The legacy 2.1.2 row is included for historical context; its pandas-based filtering is orders of magnitude slower than any packed-binary approach.)

See [BENCHMARK.md](./BENCHMARK.md) for the full per-RIR breakdown and the Rust-core / WASM numbers.

## Migrating from 2.x

The legacy `iptocc.get_country_code(ip)` is now `iptocc.country_code(ip)`. The return value is still `str | None` on a hit or miss.

```python
# 2.x
from iptocc import get_country_code, CountryCodeNotFound
try:
    cc = get_country_code("8.8.8.8")
except CountryCodeNotFound:
    cc = None

# 3.x
from iptocc import country_code
cc = country_code("8.8.8.8")  # returns None on miss, no exception
```

The `get_country(ip)` full-name lookup is not currently provided; if you need this, open an issue.

## Data sources

Lookups are based on the delegated extended statistics published by the five Regional Internet Registries:

- AFRINIC: `https://ftp.afrinic.net/stats/afrinic/delegated-afrinic-extended-latest`
- ARIN: `https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest`
- APNIC: `https://ftp.apnic.net/public/apnic/stats/apnic/delegated-apnic-extended-latest`
- LACNIC: `https://ftp.lacnic.net/pub/stats/lacnic/delegated-lacnic-extended-latest`
- RIPE NCC: `https://ftp.ripe.net/pub/stats/ripencc/delegated-ripencc-extended-latest`

A nightly GitHub Action refreshes these files, regenerates the embedded lookup tables, bumps the package versions, and publishes new releases automatically.

## Development

Common dev tasks are exposed via [Taskfile](https://taskfile.dev):

```bash
task test          # run Rust, Python, and WASM tests
task bench         # run benchmarks across all three runtimes
task build:python  # rebuild the pyo3 extension
task build:wasm    # rebuild the wasm-pack nodejs bundle
```

See [BENCHMARK.md](./BENCHMARK.md) for performance numbers and how to reproduce them.

## References

- [RIR Statistics Exchange Format](https://www.apnic.net/about-apnic/corporate-documents/documents/resource-guidelines/rir-statistics-exchange-format/)
- [ISO 3166-1 alpha-2 country codes (Wikipedia)](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)

## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://ron.sh/"><img src="https://avatars.githubusercontent.com/u/2573537?v=4?s=100" width="100px;" alt="Ronie Martinez"/><br /><sub><b>Ronie Martinez</b></sub></a><br /><a href="https://github.com/roniemartinez/IPToCC/commits?author=roniemartinez" title="Code">💻</a> <a href="https://github.com/roniemartinez/IPToCC/commits?author=roniemartinez" title="Documentation">📖</a> <a href="#infra-roniemartinez" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-roniemartinez" title="Maintenance">🚧</a> <a href="https://github.com/roniemartinez/IPToCC/pulls?q=is%3Apr+reviewed-by%3Aroniemartinez" title="Reviewed Pull Requests">👀</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/tatemz"><img src="https://avatars.githubusercontent.com/u/2190655?v=4?s=100" width="100px;" alt="Tate Barber"/><br /><sub><b>Tate Barber</b></sub></a><br /><a href="https://github.com/roniemartinez/IPToCC/commits?author=tatemz" title="Code">💻</a> <a href="https://github.com/roniemartinez/IPToCC/commits?author=tatemz" title="Documentation">📖</a> <a href="#infra-tatemz" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/mathgeek12"><img src="https://avatars.githubusercontent.com/u/36207893?v=4?s=100" width="100px;" alt="mathgeek12"/><br /><sub><b>mathgeek12</b></sub></a><br /><a href="https://github.com/roniemartinez/IPToCC/issues?q=author%3Amathgeek12" title="Bug reports">🐛</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://jamesdolan.com"><img src="https://avatars.githubusercontent.com/u/7540221?v=4?s=100" width="100px;" alt="James Dolan"/><br /><sub><b>James Dolan</b></sub></a><br /><a href="#ideas-jamesdolan" title="Ideas, Planning, & Feedback">🤔</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
