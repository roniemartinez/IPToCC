use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::{Add, Sub};
use std::path::Path;
use std::str::FromStr;

use iptocc::format::{
    TAG_EMPTY, TAG_MIXED, TAG_PURE, V4_CC_GAP, V6_BUCKET_COUNT, V6_BUCKET_EMPTY, V6_IRREGULAR, V6_REC_SIZE,
    V6_SIDE_SIZE, V6_SUB_INDEX_LEN,
};

const RIRS: &[&str] = &["afrinic", "apnic", "arin", "lacnic", "ripencc"];

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

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir).parent().expect("workspace root");
    let data_dir = workspace_root.join("data");
    let out_dir = Path::new(manifest_dir).join("src/data");
    fs::create_dir_all(&out_dir).expect("creating src/data");

    let (v4, v6) = parse_rir(&data_dir);
    let mut v4 = resolve_overlaps(v4);
    let mut v6 = resolve_overlaps(v6);

    v4.sort_unstable_by_key(|t| t.start);
    v6.sort_unstable_by_key(|t| t.start);

    assert!(
        v4.windows(2).all(|w| w[0].end < w[1].start),
        "v4 intervals still overlap after resolution"
    );
    assert!(
        v6.windows(2).all(|w| w[0].end < w[1].start),
        "v6 intervals still overlap after resolution"
    );

    for entry in v4.iter().map(|e| e.cc).chain(v6.iter().map(|e| e.cc)) {
        assert!(
            entry.iter().all(|b| b.is_ascii_uppercase()),
            "non-ASCII-uppercase code: {entry:?}"
        );
    }

    let v4_bin = transform_v4(&v4);
    let v4_path = out_dir.join("v4.bin");
    fs::write(&v4_path, &v4_bin).expect("writing v4.bin");

    let v6_bin = transform_v6(&v6);
    let v6_path = out_dir.join("v6.bin");
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

fn parse_rir(data_dir: &Path) -> (Vec<V4Interval>, Vec<V6Interval>) {
    let mut v4: Vec<V4Interval> = Vec::new();
    let mut v6: Vec<V6Interval> = Vec::new();

    for rir in RIRS {
        let path = data_dir.join(format!("delegated-{rir}-extended-latest"));
        let content = fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
        for line in content.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 7 {
                continue;
            }
            let cc = parts[1];
            let ty = parts[2];
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

            match ty {
                "ipv4" => {
                    let Ok(start_ip) = Ipv4Addr::from_str(start) else {
                        continue;
                    };
                    let Ok(count) = value.parse::<u32>() else { continue };
                    if count == 0 {
                        continue;
                    }
                    let s = u32::from(start_ip);
                    let e = s.saturating_add(count - 1);
                    v4.push(V4Interval {
                        start: s,
                        end: e,
                        cc: cc_bytes,
                    });
                }
                "ipv6" => {
                    let Ok(start_ip) = Ipv6Addr::from_str(start) else {
                        continue;
                    };
                    let Ok(prefix) = value.parse::<u32>() else { continue };
                    if prefix == 0 || prefix > 128 {
                        continue;
                    }
                    let s = u128::from(start_ip);
                    let host_bits = 128 - prefix;
                    let mask = if host_bits == 128 {
                        u128::MAX
                    } else {
                        (1u128 << host_bits) - 1
                    };
                    let e = s | mask;
                    v6.push(V6Interval {
                        start: s,
                        end: e,
                        cc: cc_bytes,
                    });
                }
                _ => {}
            }
        }
    }
    (v4, v6)
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
    out
}

