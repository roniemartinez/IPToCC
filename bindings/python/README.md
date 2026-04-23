# iptocc

[![PyPI](https://img.shields.io/pypi/v/iptocc.svg?logo=pypi&logoColor=white&label=PyPI&style=for-the-badge)](https://pypi.org/project/iptocc/)
[![Python](https://img.shields.io/pypi/pyversions/iptocc.svg?logo=python&logoColor=white&label=Python&style=for-the-badge)](https://pypi.org/project/iptocc/)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=for-the-badge)

Python bindings for the [`iptocc`](https://crates.io/crates/iptocc) Rust crate. Offline IPv4 and IPv6 to ISO 3166-1 alpha-2 country code lookup.

> **Note:** Country codes reflect the country **assigned** by a **Regional Internet Registry (RIR)** to each IP block, not where the block is being used. RIR data agrees with MaxMind for **~95%** of IPv4 addresses and has minimal discrepancies for IPv6 ([Zander, 2012](https://figshare.swinburne.edu.au/articles/report/On_the_accuracy_of_IP_geolocation_based_on_IP_allocation_data/26254751)).

> **Important:** iptocc 3.0 is a complete Rust rewrite. Versions 2.x and earlier were pure Python on top of pandas. The public API stays mostly compatible (see migration notes below), single-lookup latency dropped from ~78 ms to ~44-141 ns, and a batch API was added.

## Features

- Offline lookup, no API keys, no network calls
- IPv4 and IPv6 in one call
- Single string OR batch (any iterable of strings)
- Lookup data embedded in the wheel; no runtime file I/O
- `iptocc` CLI installed with `pip install iptocc`
- Data refreshed nightly from the five Regional Internet Registries

## Install

Requires Python 3.10 or newer.

```bash
pip install iptocc
```

## Usage

```python
from iptocc import country_code

country_code("8.8.8.8")                       # "US"
country_code(["8.8.8.8", "1.0.16.1"])         # ["US", "JP"]
```

A CLI is installed alongside the library:

```bash
$ iptocc 8.8.8.8 1.0.16.1
8.8.8.8 US
1.0.16.1 JP
```

## Migrating from 2.x

```python
# 2.x
from iptocc import get_country_code, CountryCodeNotFound
try:
    cc = get_country_code("8.8.8.8")
except CountryCodeNotFound:
    cc = None

# 3.x
from iptocc import country_code
cc = country_code("8.8.8.8")  # returns None on miss
```

## See also

- [iptocc](https://crates.io/crates/iptocc) — the Rust crate this binding is built on
- [@roniemartinez/iptocc](https://www.npmjs.com/package/@roniemartinez/iptocc) — WASM/JavaScript bindings
- [GitHub repo](https://github.com/roniemartinez/IPToCC)
