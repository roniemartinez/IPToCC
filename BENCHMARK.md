# Benchmarks

Per-call latency for `country_code(address)`, one well-known IP per RIR plus one negative case per protocol.

## Reference machine

| | |
|---|---|
| Model | Apple M3 Pro (11-core), 18 GiB |
| OS | macOS 26.3.1 |
| Rust / Python / Node | 1.94.1 / 3.14.3 / 22.15.0 |

## Rust core (Criterion)

| Case | IPv4 | IPv6 |
|---|---:|---:|
| AFRINIC `41.0.0.1` / `2001:4200::1` | 8.2 ns | 75.6 ns |
| APNIC `1.0.16.1` / `2001:200::1` | 9.3 ns | 69.7 ns |
| ARIN `8.8.8.8` / `2001:4860:4860::8888` | 7.5 ns | 81.1 ns |
| LACNIC `200.160.0.1` / `2001:1280::1` | 10.7 ns | 70.4 ns |
| RIPE NCC `193.0.6.139` / `2001:67c:18::1` | 12.7 ns | 77.9 ns |
| miss `10.0.0.0` / `::1` | 7.8 ns | 43.0 ns |

```bash
task bench:rust
```

## Python binding (pytest-benchmark)

Single-call latency:

| Case | IPv4 | IPv6 |
|---|---:|---:|
| AFRINIC | 101.3 ns | 165.3 ns |
| APNIC | 103.7 ns | 160.5 ns |
| ARIN | 100.7 ns | 167.2 ns |
| LACNIC | 107.9 ns | 117.1 ns |
| RIPE NCC | 113.9 ns | 163.8 ns |
| miss | 42.4 ns | 93.2 ns |

Batch call, `country_code(list_of_N)`:

| N | Total | Per address |
|---|---:|---:|
| 10 | 462 ns | 46.2 ns |
| 100 | 3.26 us | 32.6 ns |
| 1,000 | 31.4 us | 31.4 ns |
| 10,000 | 325.6 us | 32.6 ns |

```bash
task bench:python
```

## WASM binding (mitata)

| Case | IPv4 | IPv6 |
|---|---:|---:|
| AFRINIC | 194.1 ns | 321.8 ns |
| APNIC | 169.9 ns | 299.2 ns |
| ARIN | 158.4 ns | 323.3 ns |
| LACNIC | 173.4 ns | 283.5 ns |
| RIPE NCC | 180.8 ns | 287.1 ns |
| miss | 79.5 ns | 172.6 ns |

```bash
task bench:wasm
```

## Run all

```bash
task bench
```
