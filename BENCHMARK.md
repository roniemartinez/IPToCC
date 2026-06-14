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

| Case                                        |    IPv4 |    IPv6 |
| ------------------------------------------- | ------: | ------: |
| AFRINIC `41.0.0.1` / `2001:4200::1`         |  5.6 ns | 39.2 ns |
| APNIC `1.0.16.1` / `2001:200::1`            |  8.4 ns | 39.4 ns |
| ARIN `8.8.8.8` / `2001:4860:4860::8888`     |  5.6 ns | 49.4 ns |
| LACNIC `200.160.0.1` / `2001:1280::1`       |  6.6 ns | 39.1 ns |
| RIPE NCC `193.0.6.139` / `2001:67c:18::1`   |  7.8 ns | 51.2 ns |
| miss `10.0.0.0` / `::1`                     |  5.6 ns | 16.0 ns |

Typed input via `country_code(Ipv4Addr)` / `country_code(Ipv6Addr)`:

| Case     |   IPv4 |    IPv6 |
| -------- | -----: | ------: |
| AFRINIC  | 1.3 ns |  9.7 ns |
| APNIC    | 3.6 ns |  9.3 ns |
| ARIN     | 1.3 ns |  9.7 ns |
| LACNIC   | 1.3 ns |  9.8 ns |
| RIPE NCC | 2.7 ns | 13.3 ns |
| miss     | 1.1 ns |  1.1 ns |

```bash
task bench:rust
```

## Python binding (pytest-benchmark)

Single-call latency:

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  |  92.1 ns | 131.5 ns |
| APNIC    | 100.6 ns | 133.9 ns |
| ARIN     |  44.6 ns |  96.7 ns |
| LACNIC   |  96.0 ns | 130.6 ns |
| RIPE NCC |  98.9 ns | 152.0 ns |
| miss     |  35.3 ns |  45.9 ns |

Batch call, `country_code(list_of_N)`:

|      N |    Total | Per address |
| -----: | -------: | ----------: |
|     10 |   325 ns |     32.5 ns |
|    100 |  2.49 us |     24.9 ns |
|  1,000 |  23.6 us |     23.6 ns |
| 10,000 | 249.9 us |     25.0 ns |

```bash
task bench:python
```

## WASM binding (mitata)

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  | 190.2 ns | 279.0 ns |
| APNIC    | 206.1 ns | 275.4 ns |
| ARIN     | 191.8 ns | 301.5 ns |
| LACNIC   | 198.9 ns | 275.6 ns |
| RIPE NCC | 205.1 ns | 296.6 ns |
| miss     | 103.8 ns | 121.9 ns |

```bash
task bench:wasm
```

## Run all

```bash
task bench
```
