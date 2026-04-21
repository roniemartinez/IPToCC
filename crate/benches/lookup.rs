use core::net::{Ipv4Addr, Ipv6Addr};

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const V4_CASES: &[(&str, &str)] = &[
    ("afrinic", "41.0.0.1"),
    ("apnic", "1.0.16.1"),
    ("arin", "8.8.8.8"),
    ("lacnic", "200.160.0.1"),
    ("ripencc", "193.0.6.139"),
    ("miss_private", "10.0.0.0"),
];

const V6_CASES: &[(&str, &str)] = &[
    ("afrinic", "2001:4200::1"),
    ("apnic", "2001:200::1"),
    ("arin", "2001:4860:4860::8888"),
    ("lacnic", "2001:1280::1"),
    ("ripencc", "2001:67c:18::1"),
    ("miss_loopback", "::1"),
];

fn bench_v4(c: &mut Criterion) {
    let mut group = c.benchmark_group("v4");
    for (name, ip) in V4_CASES {
        group.bench_with_input(*name, ip, |b, &ip| b.iter(|| iptocc::country_code(black_box(ip))));
    }
    group.finish();
}

fn bench_v6(c: &mut Criterion) {
    let mut group = c.benchmark_group("v6");
    for (name, ip) in V6_CASES {
        group.bench_with_input(*name, ip, |b, &ip| b.iter(|| iptocc::country_code(black_box(ip))));
    }
    group.finish();
}

fn bench_v4_typed(c: &mut Criterion) {
    let mut group = c.benchmark_group("v4_typed");
    for (name, ip) in V4_CASES {
        let parsed: Ipv4Addr = ip.parse().unwrap();
        group.bench_with_input(*name, &parsed, |b, &ip| b.iter(|| iptocc::country_code(black_box(ip))));
    }
    group.finish();
}

fn bench_v6_typed(c: &mut Criterion) {
    let mut group = c.benchmark_group("v6_typed");
    for (name, ip) in V6_CASES {
        let parsed: Ipv6Addr = ip.parse().unwrap();
        group.bench_with_input(*name, &parsed, |b, &ip| b.iter(|| iptocc::country_code(black_box(ip))));
    }
    group.finish();
}

criterion_group!(benches, bench_v4, bench_v6, bench_v4_typed, bench_v6_typed);
criterion_main!(benches);
