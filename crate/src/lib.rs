//! Fast offline IP-to-country lookup using RIR delegated statistics.

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use core::str::FromStr;

#[doc(hidden)]
pub mod format;

use format::{FIRST_LEVEL_COUNT, V4_GAP_SENTINEL, V6_BUCKET_COUNT, V6_BUCKET_EMPTY, V6_SUB_INDEX_LEN};

static V4_BIN: &[u8] = include_bytes!("data/v4.bin");
static V6_BIN: &[u8] = include_bytes!("data/v6.bin");

struct V4Layout {
    starts_offset: usize,
    codes_offset: usize,
}

struct V6Layout {
    populated_count: usize,
    pop_first_offset: usize,
    sub_index_offset: usize,
    starts_offset: usize,
    ends_offset: usize,
    codes_offset: usize,
}

const V4: V4Layout = {
    let first_level_bytes = FIRST_LEVEL_COUNT * 4;
    let count = (V4_BIN.len() - first_level_bytes) / 6;
    V4Layout {
        starts_offset: first_level_bytes,
        codes_offset: first_level_bytes + count * 4,
    }
};

const V6: V6Layout = {
    let pop_count_offset = V6_BUCKET_COUNT;
    let populated_count = read_u32(V6_BIN, pop_count_offset) as usize;
    let pop_first_offset = pop_count_offset + 4;
    let count = read_u32(V6_BIN, pop_first_offset + populated_count * 4) as usize;
    let sub_index_offset = pop_first_offset + (populated_count + 1) * 4;
    let starts_offset = sub_index_offset + populated_count * V6_SUB_INDEX_LEN * 2;
    let ends_offset = starts_offset + count * 16;
    let codes_offset = ends_offset + count * 16;
    V6Layout {
        populated_count,
        pop_first_offset,
        sub_index_offset,
        starts_offset,
        ends_offset,
        codes_offset,
    }
};

const _: () = assert!(V6.populated_count < 256, "v6 populated bucket count exceeds u8 range");

mod sealed {
    pub trait Sealed {}
}

#[doc(hidden)]
pub trait IpAddress: sealed::Sealed {
    fn lookup(self) -> Option<&'static str>;
}

impl sealed::Sealed for &str {}
impl IpAddress for &str {
    #[inline]
    fn lookup(self) -> Option<&'static str> {
        IpAddr::from_str(self).ok().and_then(IpAddress::lookup)
    }
}

impl sealed::Sealed for String {}
impl IpAddress for String {
    #[inline]
    fn lookup(self) -> Option<&'static str> {
        self.as_str().lookup()
    }
}

impl sealed::Sealed for &String {}
impl IpAddress for &String {
    #[inline]
    fn lookup(self) -> Option<&'static str> {
        self.as_str().lookup()
    }
}

impl sealed::Sealed for Ipv4Addr {}
impl IpAddress for Ipv4Addr {
    #[inline]
    fn lookup(self) -> Option<&'static str> {
        lookup_v4(self)
    }
}

impl sealed::Sealed for Ipv6Addr {}
impl IpAddress for Ipv6Addr {
    #[inline]
    fn lookup(self) -> Option<&'static str> {
        lookup_v6(self)
    }
}

impl sealed::Sealed for IpAddr {}
impl IpAddress for IpAddr {
    #[inline]
    fn lookup(self) -> Option<&'static str> {
        match self {
            IpAddr::V4(ip) => lookup_v4(ip),
            IpAddr::V6(ip) => lookup_v6(ip),
        }
    }
}

/// Looks up the ISO 3166-1 alpha-2 country code for an IP address.
///
/// Accepts a string slice, owned `String`, `Ipv4Addr`, `Ipv6Addr`, or `IpAddr`.
/// Returns `None` if the address is not in any RIR delegated range, or if the
/// input is a string that cannot be parsed as an IP address.
pub fn country_code<T: IpAddress>(input: T) -> Option<&'static str> {
    input.lookup()
}

