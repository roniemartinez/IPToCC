# Benchmarks

Per-call latency for `country_code(address)`, one well-known IP per RIR plus one negative case per protocol.
Batch tables cycle those same addresses to length N.

## Reference machine

|                      |                                |
| -------------------- | ------------------------------ |
| Model                | Apple M3 Pro (11-core), 18 GiB |
| OS                   | macOS 26.6.2                   |
| Rust / Python / Node | 1.98.1 / 3.14.3 / 22.15.0      |

Rust figures are Criterion point estimates, Python pytest-benchmark means, WASM medians of 7 mitata runs.

## Rust core (Criterion)

String input via `country_code(&str)`:

| Case                                        |    IPv4 |    IPv6 |
| ------------------------------------------- | ------: | ------: |
| AFRINIC `41.0.0.1` / `2001:4200::1`         |  5.9 ns | 32.4 ns |
| APNIC `1.0.16.1` / `2001:200::1`            |  7.7 ns | 29.0 ns |
| ARIN `8.8.8.8` / `2001:4860:4860::8888`     |  5.3 ns | 39.6 ns |
| LACNIC `200.160.0.1` / `2001:1280::1`       |  6.2 ns | 32.4 ns |
| RIPE NCC `193.0.6.139` / `2001:67c:18::1`   |  6.9 ns | 47.1 ns |
| miss `10.0.0.0` / `::1`                     |  5.6 ns | 12.3 ns |

Typed input via `country_code(Ipv4Addr)` / `country_code(Ipv6Addr)`:

| Case     |   IPv4 |    IPv6 |
| -------- | -----: | ------: |
| AFRINIC  | 1.3 ns |  9.7 ns |
| APNIC    | 3.8 ns |  9.0 ns |
| ARIN     | 1.3 ns |  9.9 ns |
| LACNIC   | 1.3 ns | 10.5 ns |
| RIPE NCC | 2.4 ns | 14.2 ns |
| miss     | 1.1 ns |  1.1 ns |

Batch call, `country_codes(iterable_of_N)`, string input:

|      N |     IPv4 | Per address |      IPv6 | Per address |
| -----: | -------: | ----------: | --------: | ----------: |
|     10 |  99.4 ns |      9.9 ns |  404.7 ns |     40.5 ns |
|    100 | 816.2 ns |      8.2 ns |   3.73 us |     37.3 ns |
|  1,000 |  8.01 us |      8.0 ns |  37.08 us |     37.1 ns |
| 10,000 | 79.40 us |      7.9 ns | 375.59 us |     37.6 ns |

Batch call, typed input:

|      N |     IPv4 | Per address |      IPv6 | Per address |
| -----: | -------: | ----------: | --------: | ----------: |
|     10 |  44.3 ns |      4.4 ns |  143.3 ns |     14.3 ns |
|    100 | 269.6 ns |      2.7 ns |   1.24 us |     12.4 ns |
|  1,000 |  2.58 us |      2.6 ns |  12.15 us |     12.1 ns |
| 10,000 | 24.94 us |      2.5 ns | 121.27 us |     12.1 ns |

```bash
task bench:rust
```

## Python binding (pytest-benchmark)

Single-call latency:

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  |  42.9 ns | 121.0 ns |
| APNIC    |  96.9 ns |  81.6 ns |
| ARIN     |  42.2 ns | 132.0 ns |
| LACNIC   |  44.5 ns |  81.2 ns |
| RIPE NCC |  93.9 ns | 127.5 ns |
| miss     |  34.6 ns |  42.3 ns |

Batch call, `country_code(list_of_N)`:

|      N |     Total | Per address |
| -----: | --------: | ----------: |
|     10 |  332.6 ns |     33.3 ns |
|    100 |   2.47 us |     24.7 ns |
|  1,000 |  23.63 us |     23.6 ns |
| 10,000 | 253.54 us |     25.4 ns |

```bash
task bench:python
```

## WASM binding (mitata)

Single-call latency:

| Case     |     IPv4 |     IPv6 |
| -------- | -------: | -------: |
| AFRINIC  | 181.4 ns | 251.3 ns |
| APNIC    | 195.1 ns | 251.8 ns |
| ARIN     | 181.2 ns | 270.9 ns |
| LACNIC   | 189.6 ns | 251.7 ns |
| RIPE NCC | 194.6 ns | 270.2 ns |
| miss     |  98.8 ns | 113.2 ns |

Batch call, `country_code(array_of_N)`:

|      N |      IPv4 | Per address |      IPv6 | Per address |
| -----: | --------: | ----------: | --------: | ----------: |
|     10 |   1.80 us |    180.0 ns |   2.39 us |    239.0 ns |
|    100 |  16.51 us |    165.1 ns |  22.16 us |    221.6 ns |
|  1,000 | 162.59 us |    162.6 ns | 220.07 us |    220.1 ns |
| 10,000 |   1.66 ms |    166.0 ns |   2.19 ms |    219.0 ns |

```bash
task bench:wasm
```

## Run all

```bash
task bench
```
