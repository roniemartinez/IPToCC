//! Fast offline IP-to-country lookup using RIR delegated statistics.

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use core::str::FromStr;

#[doc(hidden)]
pub mod format;

use format::{
    TAG_EMPTY, TAG_MASK, TAG_PURE, V4_CC_GAP, V6_BUCKET_COUNT, V6_BUCKET_EMPTY, V6_IRREGULAR, V6_REC_SIZE,
    V6_SIDE_SIZE, V6_SUB_INDEX_LEN,
};

static V4_BIN: &[u8] = include_bytes!("data/v4.bin");
static V6_BIN: &[u8] = include_bytes!("data/v6.bin");

struct V4Layout {
    cc_dict_offset: usize,
    mixed_base_offset: usize,
    mixed_initial_offset: usize,
    transition_offsets: usize,
    transition_codes: usize,
}

struct V6Layout {
    populated_count: usize,
    pop_first_offset: usize,
    sub_index_offset: usize,
    cc_dict_offset: usize,
    side_offset: usize,
    primary_offset: usize,
}

const V4: V4Layout = {
    let first_level_bytes = 65536 * 4;
    let cc_dict_count_offset = first_level_bytes;
    let cc_dict_count = read_u32(V4_BIN, cc_dict_count_offset) as usize;
    let cc_dict_offset = cc_dict_count_offset + 4;
    let mixed_count_offset = cc_dict_offset + cc_dict_count * 2;
    let mixed_count = read_u32(V4_BIN, mixed_count_offset) as usize;
    let mixed_base_offset = mixed_count_offset + 4;
    let mixed_initial_offset = mixed_base_offset + (mixed_count + 1) * 4;
    let transitions_count_offset = mixed_initial_offset + mixed_count;
    let transitions_count = read_u32(V4_BIN, transitions_count_offset) as usize;
    let transition_offsets = transitions_count_offset + 4;
    let transition_codes = transition_offsets + transitions_count * 2;
    V4Layout {
        cc_dict_offset,
        mixed_base_offset,
        mixed_initial_offset,
        transition_offsets,
        transition_codes,
    }
};

const V6: V6Layout = {
    let pop_count_offset = V6_BUCKET_COUNT;
    let populated_count = read_u32(V6_BIN, pop_count_offset) as usize;
    let pop_first_offset = pop_count_offset + 4;
    let sub_index_offset = pop_first_offset + (populated_count + 1) * 4;
    let cc_dict_count_offset = sub_index_offset + populated_count * V6_SUB_INDEX_LEN * 2;
    let cc_dict_count = read_u32(V6_BIN, cc_dict_count_offset) as usize;
    let cc_dict_offset = cc_dict_count_offset + 4;
    let side_count_offset = cc_dict_offset + cc_dict_count * 2;
    let side_count = read_u32(V6_BIN, side_count_offset) as usize;
    let side_offset = side_count_offset + 4;
    let primary_count_offset = side_offset + side_count * V6_SIDE_SIZE;
    let primary_offset = primary_count_offset + 4;
    V6Layout {
        populated_count,
        pop_first_offset,
        sub_index_offset,
        cc_dict_offset,
        side_offset,
        primary_offset,
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
fn v4_cc(cc_idx: u8) -> &'static str {
    let co = V4.cc_dict_offset + (cc_idx as usize) * 2;
    // SAFETY: dict entries are ASCII uppercase pairs by generator construction.
    unsafe { core::str::from_utf8_unchecked(&V4_BIN[co..co + 2]) }
}

fn lookup_v4(ip: Ipv4Addr) -> Option<&'static str> {
    let key = u32::from(ip);
    let bucket = (key >> 16) as usize;
    let fl = read_u32(V4_BIN, bucket * 4);
    let tag = fl & TAG_MASK;

    if tag == TAG_EMPTY {
        return None;
    }
    if tag == TAG_PURE {
        return Some(v4_cc((fl & 0xFF) as u8));
    }

    let mixed_idx = (fl & !TAG_MASK) as usize;
    let base_o = V4.mixed_base_offset + mixed_idx * 4;
    let lo = read_u32(V4_BIN, base_o) as usize;
    let hi = read_u32(V4_BIN, base_o + 4) as usize;
    let local_key = (key & 0xFFFF) as u16;

    let idx = partition_point(lo, hi, |i| read_u16(V4_BIN, V4.transition_offsets + i * 2) <= local_key);
    let cc_idx = if idx == lo {
        V4_BIN[V4.mixed_initial_offset + mixed_idx]
    } else {
        V4_BIN[V4.transition_codes + (idx - 1)]
    };
    if cc_idx == V4_CC_GAP {
        return None;
    }
    Some(v4_cc(cc_idx))
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
    let bucket_prefix = (bucket as u128) << 112;
    let idx = partition_point(lo, hi, |i| v6_entry_start(i, bucket_prefix) <= key);
    if idx == lo {
        return None;
    }
    let i = idx - 1;
    let o = V6.primary_offset + i * V6_REC_SIZE;
    let prefix_len = V6_BIN[o + 4];
    let (end, cc_idx) = if prefix_len == V6_IRREGULAR {
        let side_idx = read_u32(V6_BIN, o) as usize;
        let side_o = V6.side_offset + side_idx * V6_SIDE_SIZE;
        (read_u128(V6_BIN, side_o + 16), V6_BIN[side_o + 32])
    } else {
        let offset = read_u32(V6_BIN, o) as u128;
        let start = bucket_prefix | (offset << 80);
        let host = 128u32 - prefix_len as u32;
        let mask = (1u128 << host) - 1;
        (start | mask, V6_BIN[o + 5])
    };
    if end < key {
        return None;
    }
    let co = V6.cc_dict_offset + (cc_idx as usize) * 2;
    // SAFETY: dict entries are ASCII uppercase pairs by generator construction.
    Some(unsafe { core::str::from_utf8_unchecked(&V6_BIN[co..co + 2]) })
}

#[inline]
fn v6_entry_start(i: usize, bucket_prefix: u128) -> u128 {
    let o = V6.primary_offset + i * V6_REC_SIZE;
    let prefix_len = V6_BIN[o + 4];
    if prefix_len == V6_IRREGULAR {
        let side_idx = read_u32(V6_BIN, o) as usize;
        read_u128(V6_BIN, V6.side_offset + side_idx * V6_SIDE_SIZE)
    } else {
        let offset = read_u32(V6_BIN, o) as u128;
        bucket_prefix | (offset << 80)
    }
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
