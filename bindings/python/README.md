# iptocc

Fast, offline IPv4/IPv6 to ISO 3166-1 alpha-2 country code lookup for Python.

> [!IMPORTANT]
> **iptocc 3.0 is a complete Rust rewrite.** Versions 2.x and earlier were pure-Python on top of pandas. The public API stays mostly compatible (see migration notes below) but is roughly 65,000x faster on bulk workloads and adds a batch API. IPv6 is still supported.

## Features

- Offline lookup, no API keys, no network calls
- IPv4 and IPv6 in one call
- Single string OR batch (any iterable of strings)
- Lookup data embedded in the wheel; no runtime file I/O
- `iptocc` CLI installed alongside the library
- Database refreshed nightly from the five Regional Internet Registries

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
