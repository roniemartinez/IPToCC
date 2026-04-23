// First-level bucket table size, shared by both protocols: one slot per top-16-bit prefix.
pub const BUCKET_COUNT: usize = 65536; // 2^16

// Sentinel for u8 lookup slots in both protocols: "no country assigned here".
pub const UNASSIGNED: u8 = 0xFF; // largest u8; real cc indices stay below 255

// IPv4 first-level tag bits (top 2 of each u32).
pub const TAG_MIXED: u32 = 0 << 30;
pub const TAG_PURE: u32 = 1 << 30;
pub const TAG_EMPTY: u32 = 2 << 30;
pub const TAG_MASK: u32 = 3 << 30; // top 2 bits

// IPv4 deep narrow: 16-way split on the top 4 bits of the within-bucket key.
pub const V4_DEEP_TOP4_LEN: usize = 17; // 2^4 + 1 total-size sentinel
pub const V4_DENSE_THRESHOLD: usize = 32; // tunable: min transitions to build a deep index

// Byte-wise v6 sub-indices, same layout at every depth.
pub const V6_BYTE_INDEX_LEN: usize = 257; // 2^8 + 1 total-size sentinel
pub const V6_DENSE_THRESHOLD: usize = 64; // tunable: min entries to build a deep index

// IPv6 primary record (6 bytes): u32 offset_or_side_idx, u8 prefix_len, u8 cc_idx.
pub const V6_REC_SIZE: usize = 6; // 4 + 1 + 1
pub const V6_SIDE_SIZE: usize = 33; // u128 start + u128 end + u8 cc_idx
pub const V6_IRREGULAR: u8 = 0; // prefix_len == 0 means "look in side table"; real prefix_len is always >= 1