fn transform_v4(intervals: &[V4Interval]) -> Vec<u8> {
    let mut merged: Vec<V4Interval> = Vec::with_capacity(intervals.len());
    for entry in intervals {
        let merge = match merged.last() {
            Some(last) => last.cc == entry.cc && last.end.checked_add(1) == Some(entry.start),
            None => false,
        };
        if merge {
            merged.last_mut().unwrap().end = entry.end;
        } else {
            merged.push(V4Interval {
                start: entry.start,
                end: entry.end,
                cc: entry.cc,
            });
        }
    }

    let mut cc_set: BTreeSet<[u8; 2]> = BTreeSet::new();
    for e in &merged {
        cc_set.insert(e.cc);
    }
    let cc_dict: Vec<[u8; 2]> = cc_set.into_iter().collect();
    assert!(cc_dict.len() <= 255, "v4 cc dict overflow: {} codes", cc_dict.len());
    let cc_to_idx: std::collections::HashMap<[u8; 2], u8> =
        cc_dict.iter().enumerate().map(|(i, cc)| (*cc, i as u8)).collect();

    const BUCKETS: usize = 65536;
    let mut bucket_segs: Vec<Vec<(u16, u16, u8)>> = vec![Vec::new(); BUCKETS];
    for iv in &merged {
        let first_bucket = iv.start >> 16;
        let last_bucket = iv.end >> 16;
        for b in first_bucket..=last_bucket {
            let bucket_start: u32 = b << 16;
            let bucket_end: u32 = bucket_start | 0xFFFF;
            let seg_start = iv.start.max(bucket_start);
            let seg_end = iv.end.min(bucket_end);
            bucket_segs[b as usize].push((
                (seg_start - bucket_start) as u16,
                (seg_end - bucket_start) as u16,
                cc_to_idx[&iv.cc],
            ));
        }
    }
    for segs in &mut bucket_segs {
        segs.sort_by_key(|s| s.0);
    }

    let mut first_level: Vec<u32> = vec![TAG_EMPTY; BUCKETS];
    let mut mixed_base: Vec<u32> = Vec::new();
    let mut mixed_initial: Vec<u8> = Vec::new();
    let mut transitions: Vec<(u16, u8)> = Vec::new();

    for (bucket, segs) in bucket_segs.iter().enumerate() {
        if segs.is_empty() {
            continue;
        }

        let mut trans: Vec<(u16, u8)> = Vec::new();
        let mut pos: u32 = 0;
        for &(off_start, off_end, cc_idx) in segs {
            if (off_start as u32) > pos {
                trans.push((pos as u16, V4_CC_GAP));
            }
            trans.push((off_start, cc_idx));
            pos = (off_end as u32) + 1;
        }
        if pos <= 65535 {
            trans.push((pos as u16, V4_CC_GAP));
        }
        if trans[0].0 != 0 {
            trans.insert(0, (0, V4_CC_GAP));
        }
        trans.dedup_by_key(|x| x.1);

        if trans.len() == 1 {
            let (_, cc) = trans[0];
            first_level[bucket] = if cc == V4_CC_GAP {
                TAG_EMPTY
            } else {
                TAG_PURE | (cc as u32)
            };
        } else {
            let mixed_idx = mixed_base.len() as u32;
            first_level[bucket] = TAG_MIXED | mixed_idx;
            mixed_base.push(transitions.len() as u32);
            mixed_initial.push(trans[0].1);
            for &(off, cc) in &trans[1..] {
                transitions.push((off, cc));
            }
        }
    }
    mixed_base.push(transitions.len() as u32);

    let mut out = Vec::new();
    for fl in &first_level {
        out.extend_from_slice(&fl.to_le_bytes());
    }
    out.extend_from_slice(&(cc_dict.len() as u32).to_le_bytes());
    for cc in &cc_dict {
        out.extend_from_slice(cc);
    }
    out.extend_from_slice(&(mixed_initial.len() as u32).to_le_bytes());
    for mb in &mixed_base {
        out.extend_from_slice(&mb.to_le_bytes());
    }
    out.extend_from_slice(&mixed_initial);
    out.extend_from_slice(&(transitions.len() as u32).to_le_bytes());
    for &(off, _) in &transitions {
        out.extend_from_slice(&off.to_le_bytes());
    }
    for &(_, cc) in &transitions {
        out.push(cc);
    }
    out
}

