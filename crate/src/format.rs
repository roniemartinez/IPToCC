pub const FIRST_LEVEL_COUNT: usize = 65537;
pub const V6_BUCKET_COUNT: usize = 65536;
pub const V6_SUB_INDEX_LEN: usize = 257;
pub const V4_GAP_SENTINEL: [u8; 2] = [0xFF, 0xFF];
pub const V6_BUCKET_EMPTY: u8 = 0xFF;

pub const V6_REC_SIZE: usize = 6;
pub const V6_SIDE_SIZE: usize = 33;
pub const V6_IRREGULAR: u8 = 0;

pub const TAG_MIXED: u32 = 0 << 30;
pub const TAG_PURE: u32 = 1 << 30;
pub const TAG_EMPTY: u32 = 2 << 30;
pub const TAG_MASK: u32 = 3 << 30;
pub const V4_CC_GAP: u8 = 0xFF;
