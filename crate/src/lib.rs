//! Fast offline IP-to-country lookup using RIR delegated statistics.

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use core::str::FromStr;

static V4_BIN: &[u8] = include_bytes!("data/v4.bin");
static V6_BIN: &[u8] = include_bytes!("data/v6.bin");

const FIRST_LEVEL_BYTES: usize = 65537 * 4;
const V4_COUNT: usize = (V4_BIN.len() - FIRST_LEVEL_BYTES) / 10;
const V6_COUNT: usize = (V6_BIN.len() - FIRST_LEVEL_BYTES) / 34;

/// Looks up the ISO 3166-1 alpha-2 country code for an IPv4 or IPv6 address.
///
/// Returns `None` if the address is not found in any RIR's delegated range,
/// or if the input cannot be parsed as an IP address.
pub fn country_code(addr: &str) -> Option<&'static str> {
    match IpAddr::from_str(addr).ok()? {
        IpAddr::V4(ip) => lookup_v4(ip),
        IpAddr::V6(ip) => lookup_v6(ip),
    }
}

/// Looks up the ISO 3166-1 alpha-2 country code for an already-parsed IPv4 address.
///
/// Faster than `country_code(&str)` because it skips the string parse. Use this
/// when you already have an `Ipv4Addr` from elsewhere in your program.
pub fn country_code_v4(ip: Ipv4Addr) -> Option<&'static str> {
    lookup_v4(ip)
}

/// Looks up the ISO 3166-1 alpha-2 country code for an already-parsed IPv6 address.
///
/// Faster than `country_code(&str)` because it skips the string parse. Use this
/// when you already have an `Ipv6Addr` from elsewhere in your program.
pub fn country_code_v6(ip: Ipv6Addr) -> Option<&'static str> {
    lookup_v6(ip)
}

/// Looks up ISO 3166-1 alpha-2 country codes for a sequence of addresses.
///
/// Each element in the returned `Vec` corresponds to the input at the same
/// position: `Some("CC")` on a hit, `None` on a miss or parse failure.
pub fn country_codes<I>(addrs: I) -> Vec<Option<&'static str>>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    addrs.into_iter().map(|a| country_code(a.as_ref())).collect()
}

fn lookup_v4(ip: Ipv4Addr) -> Option<&'static str> {
    let key = u32::from(ip);
    let bucket = (key >> 16) as usize;
    let lo = (read_u32(V4_BIN, bucket * 4) as usize).saturating_sub(1);
    let hi = read_u32(V4_BIN, (bucket + 1) * 4) as usize;
    let idx = partition_point(lo, hi, |i| v4_start(i) <= key);
    if idx == lo {
        return None;
    }
    let i = idx - 1;
    (v4_end(i) >= key).then(|| v4_code(i))
}

fn lookup_v6(ip: Ipv6Addr) -> Option<&'static str> {
    let key = u128::from(ip);
    let bucket = (key >> 112) as usize;
    let lo = (read_u32(V6_BIN, bucket * 4) as usize).saturating_sub(1);
    let hi = read_u32(V6_BIN, (bucket + 1) * 4) as usize;
    let idx = partition_point(lo, hi, |i| v6_start(i) <= key);
    if idx == lo {
        return None;
    }
    let i = idx - 1;
    (v6_end(i) >= key).then(|| v6_code(i))
}

fn partition_point(lo: usize, hi: usize, pred: impl Fn(usize) -> bool) -> usize {
    let mut left = lo;
    let mut right = hi;
    while left < right {
        let mid = left + (right - left) / 2;
        if pred(mid) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn read_u32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

fn read_u128(buf: &[u8], off: usize) -> u128 {
    u128::from_le_bytes(buf[off..off + 16].try_into().unwrap())
}

fn read_code(buf: &'static [u8], off: usize) -> &'static str {
    // SAFETY: codes validated as ASCII uppercase pairs by the generator.
    unsafe { core::str::from_utf8_unchecked(&buf[off..off + 2]) }
}

fn v4_start(i: usize) -> u32 {
    read_u32(V4_BIN, FIRST_LEVEL_BYTES + i * 4)
}

fn v4_end(i: usize) -> u32 {
    read_u32(V4_BIN, FIRST_LEVEL_BYTES + V4_COUNT * 4 + i * 4)
}

fn v4_code(i: usize) -> &'static str {
    read_code(V4_BIN, FIRST_LEVEL_BYTES + V4_COUNT * 8 + i * 2)
}

fn v6_start(i: usize) -> u128 {
    read_u128(V6_BIN, FIRST_LEVEL_BYTES + i * 16)
}

fn v6_end(i: usize) -> u128 {
    read_u128(V6_BIN, FIRST_LEVEL_BYTES + V6_COUNT * 16 + i * 16)
}

fn v6_code(i: usize) -> &'static str {
    read_code(V6_BIN, FIRST_LEVEL_BYTES + V6_COUNT * 32 + i * 2)
}
