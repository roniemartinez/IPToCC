use core::net::{Ipv4Addr, Ipv6Addr};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
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

fn bench_batch_v4(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_v4");
    for &n in &[10usize, 100, 1000, 10000] {
        let addrs: Vec<&str> = V4_CASES.iter().map(|(_, ip)| *ip).cycle().take(n).collect();
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(format!("n={n}"), &addrs, |b, addrs| {
            b.iter(|| iptocc::country_codes(black_box(addrs.iter().copied())))
        });
    }
    group.finish();
}

fn bench_batch_v4_typed(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_v4_typed");
    for &n in &[10usize, 100, 1000, 10000] {
        let addrs: Vec<Ipv4Addr> = V4_CASES
            .iter()
            .map(|(_, ip)| ip.parse().unwrap())
            .cycle()
            .take(n)
            .collect();
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(format!("n={n}"), &addrs, |b, addrs| {
            b.iter(|| iptocc::country_codes(black_box(addrs.iter().copied())))
        });
    }
    group.finish();
}

fn bench_batch_v6(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_v6");
    for &n in &[10usize, 100, 1000, 10000] {
        let addrs: Vec<&str> = V6_CASES.iter().map(|(_, ip)| *ip).cycle().take(n).collect();
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(format!("n={n}"), &addrs, |b, addrs| {
            b.iter(|| iptocc::country_codes(black_box(addrs.iter().copied())))
        });
    }
    group.finish();
}

fn bench_batch_v6_typed(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_v6_typed");
    for &n in &[10usize, 100, 1000, 10000] {
        let addrs: Vec<Ipv6Addr> = V6_CASES
            .iter()
            .map(|(_, ip)| ip.parse().unwrap())
            .cycle()
            .take(n)
            .collect();
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(format!("n={n}"), &addrs, |b, addrs| {
            b.iter(|| iptocc::country_codes(black_box(addrs.iter().copied())))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_v4,
    bench_v6,
    bench_v4_typed,
    bench_v6_typed,
    bench_batch_v4,
    bench_batch_v4_typed,
    bench_batch_v6,
    bench_batch_v6_typed
);
criterion_main!(benches);
