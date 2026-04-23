use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::{Add, Sub};
use std::path::Path;
use std::str::FromStr;

use iptocc::format::{
    BUCKET_COUNT, TAG_EMPTY, TAG_MIXED, TAG_PURE, UNASSIGNED, V4_DEEP_TOP4_LEN, V4_DENSE_THRESHOLD, V6_BYTE_INDEX_LEN,
    V6_DENSE_THRESHOLD, V6_IRREGULAR, V6_REC_SIZE, V6_SIDE_SIZE,
};

const REGISTRIES: &[&str] = &["afrinic", "apnic", "arin", "lacnic", "ripencc"];

trait Interval: Copy {
    type Addr: Copy + Ord + Add<Output = Self::Addr> + Sub<Output = Self::Addr>;
    const ONE: Self::Addr;
    fn start(&self) -> Self::Addr;
    fn end(&self) -> Self::Addr;
    fn cc(&self) -> [u8; 2];
    fn new(start: Self::Addr, end: Self::Addr, cc: [u8; 2]) -> Self;
    fn set_end(&mut self, end: Self::Addr);
}

#[derive(Clone, Copy)]
struct V4Interval {
    start: u32,
    end: u32,
    cc: [u8; 2],
}

#[derive(Clone, Copy)]
struct V6Interval {
    start: u128,
    end: u128,
    cc: [u8; 2],
}

impl Interval for V4Interval {
    type Addr = u32;
    const ONE: u32 = 1;
    fn start(&self) -> u32 {
        self.start
    }
    fn end(&self) -> u32 {
        self.end
    }
    fn cc(&self) -> [u8; 2] {
        self.cc
    }
    fn new(start: u32, end: u32, cc: [u8; 2]) -> Self {
        V4Interval { start, end, cc }
    }
    fn set_end(&mut self, end: u32) {
        self.end = end;
    }
}

impl Interval for V6Interval {
    type Addr = u128;
    const ONE: u128 = 1;
    fn start(&self) -> u128 {
        self.start
    }
    fn end(&self) -> u128 {
        self.end
    }
    fn cc(&self) -> [u8; 2] {
        self.cc
    }
    fn new(start: u128, end: u128, cc: [u8; 2]) -> Self {
        V6Interval { start, end, cc }
    }
    fn set_end(&mut self, end: u128) {
        self.end = end;
    }
}

struct MixedBucket {
    bucket: usize,
    initial_cc: u8,
    transitions: Vec<(u16, u8)>,
}

struct MixedLayout {
    mixed_base: Vec<u32>,
    mixed_initial: Vec<u8>,
    transitions: Vec<(u16, u8)>,
    deep_v4: Vec<u16>,
    dense_count: u32,
}

struct V6DeepLayout {
    sub_index: Vec<u16>,
    dense_keys: Vec<u16>,
    deep_pairs: Vec<u16>,
    dense2_keys: Vec<u16>,
    deep2_pairs: Vec<u16>,
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir).parent().expect("workspace root");
    let data_dir = workspace_root.join("data");
    let out_dir = Path::new(manifest_dir).join("src/data");
    fs::create_dir_all(&out_dir).expect("creating src/data");

    let (v4, v6) = parse_rir(&data_dir);
    let v4 = resolve_overlaps(v4);
    let v6 = resolve_overlaps(v6);
    assert_sorted_disjoint(&v4);
    assert_sorted_disjoint(&v6);
    assert_ccs_ascii_uppercase(&v4, &v6);

    let v4_bin = transform_v4(&v4);
    let v6_bin = transform_v6(&v6);

    let v4_path = out_dir.join("v4.bin");
    let v6_path = out_dir.join("v6.bin");
    fs::write(&v4_path, &v4_bin).expect("writing v4.bin");
    fs::write(&v6_path, &v6_bin).expect("writing v6.bin");

    let unique_codes: BTreeSet<[u8; 2]> = v4.iter().map(|e| e.cc).chain(v6.iter().map(|e| e.cc)).collect();
    println!(
        "wrote {} ({} bytes), {} ({} bytes); {} v4 intervals, {} v6 intervals, {} countries",
        v4_path.display(),
        v4_bin.len(),
        v6_path.display(),
        v6_bin.len(),
        v4.len(),
        v6.len(),
        unique_codes.len()
    );
}

fn assert_sorted_disjoint<I: Interval>(intervals: &[I]) {
    assert!(
        intervals.windows(2).all(|w| w[0].end() < w[1].start()),
        "intervals still overlap after resolution"
    );
}

