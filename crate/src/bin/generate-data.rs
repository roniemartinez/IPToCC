use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Write;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::str::FromStr;

const RIRS: &[&str] = &["afrinic", "apnic", "arin", "lacnic", "ripencc"];
const FIRST_LEVEL_COUNT: usize = 65537;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir).parent().expect("workspace root");
    let data_dir = workspace_root.join("data");
    let out_dir = Path::new(manifest_dir).join("src/data");
    fs::create_dir_all(&out_dir).expect("creating src/data");

    let mut v4: Vec<(u32, u32, [u8; 2])> = Vec::new();
    let mut v6: Vec<(u128, u128, [u8; 2])> = Vec::new();

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
            if ty != "ipv4" && ty != "ipv6" {
                continue;
            }
            if status != "allocated" && status != "assigned" {
                continue;
            }
            let cc_bytes = [cc.as_bytes()[0], cc.as_bytes()[1]];

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
                    v4.push((s, e, cc_bytes));
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
                    v6.push((s, e, cc_bytes));
                }
                _ => unreachable!(),
            }
        }
    }

    v4.sort_unstable_by_key(|t| t.0);
    v6.sort_unstable_by_key(|t| t.0);

    let mut first_level: Vec<u32> = Vec::with_capacity(FIRST_LEVEL_COUNT);
    let mut pos: usize = 0;
    for bucket in 0..FIRST_LEVEL_COUNT {
        let bucket_floor: u64 = (bucket as u64) << 16;
        while pos < v4.len() && (v4[pos].0 as u64) < bucket_floor {
            pos += 1;
        }
        first_level.push(pos as u32);
    }

    let v4_path = out_dir.join("v4.bin");
    let mut w = fs::File::create(&v4_path).expect("creating v4.bin");
    for &fl in &first_level {
        w.write_all(&fl.to_le_bytes()).unwrap();
    }
    for &(s, _, _) in &v4 {
        w.write_all(&s.to_le_bytes()).unwrap();
    }
    for &(_, e, _) in &v4 {
        w.write_all(&e.to_le_bytes()).unwrap();
    }
    for &(_, _, cc) in &v4 {
        w.write_all(&cc).unwrap();
    }

    let v6_path = out_dir.join("v6.bin");
    let mut w = fs::File::create(&v6_path).expect("creating v6.bin");
    for &(s, _, _) in &v6 {
        w.write_all(&s.to_le_bytes()).unwrap();
    }
    for &(_, e, _) in &v6 {
        w.write_all(&e.to_le_bytes()).unwrap();
    }
    for &(_, _, cc) in &v6 {
        w.write_all(&cc).unwrap();
    }

    let unique_codes: HashSet<[u8; 2]> = v4.iter().map(|t| t.2).chain(v6.iter().map(|t| t.2)).collect();
    let v4_size = fs::metadata(&v4_path).unwrap().len();
    let v6_size = fs::metadata(&v6_path).unwrap().len();
    println!(
        "wrote {} ({} bytes), {} ({} bytes); {} v4, {} v6, {} countries",
        v4_path.display(),
        v4_size,
        v6_path.display(),
        v6_size,
        v4.len(),
        v6.len(),
        unique_codes.len()
    );
}
