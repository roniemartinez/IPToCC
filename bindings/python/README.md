# iptocc (Python bindings)

Fast, offline IP-to-country lookup. Python bindings over the [iptocc](https://github.com/roniemartinez/IPToCC) Rust crate.

## Install

Requires Python 3.10 or newer.

```bash
pip install iptocc
```

## Usage

```python
from iptocc import country_code

country_code("8.8.8.8")                # "US"
country_code("2001:4860:4860::8888")   # "US"
country_code("10.0.0.0")               # None
```

A command-line tool is installed alongside the library:

```bash
iptocc 8.8.8.8
# US
```

See the [repository README](https://github.com/roniemartinez/IPToCC) for the full feature list, benchmarks, and data sources.
