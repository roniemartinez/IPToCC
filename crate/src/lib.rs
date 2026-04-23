//! Fast offline IP-to-country lookup using RIR delegated statistics.

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use core::str::FromStr;

#[doc(hidden)]
pub mod format;

use format::{
    BUCKET_COUNT, TAG_EMPTY, TAG_MASK, TAG_PURE, UNASSIGNED, V4_DEEP_TOP4_LEN, V6_BYTE_INDEX_LEN, V6_DENSE_THRESHOLD,
    V6_IRREGULAR, V6_REC_SIZE, V6_SIDE_SIZE,
};

static V4_BIN: &[u8] = include_bytes!("data/v4.bin");
static V6_BIN: &[u8] = include_bytes!("data/v6.bin");

struct V4Layout {
    cc_dict: usize,
    mixed_base: usize,
    mixed_initial: usize,
    transition_offsets: usize,
    transition_codes: usize,
    dense_mixed_count: usize,
    deep_narrow: usize,
}

struct DeepIndex {
    count: usize,
    keys: usize,
    pairs: usize,
}

struct V6Layout {
    populated_first: usize,
    sub_index: usize,
    deep_32: DeepIndex,
    deep_40: DeepIndex,
    cc_dict: usize,
    side: usize,
    primary: usize,
}

const V4: V4Layout = {
    let first_level_bytes = BUCKET_COUNT * 4;
    let cc_dict_count = read_u32(V4_BIN, first_level_bytes) as usize;
    let cc_dict = first_level_bytes + 4;
    let mixed_count_at = cc_dict + cc_dict_count * 2;
    let mixed_count = read_u32(V4_BIN, mixed_count_at) as usize;
    let mixed_base = mixed_count_at + 4;
    let mixed_initial = mixed_base + (mixed_count + 1) * 4;
    let transitions_count_at = mixed_initial + mixed_count;
    let transitions_count = read_u32(V4_BIN, transitions_count_at) as usize;
    let transition_offsets = transitions_count_at + 4;
    let transition_codes = transition_offsets + transitions_count * 2;
    let dense_count_at = transition_codes + transitions_count;
    let dense_mixed_count = read_u32(V4_BIN, dense_count_at) as usize;
    let deep_narrow = dense_count_at + 4;
    V4Layout {
        cc_dict,
        mixed_base,
        mixed_initial,
        transition_offsets,
        transition_codes,
        dense_mixed_count,
        deep_narrow,
    }
};

const V6: V6Layout = {
    const DEEP_PAIRS_BYTES_PER: usize = V6_BYTE_INDEX_LEN * 2 * 2;

    let populated_count = read_u32(V6_BIN, BUCKET_COUNT) as usize;
    let populated_first = BUCKET_COUNT + 4;
    let sub_index = populated_first + (populated_count + 1) * 4;

    let deep_32_count_at = sub_index + populated_count * V6_BYTE_INDEX_LEN * 2;
    let deep_32 = deep_index_at(V6_BIN, deep_32_count_at);
    let deep_40_count_at = deep_32.pairs + deep_32.count * DEEP_PAIRS_BYTES_PER;
    let deep_40 = deep_index_at(V6_BIN, deep_40_count_at);

    let cc_dict_count_at = deep_40.pairs + deep_40.count * DEEP_PAIRS_BYTES_PER;
    let cc_dict_count = read_u32(V6_BIN, cc_dict_count_at) as usize;
    let cc_dict = cc_dict_count_at + 4;
    let side_count_at = cc_dict + cc_dict_count * 2;
    let side_count = read_u32(V6_BIN, side_count_at) as usize;
    let side = side_count_at + 4;
    let primary_count_at = side + side_count * V6_SIDE_SIZE;
    let primary = primary_count_at + 4;

    V6Layout {
        populated_first,
        sub_index,
        deep_32,
        deep_40,
        cc_dict,
        side,
        primary,
    }
};

