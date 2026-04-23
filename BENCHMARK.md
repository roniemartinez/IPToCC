# Benchmarks

Per-call latency for `country_code(address)`, one well-known IP per RIR plus one negative case per protocol.

## Reference machine

|                      |                                |
| -------------------- | ------------------------------ |
| Model                | Apple M3 Pro (11-core), 18 GiB |
| OS                   | macOS 26.3.1                   |
| Rust / Python / Node | 1.95.0 / 3.14.3 / 22.15.0      |

## Rust core (Criterion)

String input via `country_code(&str)`:

| Case                                       |    IPv4 |    IPv6 |
| ------------------------------------------ | ------: | ------: |
| AFRINIC `41.0.0.1` / `2001:4200::1`        |  5.6 ns | 39.7 ns |
| APNIC `1.0.16.1` / `2001:200::1`           |  8.3 ns | 39.0 ns |
| ARIN `8.8.8.8` / `2001:4860:4860::8888`    |  5.6 ns | 49.7 ns |
| LACNIC `200.160.0.1` / `2001:1280::1`      |  6.7 ns | 42.0 ns |
| RIPE NCC `193.0.6.139` / `2001:67c:18::1` | 13.9 ns | 74.6 ns |
| miss `10.0.0.0` / `::1`                    |  5.6 ns | 16.2 ns |

Typed input via `country_code(Ipv4Addr)` / `country_code(Ipv6Addr)`:

| Case     |   IPv4 |    IPv6 |
| -------- | -----: | ------: |
| AFRINIC  | 1.3 ns |  9.8 ns |
| APNIC    | 3.5 ns |  9.3 ns |
| ARIN     | 1.3 ns |  9.8 ns |
| LACNIC   | 1.4 ns |  9.8 ns |
| RIPE NCC | 8.6 ns | 29.5 ns |
| miss     | 1.1 ns |  1.1 ns |

```bash
task bench:rust
```

## Python binding (pytest-benchmark)

Single-call latency:

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  |  98.8 ns | 128.7 ns |
| APNIC    | 103.7 ns | 128.6 ns |
| ARIN     |  93.7 ns | 135.8 ns |
| LACNIC   |  98.8 ns |  87.6 ns |
| RIPE NCC | 116.7 ns | 164.3 ns |
| miss     |  36.6 ns |  92.0 ns |

Batch call, `country_code(list_of_N)`:

|      N |    Total | Per address |
| -----: | -------: | ----------: |
|     10 |   349 ns |     34.9 ns |
|    100 |  2.88 us |     28.8 ns |
|  1,000 |  27.5 us |     27.5 ns |
| 10,000 | 288.0 us |     28.8 ns |

```bash
task bench:python
```

## WASM binding (mitata)

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  | 244.2 ns | 273.2 ns |
| APNIC    | 200.5 ns | 272.7 ns |
| ARIN     | 185.0 ns | 296.8 ns |
| LACNIC   | 194.7 ns | 272.6 ns |
| RIPE NCC | 217.7 ns | 323.2 ns |
| miss     | 102.5 ns | 120.2 ns |

```bash
task bench:wasm
```

## Run all

```bash
task bench
```
