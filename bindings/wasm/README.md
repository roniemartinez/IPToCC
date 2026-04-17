# @roniemartinez/iptocc

Fast, offline IPv4/IPv6 to ISO 3166-1 alpha-2 country code lookup for JavaScript.

## Features

- Offline lookup, no API keys, no network calls
- IPv4 and IPv6 in one call
- Single string OR batch (array of strings)
- Multi-target output: Node, bundlers, browser, Deno, no-modules
- `iptocc` CLI installed alongside the library
- Database refreshed nightly from the five Regional Internet Registries

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

A CLI is installed alongside the library:

```bash
$ iptocc 8.8.8.8 1.0.16.1
8.8.8.8 US
1.0.16.1 JP
```
