use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::str::FromStr;

use iptocc::format::{FIRST_LEVEL_COUNT, V4_GAP_SENTINEL, V6_BUCKET_COUNT, V6_BUCKET_EMPTY, V6_SUB_INDEX_LEN};

const RIRS: &[&str] = &["afrinic", "apnic", "arin", "lacnic", "ripencc"];

struct V4Interval {
    start: u32,
    end: u32,
    cc: [u8; 2],
}

struct V6Interval {
    start: u128,
    end: u128,
    cc: [u8; 2],
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir).parent().expect("workspace root");
    let data_dir = workspace_root.join("data");
    let out_dir = Path::new(manifest_dir).join("src/data");
    fs::create_dir_all(&out_dir).expect("creating src/data");

    let (mut v4, mut v6) = parse_rir(&data_dir);

    v4.sort_unstable_by_key(|t| t.start);
    v6.sort_unstable_by_key(|t| t.start);

    assert!(v4.windows(2).all(|w| w[0].end < w[1].start), "v4 intervals overlap");
    assert!(v6.windows(2).all(|w| w[0].end < w[1].start), "v6 intervals overlap");

    let v6_buckets: BTreeSet<u128> = v6.iter().map(|e| e.start >> 112).collect();
    assert!(
        v6_buckets.len() < 255,
        "v6 populated bucket count {} exceeds u8 range",
        v6_buckets.len()
    );

    for entry in &v6 {
        assert_eq!(
            entry.start >> 112,
            entry.end >> 112,
            "v6 interval spans /16 buckets: {:x}..{:x}",
            entry.start,
            entry.end
        );
    }

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

    let mut entries: Vec<(u32, [u8; 2])> = Vec::new();
    if merged.first().is_some_and(|e| e.start > 0) {
        entries.push((0, V4_GAP_SENTINEL));
    }
    for i in 0..merged.len() {
        let e = &merged[i];
        entries.push((e.start, e.cc));
        if let Some(next) = merged.get(i + 1) {
            if next.start > e.end + 1 {
                entries.push((e.end + 1, V4_GAP_SENTINEL));
            }
        }
    }
    if let Some(last) = merged.last() {
        if last.end < u32::MAX {
            entries.push((last.end + 1, V4_GAP_SENTINEL));
        }
    }

    let n = entries.len();
    let mut first_level: Vec<u32> = Vec::with_capacity(FIRST_LEVEL_COUNT);
    let mut pos: usize = 0;
    for bucket in 0..FIRST_LEVEL_COUNT {
        let bucket_floor: u64 = (bucket as u64) << 16;
        while pos < n && (entries[pos].0 as u64) < bucket_floor {
            pos += 1;
        }
        first_level.push(pos as u32);
    }
    debug_assert_eq!(first_level[FIRST_LEVEL_COUNT - 1] as usize, n);

    let mut out = Vec::with_capacity(FIRST_LEVEL_COUNT * 4 + n * 6);
    for fl in &first_level {
        out.extend_from_slice(&fl.to_le_bytes());
    }
    for (s, _) in &entries {
        out.extend_from_slice(&s.to_le_bytes());
    }
    for (_, cc) in &entries {
        out.extend_from_slice(cc);
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
        populated.len() < 255,
        "v6 populated bucket count {} exceeds u8 range",
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

    let n = merged.len();
    let header = V6_BUCKET_COUNT + 4 + (populated.len() + 1) * 4 + sub_index.len() * 2;
    let body = n * 34;
    let mut out = Vec::with_capacity(header + body);
    out.extend_from_slice(&bucket_lookup);
    out.extend_from_slice(&(populated.len() as u32).to_le_bytes());
    for fi in &populated_first {
        out.extend_from_slice(&fi.to_le_bytes());
    }
    for si in &sub_index {
        out.extend_from_slice(&si.to_le_bytes());
    }
    for e in &merged {
        out.extend_from_slice(&e.start.to_le_bytes());
    }
    for e in &merged {
        out.extend_from_slice(&e.end.to_le_bytes());
    }
    for e in &merged {
        out.extend_from_slice(&e.cc);
    }
    out
}
