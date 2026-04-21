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
| AFRINIC `41.0.0.1` / `2001:4200::1`        |  6.4 ns | 37.6 ns |
| APNIC `1.0.16.1` / `2001:200::1`           |  8.8 ns | 39.6 ns |
| ARIN `8.8.8.8` / `2001:4860:4860::8888`    |  6.2 ns | 46.6 ns |
| LACNIC `200.160.0.1` / `2001:1280::1`      |  7.2 ns | 36.7 ns |
| RIPE NCC `193.0.6.139` / `2001:67c:18::1` | 12.6 ns | 63.8 ns |
| miss `10.0.0.0` / `::1`                    |  6.4 ns | 16.5 ns |

Typed input via `country_code(Ipv4Addr)` / `country_code(Ipv6Addr)`:

| Case     |   IPv4 |    IPv6 |
| -------- | -----: | ------: |
| AFRINIC  | 1.9 ns |  8.2 ns |
| APNIC    | 5.2 ns |  8.5 ns |
| ARIN     | 1.9 ns |  8.2 ns |
| LACNIC   | 1.9 ns |  8.3 ns |
| RIPE NCC | 8.9 ns | 25.8 ns |
| miss     | 1.9 ns |  1.1 ns |

```bash
task bench:rust
```

## Python binding (pytest-benchmark)

Single-call latency:

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  | 102.3 ns | 131.2 ns |
| APNIC    | 108.7 ns | 164.4 ns |
| ARIN     | 100.2 ns | 110.6 ns |
| LACNIC   | 101.9 ns | 150.7 ns |
| RIPE NCC | 112.0 ns | 169.5 ns |
| miss     |  41.5 ns |  48.4 ns |

Batch call, `country_code(list_of_N)`:

|      N |    Total | Per address |
| -----: | -------: | ----------: |
|     10 |   423 ns |     42.3 ns |
|    100 |  3.81 us |     38.1 ns |
|  1,000 |  28.3 us |     28.3 ns |
| 10,000 | 298.6 us |     29.9 ns |

```bash
task bench:python
```

## WASM binding (mitata)

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  | 246.0 ns | 273.3 ns |
| APNIC    | 210.2 ns | 273.2 ns |
| ARIN     | 202.9 ns | 297.0 ns |
| LACNIC   | 210.2 ns | 270.2 ns |
| RIPE NCC | 220.5 ns | 315.4 ns |
| miss     | 114.0 ns | 122.2 ns |

```bash
task bench:wasm
```

## Run all

```bash
task bench
```
