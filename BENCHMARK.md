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
|---------------------------------------------| ------: | ------: |
| AFRINIC `41.0.0.1` / `2001:4200::1`         |  5.7 ns | 39.4 ns |
| APNIC `1.0.16.1` / `2001:200::1`            |  8.4 ns | 39.0 ns |
| ARIN `8.8.8.8` / `2001:4860:4860::8888`     |  5.6 ns | 50.8 ns |
| LACNIC `200.160.0.1` / `2001:1280::1`       |  6.6 ns | 39.3 ns |
| RIPE NCC `193.0.6.139` / `2001:67c:18::1`   |  7.5 ns | 50.5 ns |
| miss `10.0.0.0` / `::1`                     |  5.6 ns | 16.1 ns |

Typed input via `country_code(Ipv4Addr)` / `country_code(Ipv6Addr)`:

| Case     |   IPv4 |    IPv6 |
| -------- | -----: | ------: |
| AFRINIC  | 1.3 ns |  9.9 ns |
| APNIC    | 3.7 ns |  9.3 ns |
| ARIN     | 1.4 ns |  9.9 ns |
| LACNIC   | 1.3 ns |  9.9 ns |
| RIPE NCC | 2.1 ns | 13.3 ns |
| miss     | 1.1 ns |  1.1 ns |

```bash
task bench:rust
```

## Python binding (pytest-benchmark)

Single-call latency:

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  |  84.6 ns | 134.4 ns |
| APNIC    |  99.3 ns | 133.3 ns |
| ARIN     |  44.0 ns |  95.5 ns |
| LACNIC   |  92.8 ns |  89.6 ns |
| RIPE NCC |  98.6 ns | 140.9 ns |
| miss     |  35.3 ns |  45.8 ns |

Batch call, `country_code(list_of_N)`:

|      N |    Total | Per address |
| -----: | -------: | ----------: |
|     10 |   367 ns |     36.7 ns |
|    100 |  2.47 us |     24.7 ns |
|  1,000 |  23.8 us |     23.8 ns |
| 10,000 | 251.5 us |     25.1 ns |

```bash
task bench:python
```

## WASM binding (mitata)

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  | 187.4 ns | 272.2 ns |
| APNIC    | 197.7 ns | 270.6 ns |
| ARIN     | 184.2 ns | 297.1 ns |
| LACNIC   | 192.2 ns | 272.3 ns |
| RIPE NCC | 197.7 ns | 295.0 ns |
| miss     | 103.2 ns | 119.4 ns |

```bash
task bench:wasm
```

## Run all

```bash
task bench
```
