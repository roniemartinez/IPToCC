//! Single-pass IP address parsing.
//!
//! `core::net`'s `FromStr` is a backtracking parser shared by socket-address and
//! CIDR syntax; addresses arrive here already isolated, so a straight-line scan
//! does the same job in a fraction of the work. Accepts exactly the same set of
//! strings as `IpAddr::from_str` — including its rejection of leading zeros in
//! IPv4 octets — which `parity_with_std` in the test module pins down.

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[inline]
pub fn parse_ip(s: &str) -> Option<IpAddr> {
    let b = s.as_bytes();
    // The grammars are disjoint, so trying IPv4 first costs nothing to reject: a
    // v6 literal fails within the first few bytes (on ':' or a fifth hex digit).
    // Scanning ahead for a ':' to pick the branch measured slower — the scan is a
    // whole extra pass over the common, short, IPv4 case.
    if let Some(bits) = parse_v4(b) {
        return Some(IpAddr::V4(Ipv4Addr::from_bits(bits)));
    }
    parse_v6(b).map(|bits| IpAddr::V6(Ipv6Addr::from_bits(bits)))
}

/// Parses a dotted-quad into host-order bits. Rejects leading zeros, matching std.
#[inline]
fn parse_v4(b: &[u8]) -> Option<u32> {
    let mut result = 0u32;
    let mut i = 0usize;

    for octet in 0..4 {
        if octet > 0 {
            if b.get(i) != Some(&b'.') {
                return None;
            }
            i += 1;
        }

        let start = i;
        let mut value = 0u32;
        while i < b.len() && b[i].is_ascii_digit() {
            // Bail on the fourth digit rather than after the loop: an unbounded
            // run of digits would otherwise overflow `value`.
            if i - start == 3 {
                return None;
            }
            value = value * 10 + (b[i] - b'0') as u32;
            i += 1;
        }

        let len = i - start;
        if len == 0 || value > 255 {
            return None;
        }
        if len > 1 && b[start] == b'0' {
            return None;
        }
        result = (result << 8) | value;
    }

    (i == b.len()).then_some(result)
}

