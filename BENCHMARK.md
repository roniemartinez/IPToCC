# Benchmarks

Per-call latency for `country_code(address)`, one well-known IP per RIR plus one negative case per protocol.

## Reference machine

| | |
|---|---|
| Model | Apple M3 Pro (11-core), 18 GiB |
| OS | macOS 26.3.1 |
| Rust / Python / Node | 1.94.1 / 3.14.3 / 22.15.0 |

## Rust core (Criterion)

String input via `country_code(&str)`:

| Case | IPv4 | IPv6 |
|---|---:|---:|
| AFRINIC `41.0.0.1` / `2001:4200::1` | 6.7 ns | 66.7 ns |
| APNIC `1.0.16.1` / `2001:200::1` | 9.1 ns | 65.7 ns |
| ARIN `8.8.8.8` / `2001:4860:4860::8888` | 7.8 ns | 76.3 ns |
| LACNIC `200.160.0.1` / `2001:1280::1` | 9.8 ns | 62.9 ns |
| RIPE NCC `193.0.6.139` / `2001:67c:18::1` | 12.9 ns | 73.5 ns |
| miss `10.0.0.0` / `::1` | 6.4 ns | 16.1 ns |

Typed input via `country_code_v4(Ipv4Addr)` / `country_code_v6(Ipv6Addr)`:

| Case | IPv4 | IPv6 |
|---|---:|---:|
| AFRINIC | 2.1 ns | 30.4 ns |
| APNIC | 5.4 ns | 30.4 ns |
| ARIN | 4.1 ns | 30.3 ns |
| LACNIC | 5.4 ns | 27.8 ns |
| RIPE NCC | 8.5 ns | 30.5 ns |
| miss | 1.9 ns | 1.1 ns |

```bash
task bench:rust
```

## Python binding (pytest-benchmark)

Single-call latency:

| Case | IPv4 | IPv6 |
|---|---:|---:|
| AFRINIC | 98.8 ns | 155.0 ns |
| APNIC | 103.6 ns | 156.2 ns |
| ARIN | 103.7 ns | 162.2 ns |
| LACNIC | 106.5 ns | 110.4 ns |
| RIPE NCC | 110.1 ns | 161.5 ns |
| miss | 37.8 ns | 91.7 ns |

Batch call, `country_code(list_of_N)`:

| N | Total | Per address |
|---|---:|---:|
| 10 | 672 ns | 67.2 ns |
| 100 | 3.25 us | 32.5 ns |
| 1,000 | 29.0 us | 29.0 ns |
| 10,000 | 302.8 us | 30.3 ns |

```bash
task bench:python
```

## WASM binding (mitata)

| Case | IPv4 | IPv6 |
|---|---:|---:|
| AFRINIC | 194.1 ns | 304.3 ns |
| APNIC | 169.9 ns | 301.8 ns |
| ARIN | 158.4 ns | 327.5 ns |
| LACNIC | 173.4 ns | 298.8 ns |
| RIPE NCC | 180.8 ns | 316.8 ns |
| miss | 100.9 ns | 123.9 ns |

```bash
task bench:wasm
```

## Run all

```bash
task bench
```