/// Looks up ISO 3166-1 alpha-2 country codes for a sequence of addresses.
///
/// Each element in the returned `Vec` corresponds to the input at the same
/// position: `Some("CC")` on a hit, `None` on a miss or unparseable string.
pub fn country_codes<I, T>(inputs: I) -> Vec<Option<&'static str>>
where
    I: IntoIterator<Item = T>,
    T: IpAddress,
{
    inputs.into_iter().map(IpAddress::lookup).collect()
}

#[inline]
fn read_code(buf: &'static [u8], offset: usize) -> Option<&'static str> {
    let bytes = buf[offset..offset + 2].first_chunk::<2>().unwrap();
    if *bytes == V4_GAP_SENTINEL {
        return None;
    }
    // SAFETY: codes are ASCII uppercase pairs by generator construction.
    Some(unsafe { core::str::from_utf8_unchecked(bytes) })
}

fn lookup_v4(ip: Ipv4Addr) -> Option<&'static str> {
    let key = u32::from(ip);
    let bucket = (key >> 16) as usize;
    let (lo, hi) = v4_bucket_range(bucket);
    let idx = partition_point(lo, hi, |i| v4_start(i) <= key);
    if idx == lo {
        return None;
    }
    read_code(V4_BIN, V4.codes_offset + (idx - 1) * 2)
}

fn lookup_v6(ip: Ipv6Addr) -> Option<&'static str> {
    let key = u128::from(ip);
    let bucket = (key >> 112) as usize;
    let bucket_idx = V6_BIN[bucket];
    if bucket_idx == V6_BUCKET_EMPTY {
        return None;
    }
    let sub_byte = ((key >> 104) & 0xFF) as usize;
    let (lo, hi) = v6_sub_range(bucket_idx as usize, sub_byte)?;
    let idx = partition_point(lo, hi, |i| v6_start(i) <= key);
    if idx == lo {
        return None;
    }
    let i = idx - 1;
    if v6_end(i) < key {
        return None;
    }
    read_code(V6_BIN, V6.codes_offset + i * 2)
}

#[inline]
fn v4_bucket_range(bucket: usize) -> (usize, usize) {
    let lo = (read_u32(V4_BIN, bucket * 4) as usize).saturating_sub(1);
    let hi = read_u32(V4_BIN, (bucket + 1) * 4) as usize;
    (lo, hi)
}

#[inline]
fn v6_sub_range(populated_idx: usize, sub_byte: usize) -> Option<(usize, usize)> {
    let sub_offset = V6.sub_index_offset + populated_idx * V6_SUB_INDEX_LEN * 2 + sub_byte * 2;
    let sub_lo = read_u16(V6_BIN, sub_offset) as usize;
    let sub_hi = read_u16(V6_BIN, sub_offset + 2) as usize;
    if sub_lo == sub_hi {
        return None;
    }
    let bucket_lo = read_u32(V6_BIN, V6.pop_first_offset + populated_idx * 4) as usize;
    Some((bucket_lo + sub_lo, bucket_lo + sub_hi))
}

// Hand-rolled because the bin file is `&[u8]` (1-byte aligned) and stdlib's
// `slice::partition_point` requires a typed slice; `&[u32]` / `&[u128]` would
// need an alignment-checked transmute that the compiler cannot prove safe from
// `include_bytes!`.
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

#[inline]
const fn read_u16(buf: &[u8], offset: usize) -> u16 {
    let bytes = buf.split_at(offset).1.first_chunk::<2>().unwrap();
    u16::from_le_bytes(*bytes)
}

#[inline]
const fn read_u32(buf: &[u8], offset: usize) -> u32 {
    let bytes = buf.split_at(offset).1.first_chunk::<4>().unwrap();
    u32::from_le_bytes(*bytes)
}

#[inline]
const fn read_u128(buf: &[u8], offset: usize) -> u128 {
    let bytes = buf.split_at(offset).1.first_chunk::<16>().unwrap();
    u128::from_le_bytes(*bytes)
}

#[inline]
fn v4_start(i: usize) -> u32 {
    read_u32(V4_BIN, V4.starts_offset + i * 4)
}

#[inline]
fn v6_start(i: usize) -> u128 {
    read_u128(V6_BIN, V6.starts_offset + i * 16)
}

#[inline]
fn v6_end(i: usize) -> u128 {
    read_u128(V6_BIN, V6.ends_offset + i * 16)
}