/// Parses an IPv6 literal, including `::` compression and an embedded IPv4 tail.
fn parse_v6(b: &[u8]) -> Option<u128> {
    let mut groups = [0u16; 8];
    let mut filled = 0usize;
    let mut ellipsis: Option<usize> = None;
    let mut i = 0usize;

    // A leading ':' is only legal as the first half of "::".
    if b.first() == Some(&b':') {
        if b.get(1) != Some(&b':') {
            return None;
        }
        ellipsis = Some(0);
        i = 2;
        if i == b.len() {
            return Some(0); // "::"
        }
    }

    loop {
        if filled == 8 {
            return None;
        }

        let start = i;
        let mut value = 0u32;
        while i < b.len() {
            let digit = match b[i] {
                c @ b'0'..=b'9' => c - b'0',
                c @ b'a'..=b'f' => c - b'a' + 10,
                c @ b'A'..=b'F' => c - b'A' + 10,
                _ => break,
            };
            value = (value << 4) | digit as u32;
            i += 1;
            if i - start > 4 {
                return None;
            }
        }
        if i == start {
            return None;
        }

        // A '.' means this run of digits was really the start of an IPv4 tail,
        // which occupies the last two groups and ends the address.
        if b.get(i) == Some(&b'.') {
            if filled > 6 {
                return None;
            }
            let v4 = parse_v4(&b[start..])?;
            groups[filled] = (v4 >> 16) as u16;
            groups[filled + 1] = v4 as u16;
            filled += 2;
            break; // parse_v4 already required the tail to reach the end
        }

        groups[filled] = value as u16;
        filled += 1;

        if i == b.len() {
            break;
        }
        if b[i] != b':' {
            return None;
        }
        i += 1;

        if b.get(i) == Some(&b':') {
            if ellipsis.is_some() {
                return None;
            }
            ellipsis = Some(filled);
            i += 1;
            if i == b.len() {
                break; // trailing "::"
            }
        } else if i == b.len() {
            return None; // dangling ':'
        }
    }

    match ellipsis {
        // Without compression every group must be spelled out.
        None => {
            if filled != 8 {
                return None;
            }
        }
        // "::" has to stand for at least one group, so a full set is a conflict.
        Some(at) => {
            if filled == 8 {
                return None;
            }
            let tail = filled - at;
            let dst = 8 - tail;
            for k in (0..tail).rev() {
                groups[dst + k] = groups[at + k];
            }
            for slot in &mut groups[at..dst] {
                *slot = 0;
            }
        }
    }

    let mut bits = 0u128;
    for group in groups {
        bits = (bits << 16) | group as u128;
    }
    Some(bits)
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;

    /// Every string below must parse (or fail) identically to `IpAddr::from_str`.
    /// This is the whole safety net for replacing std's parser.
    #[test]
    fn parity_with_std() {
        let mut cases: Vec<String> = Vec::new();

        for s in [
            // v4: valid, boundary, and malformed
            "0.0.0.0",
            "255.255.255.255",
            "8.8.8.8",
            "1.0.16.1",
            "10.0.0.0",
            "256.0.0.1",
            "1.2.3",
            "1.2.3.4.5",
            "1.2.3.",
            ".1.2.3",
            "01.2.3.4",
            "1.02.3.4",
            "0.0.0.00",
            "1.2.3.4 ",
            " 1.2.3.4",
            "1.2.3.-4",
            "1..2.3",
            "1.2.3.4a",
            "",
            "not-an-ip",
            // v6: compression in every position
            "::",
            "::1",
            "1::",
            "::ffff",
            "1:2:3:4:5:6:7:8",
            "1:2:3:4:5:6:7::",
            "1::8",
            "1:2::8",
            "1:2:3:4:5:6::8",
            "2001:4860:4860::8888",
            "2001:67c:18::1",
            "2001:db8:0:0:0:0:0:1",
            "0000:0000:0000:0000:0000:0000:0000:0001",
            "fe80::1%eth0",
            // v6: embedded v4
            "::ffff:192.168.0.1",
            "::192.168.0.1",
            "64:ff9b::1.2.3.4",
            "1:2:3:4:5:6:1.2.3.4",
            "1:2:3:4:5:6:7:1.2.3.4",
            "::ffff:1.2.3.4:5",
            "::ffff:256.1.1.1",
            "::ffff:01.2.3.4",
            // v6: malformed
            "1:2:3:4:5:6:7:8:9",
            "1:2:3:4:5:6:7",
            "1::2::3",
            "1:2:3:4::5:6:7:8",
            ":1:2:3:4:5:6:7",
            "1:2:3:4:5:6:7:",
            ":::",
            "12345::",
            "1:2:3:4:5:6:7:8::",
            "::1:2:3:4:5:6:7:8",
            "g::1",
            "1:2:3:4:5:6:7:8 ",
            "1:",
            ":",
            "::.",
            "1::2:",
        ] {
            cases.push(s.to_string());
        }

        // Exhaustive small-alphabet fuzz: every string up to length 4 over the
        // characters that matter for both grammars.
        let alphabet = b"0129afAF:.";
        for len in 0..=4 {
            let mut indices = vec![0usize; len];
            loop {
                cases.push(indices.iter().map(|&i| alphabet[i] as char).collect());
                let mut pos = 0;
                while pos < len {
                    indices[pos] += 1;
                    if indices[pos] < alphabet.len() {
                        break;
                    }
                    indices[pos] = 0;
                    pos += 1;
                }
                if pos == len {
                    break;
                }
            }
        }

        // Structured pseudo-random strings, biased toward near-valid addresses.
        let mut seed = 0x2545_F491_4F6C_DD1Du64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for _ in 0..200_000 {
            let len = (next() % 44) as usize + 1;
            let s: String = (0..len)
                .map(|_| alphabet[(next() % alphabet.len() as u64) as usize] as char)
                .collect();
            cases.push(s);
        }
        for _ in 0..50_000 {
            let a = next();
            cases.push(format!(
                "{}.{}.{}.{}",
                a as u8,
                (a >> 8) as u8,
                (a >> 16) as u8,
                (a >> 24) as u8
            ));
            let b = next();
            cases.push(format!("{:x}:{:x}::{:x}", a as u16, (a >> 16) as u16, b as u16));
            cases.push(Ipv6Addr::from_bits(((a as u128) << 64) | b as u128).to_string());
        }

        for case in &cases {
            assert_eq!(
                parse_ip(case),
                IpAddr::from_str(case).ok(),
                "parser disagrees with std on {case:?}"
            );
        }
    }
}