fn assert_ccs_ascii_uppercase(v4: &[V4Interval], v6: &[V6Interval]) {
    for entry in v4.iter().map(|e| e.cc).chain(v6.iter().map(|e| e.cc)) {
        assert!(
            entry.iter().all(|b| b.is_ascii_uppercase()),
            "non-ASCII-uppercase code: {entry:?}"
        );
    }
}

fn parse_rir(data_dir: &Path) -> (Vec<V4Interval>, Vec<V6Interval>) {
    let mut v4 = Vec::new();
    let mut v6 = Vec::new();
    for registry in REGISTRIES {
        let path = data_dir.join(format!("delegated-{registry}-extended-latest"));
        parse_registry_file(&path, &mut v4, &mut v6);
    }
    (v4, v6)
}

fn parse_registry_file(path: &Path, v4: &mut Vec<V4Interval>, v6: &mut Vec<V6Interval>) {
    let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    for line in content.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 7 {
            continue;
        }
        let cc = parts[1];
        let kind = parts[2];
        let start = parts[3];
        let value = parts[4];
        let status = parts[6];

        if cc.len() != 2 || !cc.bytes().all(|b| b.is_ascii_uppercase()) || cc == "ZZ" {
            continue;
        }
        if status != "allocated" && status != "assigned" {
            continue;
        }
        let cc_bytes: [u8; 2] = cc.as_bytes().try_into().unwrap();

        match kind {
            "ipv4" => {
                if let Some(interval) = parse_v4_line(start, value, cc_bytes) {
                    v4.push(interval);
                }
            }
            "ipv6" => {
                if let Some(interval) = parse_v6_line(start, value, cc_bytes) {
                    v6.push(interval);
                }
            }
            _ => {}
        }
    }
}

fn parse_v4_line(start: &str, value: &str, cc: [u8; 2]) -> Option<V4Interval> {
    let start_ip = Ipv4Addr::from_str(start).ok()?;
    let count = value.parse::<u32>().ok()?;
    if count == 0 {
        return None;
    }
    let s = u32::from(start_ip);
    let e = s.checked_add(count - 1)?;
    Some(V4Interval { start: s, end: e, cc })
}

fn parse_v6_line(start: &str, value: &str, cc: [u8; 2]) -> Option<V6Interval> {
    let start_ip = Ipv6Addr::from_str(start).ok()?;
    let prefix = value.parse::<u32>().ok()?;
    if prefix == 0 || prefix > 128 {
        return None;
    }
    let s = u128::from(start_ip);
    let host_bits = 128 - prefix;
    let mask = (1u128 << host_bits) - 1;
    Some(V6Interval {
        start: s,
        end: s | mask,
        cc,
    })
}

fn resolve_overlaps<I: Interval>(mut intervals: Vec<I>) -> Vec<I> {
    intervals.sort_by_key(|t| t.start());
    let mut out: Vec<I> = Vec::with_capacity(intervals.len());
    for entry in intervals {
        let mut tail: Option<I> = None;
        while let Some(&last) = out.last() {
            if last.end() < entry.start() {
                break;
            }
            if last.end() > entry.end() && tail.is_none() {
                tail = Some(I::new(entry.end() + I::ONE, last.end(), last.cc()));
            }
            if last.start() < entry.start() {
                out.last_mut().unwrap().set_end(entry.start() - I::ONE);
                break;
            }
            out.pop();
        }
        out.push(entry);
        if let Some(t) = tail {
            out.push(t);
        }
    }
    out.sort_unstable_by_key(|t| t.start());
    out
}