fn transform_v6(intervals: &[V6Interval]) -> Vec<u8> {
    let mut split: Vec<V6Interval> = Vec::with_capacity(intervals.len());
    for entry in intervals {
        let mut s = entry.start;
        while (s >> 104) != (entry.end >> 104) {
            let sub_end = ((s >> 104) + 1) << 104;
            split.push(V6Interval {
                start: s,
                end: sub_end - 1,
                cc: entry.cc,
            });
            s = sub_end;
        }
        split.push(V6Interval {
            start: s,
            end: entry.end,
            cc: entry.cc,
        });
    }

    let mut merged: Vec<V6Interval> = Vec::with_capacity(split.len());
    for entry in &split {
        let merge = match merged.last() {
            Some(last) => {
                last.cc == entry.cc
                    && last.end.checked_add(1) == Some(entry.start)
                    && (last.start >> 104) == (entry.end >> 104)
            }
            None => false,
        };
        if merge {
            merged.last_mut().unwrap().end = entry.end;
        } else {
            merged.push(V6Interval {
                start: entry.start,
                end: entry.end,
                cc: entry.cc,
            });
        }
    }

    let mut by_bucket: BTreeMap<u128, Vec<usize>> = BTreeMap::new();
    for (i, e) in merged.iter().enumerate() {
        by_bucket.entry(e.start >> 112).or_default().push(i);
    }

    let populated: Vec<u128> = by_bucket.keys().copied().collect();
    assert!(
        populated.len() < 256,
        "v6 populated bucket count {} would collide with V6_BUCKET_EMPTY (0xFF)",
        populated.len()
    );

    let mut bucket_lookup = vec![V6_BUCKET_EMPTY; V6_BUCKET_COUNT];
    for (ord_i, b) in populated.iter().enumerate() {
        bucket_lookup[*b as usize] = ord_i as u8;
    }

    let mut populated_first: Vec<u32> = Vec::with_capacity(populated.len() + 1);
    let mut running: u32 = 0;
    for b in &populated {
        populated_first.push(running);
        running += by_bucket[b].len() as u32;
    }
    populated_first.push(running);

    let mut sub_index: Vec<u16> = Vec::with_capacity(populated.len() * V6_SUB_INDEX_LEN);
    for b in &populated {
        let entries_in_bucket = &by_bucket[b];
        let bucket_size = entries_in_bucket.len();
        assert!(bucket_size <= u16::MAX as usize, "bucket size exceeds u16");
        let mut local_offsets = [0u16; V6_SUB_INDEX_LEN];
        let mut local_pos: usize = 0;
        for sub in 0..256u16 {
            local_offsets[sub as usize] = local_pos as u16;
            while local_pos < bucket_size {
                let global_idx = entries_in_bucket[local_pos];
                let entry_sub = ((merged[global_idx].start >> 104) & 0xFF) as u16;
                if entry_sub > sub {
                    break;
                }
                local_pos += 1;
            }
        }
        local_offsets[256] = bucket_size as u16;
        sub_index.extend_from_slice(&local_offsets);
    }

    let mut cc_set: BTreeSet<[u8; 2]> = BTreeSet::new();
    for e in &merged {
        cc_set.insert(e.cc);
    }
    let cc_dict: Vec<[u8; 2]> = cc_set.into_iter().collect();
    assert!(cc_dict.len() <= 255, "v6 cc dict overflow: {} codes", cc_dict.len());
    let cc_to_idx: std::collections::HashMap<[u8; 2], u8> =
        cc_dict.iter().enumerate().map(|(i, cc)| (*cc, i as u8)).collect();

    let mut primary: Vec<u8> = Vec::with_capacity(merged.len() * V6_REC_SIZE);
    let mut side: Vec<u8> = Vec::new();

    for e in &merged {
        let cc_idx = cc_to_idx[&e.cc];
        let size = e.end - e.start + 1;
        let is_clean = size.is_power_of_two() && (e.start & (size - 1)) == 0;
        let prefix_len = if is_clean { 128 - size.trailing_zeros() } else { 0 };
        let aligned_to_48 = (e.start & ((1u128 << 80) - 1)) == 0;

        if (24..=48).contains(&prefix_len) && aligned_to_48 {
            let offset = ((e.start >> 80) & ((1u128 << 32) - 1)) as u32;
            primary.extend_from_slice(&offset.to_le_bytes());
            primary.push(prefix_len as u8);
            primary.push(cc_idx);
        } else {
            let side_idx = (side.len() / V6_SIDE_SIZE) as u32;
            side.extend_from_slice(&e.start.to_le_bytes());
            side.extend_from_slice(&e.end.to_le_bytes());
            side.push(cc_idx);
            primary.extend_from_slice(&side_idx.to_le_bytes());
            primary.push(V6_IRREGULAR);
            primary.push(0);
        }
    }

    let mut out = Vec::new();
    out.extend_from_slice(&bucket_lookup);
    out.extend_from_slice(&(populated.len() as u32).to_le_bytes());
    for fi in &populated_first {
        out.extend_from_slice(&fi.to_le_bytes());
    }
    for si in &sub_index {
        out.extend_from_slice(&si.to_le_bytes());
    }
    out.extend_from_slice(&(cc_dict.len() as u32).to_le_bytes());
    for cc in &cc_dict {
        out.extend_from_slice(cc);
    }
    out.extend_from_slice(&((side.len() / V6_SIDE_SIZE) as u32).to_le_bytes());
    out.extend_from_slice(&side);
    out.extend_from_slice(&((primary.len() / V6_REC_SIZE) as u32).to_le_bytes());
    out.extend_from_slice(&primary);
    out
}