const fn deep_index_at(buf: &[u8], count_offset: usize) -> DeepIndex {
    let count = read_u32(buf, count_offset) as usize;
    let keys = count_offset + 4;
    let pairs = keys + count * 2;
    DeepIndex { count, keys, pairs }
}

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
    cc_from_dict(V4_BIN, V4.cc_dict, cc_idx)
}

#[inline]
fn v6_cc(cc_idx: u8) -> &'static str {
    cc_from_dict(V6_BIN, V6.cc_dict, cc_idx)
}

#[inline]
fn cc_from_dict(buf: &'static [u8], dict_offset: usize, cc_idx: u8) -> &'static str {
    let entry = dict_offset + (cc_idx as usize) * 2;
    // SAFETY: dict entries are ASCII uppercase pairs by generator construction.
    unsafe { core::str::from_utf8_unchecked(&buf[entry..entry + 2]) }
}

#[inline]
fn lookup_v4(ip: Ipv4Addr) -> Option<&'static str> {
    let ip_u32 = u32::from(ip);
    let top_16 = (ip_u32 >> 16) as usize;
    let first_level = read_u32(V4_BIN, top_16 * 4);
    let tag = first_level & TAG_MASK;

    if tag == TAG_EMPTY {
        return None;
    }
    if tag == TAG_PURE {
        return Some(v4_cc((first_level & 0xFF) as u8));
    }

    let mixed_idx = (first_level & !TAG_MASK) as usize;
    let base = V4.mixed_base + mixed_idx * 4;
    let bucket_start = read_u32(V4_BIN, base) as usize;
    let bucket_end = read_u32(V4_BIN, base + 4) as usize;
    let within_bucket = (ip_u32 & 0xFFFF) as u16;

    let (search_start, search_end) = narrow_v4(mixed_idx, within_bucket, bucket_start, bucket_end);
    let pivot = partition_point(search_start, search_end, |i| {
        read_u16(V4_BIN, V4.transition_offsets + i * 2) <= within_bucket
    });

    let cc_idx = if pivot == bucket_start {
        V4_BIN[V4.mixed_initial + mixed_idx]
    } else {
        V4_BIN[V4.transition_codes + (pivot - 1)]
    };
    if cc_idx == UNASSIGNED {
        return None;
    }
    Some(v4_cc(cc_idx))
}

#[inline]
fn narrow_v4(mixed_idx: usize, within_bucket: u16, bucket_start: usize, bucket_end: usize) -> (usize, usize) {
    if mixed_idx >= V4.dense_mixed_count {
        return (bucket_start, bucket_end);
    }
    let top_4 = (within_bucket >> 12) as usize;
    let pair = V4.deep_narrow + (mixed_idx * V4_DEEP_TOP4_LEN + top_4) * 2;
    let offset_start = read_u16(V4_BIN, pair) as usize;
    let offset_end = read_u16(V4_BIN, pair + 2) as usize;
    (bucket_start + offset_start, bucket_start + offset_end)
}

#[inline]
fn lookup_v6(ip: Ipv6Addr) -> Option<&'static str> {
    let ip_u128 = u128::from(ip);
    let top_16 = (ip_u128 >> 112) as usize;
    let populated_idx = V6_BIN[top_16];
    if populated_idx == UNASSIGNED {
        return None;
    }

    let byte_24 = ((ip_u128 >> 104) & 0xFF) as usize;
    let (mut search_start, mut search_end) = v6_sub_range(populated_idx as usize, byte_24)?;

    if search_end - search_start > V6_DENSE_THRESHOLD {
        let deep_32_key = ((populated_idx as u16) << 8) | byte_24 as u16;
        if let Some(deep_32_idx) = deep_find(&V6.deep_32, deep_32_key) {
            let byte_32 = ((ip_u128 >> 96) & 0xFF) as usize;
            (search_start, search_end) = narrow_v6(&V6.deep_32, deep_32_idx, byte_32, search_start);

            if search_end - search_start > V6_DENSE_THRESHOLD {
                let deep_40_key = ((deep_32_idx as u16) << 8) | byte_32 as u16;
                if let Some(deep_40_idx) = deep_find(&V6.deep_40, deep_40_key) {
                    let byte_40 = ((ip_u128 >> 88) & 0xFF) as usize;
                    (search_start, search_end) = narrow_v6(&V6.deep_40, deep_40_idx, byte_40, search_start);
                }
            }
        }
    }

    let bucket_prefix = (top_16 as u128) << 112;
    let pivot = partition_point(search_start, search_end, |i| {
        v6_entry_start(i, bucket_prefix) <= ip_u128
    });
    if pivot == search_start {
        return None;
    }

    let (entry_end, cc_idx) = v6_entry_end_and_cc(pivot - 1, bucket_prefix);
    if entry_end < ip_u128 {
        return None;
    }
    Some(v6_cc(cc_idx))
}

