# iptocc (WASM bindings)

Fast, offline IP-to-country lookup. WebAssembly bindings over the [iptocc](https://github.com/roniemartinez/IPToCC) Rust crate. Targets Node, browser bundlers, web (no bundler), no-modules, and Deno.

## Install

```bash
npm install @roniemartinez/iptocc
```

## Usage

### Node (CommonJS)

```javascript
const { country_code } = require("@roniemartinez/iptocc");

country_code("8.8.8.8");                // "US"
country_code("2001:4860:4860::8888");   // "US"
country_code("10.0.0.0");               // undefined
```

### Bundlers, browser, Deno

```javascript
import { country_code } from "@roniemartinez/iptocc";

country_code("8.8.8.8");                // "US"
```

The package's `exports` field maps to per-target entry points: `@roniemartinez/iptocc/nodejs`, `@roniemartinez/iptocc/bundler`, `@roniemartinez/iptocc/web`, `@roniemartinez/iptocc/deno`, `@roniemartinez/iptocc/no-modules`.

A command-line tool is installed alongside the library:

```bash
iptocc 8.8.8.8
# US
```

See the [repository README](https://github.com/roniemartinez/IPToCC) for the full feature list, benchmarks, and data sources.