fn build_cc_dict<I: IntoIterator<Item = [u8; 2]>>(entries: I) -> (Vec<[u8; 2]>, HashMap<[u8; 2], u8>) {
    let cc_set: BTreeSet<[u8; 2]> = entries.into_iter().collect();
    let cc_dict: Vec<[u8; 2]> = cc_set.into_iter().collect();
    assert!(cc_dict.len() <= 255, "cc dict overflow: {} codes", cc_dict.len());
    let cc_to_idx: HashMap<[u8; 2], u8> = cc_dict.iter().enumerate().map(|(i, cc)| (*cc, i as u8)).collect();
    (cc_dict, cc_to_idx)
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_u32s(out: &mut Vec<u8>, values: &[u32]) {
    for v in values {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

fn push_u16s(out: &mut Vec<u8>, values: &[u16]) {
    for v in values {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

fn push_cc_pairs(out: &mut Vec<u8>, pairs: &[[u8; 2]]) {
    for cc in pairs {
        out.extend_from_slice(cc);
    }
}

fn transform_v4(intervals: &[V4Interval]) -> Vec<u8> {
    let merged = merge_v4_adjacent(intervals);
    let (cc_dict, cc_to_idx) = build_cc_dict(merged.iter().map(|e| e.cc));
    let bucket_segments = build_v4_bucket_segments(&merged, &cc_to_idx);

    let mut first_level = vec![TAG_EMPTY; BUCKET_COUNT];
    let mut mixed_buckets: Vec<MixedBucket> = Vec::new();

    for (bucket, segments) in bucket_segments.iter().enumerate() {
        if segments.is_empty() {
            continue;
        }
        let transitions = build_v4_transitions(segments);

        if transitions.len() == 1 {
            let cc = transitions[0].1;
            first_level[bucket] = if cc == UNASSIGNED {
                TAG_EMPTY
            } else {
                TAG_PURE | (cc as u32)
            };
        } else {
            mixed_buckets.push(MixedBucket {
                bucket,
                initial_cc: transitions[0].1,
                transitions: transitions[1..].to_vec(),
            });
        }
    }

    // Dense-first ordering; see layout_mixed_buckets dense-contiguity assertion.
    mixed_buckets.sort_by_key(|m| {
        let is_sparse = m.transitions.len() <= V4_DENSE_THRESHOLD;
        (is_sparse, m.bucket)
    });

    let layout = layout_mixed_buckets(&mut first_level, &mixed_buckets);
    emit_v4(&first_level, &cc_dict, &layout)
}

fn merge_v4_adjacent(intervals: &[V4Interval]) -> Vec<V4Interval> {
    let mut merged: Vec<V4Interval> = Vec::with_capacity(intervals.len());
    for entry in intervals {
        let can_merge = matches!(
            merged.last(),
            Some(last) if last.cc == entry.cc && last.end.checked_add(1) == Some(entry.start)
        );
        if can_merge {
            merged.last_mut().unwrap().end = entry.end;
        } else {
            merged.push(*entry);
        }
    }
    merged
}

fn build_v4_bucket_segments(merged: &[V4Interval], cc_to_idx: &HashMap<[u8; 2], u8>) -> Vec<Vec<(u16, u16, u8)>> {
    let mut bucket_segments: Vec<Vec<(u16, u16, u8)>> = vec![Vec::new(); BUCKET_COUNT];
    for interval in merged {
        let first_bucket = interval.start >> 16;
        let last_bucket = interval.end >> 16;
        for bucket in first_bucket..=last_bucket {
            let bucket_start: u32 = bucket << 16;
            let bucket_end: u32 = bucket_start | 0xFFFF;
            let seg_start = interval.start.max(bucket_start);
            let seg_end = interval.end.min(bucket_end);
            bucket_segments[bucket as usize].push((
                (seg_start - bucket_start) as u16,
                (seg_end - bucket_start) as u16,
                cc_to_idx[&interval.cc],
            ));
        }
    }
    for segments in &mut bucket_segments {
        segments.sort_by_key(|s| s.0);
    }
    bucket_segments
}

fn build_v4_transitions(segments: &[(u16, u16, u8)]) -> Vec<(u16, u8)> {
    let mut transitions: Vec<(u16, u8)> = Vec::new();
    let mut position: u32 = 0;
    for &(seg_start, seg_end, cc_idx) in segments {
        if seg_start as u32 > position {
            transitions.push((position as u16, UNASSIGNED));
        }
        transitions.push((seg_start, cc_idx));
        position = seg_end as u32 + 1;
    }
    if position <= 0xFFFF {
        transitions.push((position as u16, UNASSIGNED));
    }
    if transitions[0].0 != 0 {
        transitions.insert(0, (0, UNASSIGNED));
    }
    transitions.dedup_by_key(|x| x.1);
    transitions
}

fn layout_mixed_buckets(first_level: &mut [u32], mixed_buckets: &[MixedBucket]) -> MixedLayout {
    let mut mixed_base: Vec<u32> = Vec::with_capacity(mixed_buckets.len() + 1);
    let mut mixed_initial: Vec<u8> = Vec::with_capacity(mixed_buckets.len());
    let mut transitions: Vec<(u16, u8)> = Vec::new();
    let mut deep_v4: Vec<u16> = Vec::new();
    let mut dense_count: u32 = 0;

    for (mixed_idx, bucket) in mixed_buckets.iter().enumerate() {
        first_level[bucket.bucket] = TAG_MIXED | mixed_idx as u32;
        mixed_base.push(transitions.len() as u32);
        mixed_initial.push(bucket.initial_cc);
        let range_start = transitions.len();
        transitions.extend(bucket.transitions.iter().copied());
        let range_end = transitions.len();

        if range_end - range_start > V4_DENSE_THRESHOLD {
            assert_eq!(
                mixed_idx as u32, dense_count,
                "dense buckets must be contiguous at the start"
            );
            dense_count += 1;
            deep_v4.extend_from_slice(&build_v4_deep_one(&transitions, range_start, range_end));
        }
    }
    mixed_base.push(transitions.len() as u32);

    MixedLayout {
        mixed_base,
        mixed_initial,
        transitions,
        deep_v4,
        dense_count,
    }
}

fn build_v4_deep_one(transitions: &[(u16, u8)], range_start: usize, range_end: usize) -> [u16; V4_DEEP_TOP4_LEN] {
    let mut deep = [0u16; V4_DEEP_TOP4_LEN];
    let mut cursor = range_start;
    for top_4 in 0u32..16 {
        let top_4_start = (top_4 << 12) as u16;
        while cursor < range_end && transitions[cursor].0 < top_4_start {
            cursor += 1;
        }
        deep[top_4 as usize] = (cursor - range_start) as u16;
    }
    deep[16] = (range_end - range_start) as u16;
    deep
}

fn emit_v4(first_level: &[u32], cc_dict: &[[u8; 2]], layout: &MixedLayout) -> Vec<u8> {
    let mut out = Vec::new();
    push_u32s(&mut out, first_level);
    push_u32(&mut out, cc_dict.len() as u32);
    push_cc_pairs(&mut out, cc_dict);
    push_u32(&mut out, layout.mixed_initial.len() as u32);
    push_u32s(&mut out, &layout.mixed_base);
    out.extend_from_slice(&layout.mixed_initial);
    push_u32(&mut out, layout.transitions.len() as u32);
    for (offset, _) in &layout.transitions {
        out.extend_from_slice(&offset.to_le_bytes());
    }
    for (_, cc) in &layout.transitions {
        out.push(*cc);
    }
    push_u32(&mut out, layout.dense_count);
    push_u16s(&mut out, &layout.deep_v4);
    out
}

fn transform_v6(intervals: &[V6Interval]) -> Vec<u8> {
    let split = split_v6_at_byte_24_boundary(intervals);
    let merged = merge_v6_adjacent_within_24(&split);
    let by_bucket = group_v6_by_bucket(&merged);

    let populated: Vec<u128> = by_bucket.keys().copied().collect();
    assert!(
        populated.len() < 256,
        "v6 populated bucket count {} would collide with UNASSIGNED (0xFF)",
        populated.len()
    );

    let bucket_lookup = build_v6_bucket_lookup(&populated);
    let populated_first = build_v6_populated_first(&populated, &by_bucket);
    let deep = build_v6_deep_layout(&populated, &by_bucket, &merged);

    let (cc_dict, cc_to_idx) = build_cc_dict(merged.iter().map(|e| e.cc));
    let (primary, side) = build_v6_records(&merged, &cc_to_idx);

    emit_v6(
        populated.len(),
        &bucket_lookup,
        &populated_first,
        &deep,
        &cc_dict,
        &side,
        &primary,
    )
}

fn split_v6_at_byte_24_boundary(intervals: &[V6Interval]) -> Vec<V6Interval> {
    let mut split: Vec<V6Interval> = Vec::with_capacity(intervals.len());
    for entry in intervals {
        let mut s = entry.start;
        while (s >> 104) != (entry.end >> 104) {
            let next_24 = ((s >> 104) + 1) << 104;
            split.push(V6Interval {
                start: s,
                end: next_24 - 1,
                cc: entry.cc,
            });
            s = next_24;
        }
        split.push(V6Interval {
            start: s,
            end: entry.end,
            cc: entry.cc,
        });
    }
    split
}

fn merge_v6_adjacent_within_24(split: &[V6Interval]) -> Vec<V6Interval> {
    let mut merged: Vec<V6Interval> = Vec::with_capacity(split.len());
    for entry in split {
        let can_merge = matches!(
            merged.last(),
            Some(last) if last.cc == entry.cc
                && last.end.checked_add(1) == Some(entry.start)
                && (last.start >> 104) == (entry.end >> 104)
        );
        if can_merge {
            merged.last_mut().unwrap().end = entry.end;
        } else {
            merged.push(*entry);
        }
    }
    merged
}

fn group_v6_by_bucket(merged: &[V6Interval]) -> BTreeMap<u128, Vec<usize>> {
    let mut by_bucket: BTreeMap<u128, Vec<usize>> = BTreeMap::new();
    for (i, entry) in merged.iter().enumerate() {
        by_bucket.entry(entry.start >> 112).or_default().push(i);
    }
    by_bucket
}

fn build_v6_bucket_lookup(populated: &[u128]) -> Vec<u8> {
    let mut bucket_lookup = vec![UNASSIGNED; BUCKET_COUNT];
    for (populated_idx, bucket_prefix) in populated.iter().enumerate() {
        bucket_lookup[*bucket_prefix as usize] = populated_idx as u8;
    }
    bucket_lookup
}

fn build_v6_populated_first(populated: &[u128], by_bucket: &BTreeMap<u128, Vec<usize>>) -> Vec<u32> {
    let mut populated_first: Vec<u32> = Vec::with_capacity(populated.len() + 1);
    let mut running: u32 = 0;
    for bucket_prefix in populated {
        populated_first.push(running);
        running += by_bucket[bucket_prefix].len() as u32;
    }
    populated_first.push(running);
    populated_first
}

fn build_v6_deep_layout(
    populated: &[u128],
    by_bucket: &BTreeMap<u128, Vec<usize>>,
    merged: &[V6Interval],
) -> V6DeepLayout {
    let mut sub_index: Vec<u16> = Vec::with_capacity(populated.len() * V6_BYTE_INDEX_LEN);
    let mut dense_keys: Vec<u16> = Vec::new();
    let mut deep_pairs: Vec<u16> = Vec::new();
    let mut dense2_keys: Vec<u16> = Vec::new();
    let mut deep2_pairs: Vec<u16> = Vec::new();

    for (populated_idx, bucket_prefix_16) in populated.iter().enumerate() {
        let entries = &by_bucket[bucket_prefix_16];
        assert!(entries.len() <= u16::MAX as usize, "bucket size exceeds u16");

        let local_offsets = build_v6_sub_index(entries, merged);
        sub_index.extend_from_slice(&local_offsets);

        for byte_24 in 0u16..256 {
            let sub_start = local_offsets[byte_24 as usize] as usize;
            let sub_end = local_offsets[(byte_24 + 1) as usize] as usize;
            if sub_end - sub_start <= V6_DENSE_THRESHOLD {
                continue;
            }
            let dense_idx = dense_keys.len();
            assert!(populated_idx < 256);
            dense_keys.push(((populated_idx as u16) << 8) | byte_24);

            let prefix_24 = ((*bucket_prefix_16 << 8) | (byte_24 as u128)) << 104;
            let deep = build_v6_deep_pairs(entries, merged, sub_start, sub_end, prefix_24, 96);
            deep_pairs.extend_from_slice(&deep);

            for byte_32 in 0u32..256 {
                let inner_start = deep[byte_32 as usize * 2] as usize;
                let inner_end = deep[byte_32 as usize * 2 + 1] as usize;
                if inner_end - inner_start <= V6_DENSE_THRESHOLD {
                    continue;
                }
                assert!(dense_idx < 256);
                dense2_keys.push(((dense_idx as u16) << 8) | byte_32 as u16);

                let prefix_32 = prefix_24 | ((byte_32 as u128) << 96);
                let deep2 = build_v6_deep_pairs(
                    entries,
                    merged,
                    sub_start + inner_start,
                    sub_start + inner_end,
                    prefix_32,
                    88,
                );
                deep2_pairs.extend_from_slice(&deep2);
            }
        }
    }

    assert!(
        dense_keys.windows(2).all(|w| w[0] < w[1]),
        "dense_keys must be strictly sorted"
    );
    assert!(
        dense2_keys.windows(2).all(|w| w[0] < w[1]),
        "dense2_keys must be strictly sorted"
    );

    V6DeepLayout {
        sub_index,
        dense_keys,
        deep_pairs,
        dense2_keys,
        deep2_pairs,
    }
}

fn build_v6_sub_index(entries: &[usize], merged: &[V6Interval]) -> [u16; V6_BYTE_INDEX_LEN] {
    let mut local_offsets = [0u16; V6_BYTE_INDEX_LEN];
    let mut cursor: usize = 0;
    for byte_24 in 0u16..256 {
        local_offsets[byte_24 as usize] = cursor as u16;
        while cursor < entries.len() {
            let entry_byte_24 = ((merged[entries[cursor]].start >> 104) & 0xFF) as u16;
            if entry_byte_24 > byte_24 {
                break;
            }
            cursor += 1;
        }
    }
    local_offsets[256] = entries.len() as u16;
    local_offsets
}

fn build_v6_deep_pairs(
    entries: &[usize],
    merged: &[V6Interval],
    range_start: usize,
    range_end: usize,
    prefix: u128,
    byte_shift: u32,
) -> [u16; V6_BYTE_INDEX_LEN * 2] {
    let mut deep = [0u16; V6_BYTE_INDEX_LEN * 2];
    let mut lo_cursor = range_start;
    let mut hi_cursor = range_start;
    for byte in 0u32..256 {
        let byte_lo = prefix | ((byte as u128) << byte_shift);
        let byte_hi = byte_lo | ((1u128 << byte_shift) - 1);
        while lo_cursor < range_end && merged[entries[lo_cursor]].end < byte_lo {
            lo_cursor += 1;
        }
        while hi_cursor < range_end && merged[entries[hi_cursor]].start <= byte_hi {
            hi_cursor += 1;
        }
        deep[byte as usize * 2] = (lo_cursor - range_start) as u16;
        deep[byte as usize * 2 + 1] = (hi_cursor - range_start) as u16;
    }
    let span = (range_end - range_start) as u16;
    deep[256 * 2] = span;
    deep[256 * 2 + 1] = span;
    deep
}

fn build_v6_records(merged: &[V6Interval], cc_to_idx: &HashMap<[u8; 2], u8>) -> (Vec<u8>, Vec<u8>) {
    let mut primary: Vec<u8> = Vec::with_capacity(merged.len() * V6_REC_SIZE);
    let mut side: Vec<u8> = Vec::new();

    for entry in merged {
        let cc_idx = cc_to_idx[&entry.cc];
        let size = entry.end - entry.start + 1;
        let is_clean = size.is_power_of_two() && (entry.start & (size - 1)) == 0;
        let prefix_len = if is_clean { 128 - size.trailing_zeros() } else { 0 };
        let fits_primary = (24..=48).contains(&prefix_len) && aligned_to_primary(entry.start);

        if fits_primary {
            let offset = (entry.start >> 80) as u32;
            primary.extend_from_slice(&offset.to_le_bytes());
            primary.push(prefix_len as u8);
            primary.push(cc_idx);
        } else {
            let side_idx = (side.len() / V6_SIDE_SIZE) as u32;
            side.extend_from_slice(&entry.start.to_le_bytes());
            side.extend_from_slice(&entry.end.to_le_bytes());
            side.push(cc_idx);
            primary.extend_from_slice(&side_idx.to_le_bytes());
            primary.push(V6_IRREGULAR);
            primary.push(0);
        }
    }
    (primary, side)
}

fn aligned_to_primary(start: u128) -> bool {
    (start & ((1u128 << 80) - 1)) == 0
}

fn emit_v6(
    populated_count: usize,
    bucket_lookup: &[u8],
    populated_first: &[u32],
    deep: &V6DeepLayout,
    cc_dict: &[[u8; 2]],
    side: &[u8],
    primary: &[u8],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(bucket_lookup);
    push_u32(&mut out, populated_count as u32);
    push_u32s(&mut out, populated_first);
    push_u16s(&mut out, &deep.sub_index);
    push_u32(&mut out, deep.dense_keys.len() as u32);
    push_u16s(&mut out, &deep.dense_keys);
    push_u16s(&mut out, &deep.deep_pairs);
    push_u32(&mut out, deep.dense2_keys.len() as u32);
    push_u16s(&mut out, &deep.dense2_keys);
    push_u16s(&mut out, &deep.deep2_pairs);
    push_u32(&mut out, cc_dict.len() as u32);
    push_cc_pairs(&mut out, cc_dict);
    push_u32(&mut out, (side.len() / V6_SIDE_SIZE) as u32);
    out.extend_from_slice(side);
    push_u32(&mut out, (primary.len() / V6_REC_SIZE) as u32);
    out.extend_from_slice(primary);
    out
}