// Linear scan: with ~20-30 keys (one cache line) it beats binary search on branch prediction.
#[inline]
fn deep_find(index: &DeepIndex, key: u16) -> Option<usize> {
    (0..index.count).find(|&i| read_u16(V6_BIN, index.keys + i * 2) == key)
}

#[inline]
fn narrow_v6(index: &DeepIndex, deep_idx: usize, byte: usize, window_start: usize) -> (usize, usize) {
    let pair = index.pairs + (deep_idx * V6_BYTE_INDEX_LEN + byte) * 4;
    let offset_start = read_u16(V6_BIN, pair) as usize;
    let offset_end = read_u16(V6_BIN, pair + 2) as usize;
    (window_start + offset_start, window_start + offset_end)
}

#[inline]
fn v6_sub_range(populated_idx: usize, byte_24: usize) -> Option<(usize, usize)> {
    let entry = V6.sub_index + populated_idx * V6_BYTE_INDEX_LEN * 2 + byte_24 * 2;
    let sub_start = read_u16(V6_BIN, entry) as usize;
    let sub_end = read_u16(V6_BIN, entry + 2) as usize;
    if sub_start == sub_end {
        return None;
    }
    let bucket_start = read_u32(V6_BIN, V6.populated_first + populated_idx * 4) as usize;
    Some((bucket_start + sub_start, bucket_start + sub_end))
}

// Primary record layout (6 bytes): u32 offset_or_side_idx, u8 prefix_len, u8 cc_idx.
// Side record layout (33 bytes): u128 start, u128 end, u8 cc_idx.
#[inline]
fn v6_entry_start(entry_idx: usize, bucket_prefix: u128) -> u128 {
    let record = V6.primary + entry_idx * V6_REC_SIZE;
    let prefix_len = V6_BIN[record + 4];
    if prefix_len == V6_IRREGULAR {
        let side_idx = read_u32(V6_BIN, record) as usize;
        read_u128(V6_BIN, V6.side + side_idx * V6_SIDE_SIZE)
    } else {
        let offset = read_u32(V6_BIN, record) as u128;
        bucket_prefix | (offset << 80)
    }
}

#[inline]
fn v6_entry_end_and_cc(entry_idx: usize, bucket_prefix: u128) -> (u128, u8) {
    let record = V6.primary + entry_idx * V6_REC_SIZE;
    let prefix_len = V6_BIN[record + 4];
    if prefix_len == V6_IRREGULAR {
        let side = V6.side + (read_u32(V6_BIN, record) as usize) * V6_SIDE_SIZE;
        (read_u128(V6_BIN, side + 16), V6_BIN[side + 32])
    } else {
        let offset = read_u32(V6_BIN, record) as u128;
        let start = bucket_prefix | (offset << 80);
        let host_bits = 128 - prefix_len as u32;
        (start | ((1u128 << host_bits) - 1), V6_BIN[record + 5])
    }
}

fn partition_point(start: usize, end: usize, pred: impl Fn(usize) -> bool) -> usize {
    let mut left = start;
    let mut right = end;
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
