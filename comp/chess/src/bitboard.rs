// All bitboards are encoded in little endian,
// with indexes 0, 1, 2, ... referring to
// squares a1, a2, a3, ...

// Pre-computed bitboards for the attack pattern of a knight or king at a given position.
pub const KNIGHT_ATTACKS: [u64; 64] = [
    0x20400,
    0x50800,
    0xa1100,
    0x142200,
    0x284400,
    0x508800,
    0xa01000,
    0x402000,
    0x2040004,
    0x5080008,
    0xa110011,
    0x14220022,
    0x28440044,
    0x50880088,
    0xa0100010,
    0x40200020,
    0x204000402,
    0x508000805,
    0xa1100110a,
    0x1422002214,
    0x2844004428,
    0x5088008850,
    0xa0100010a0,
    0x4020002040,
    0x20400040200,
    0x50800080500,
    0xa1100110a00,
    0x142200221400,
    0x284400442800,
    0x508800885000,
    0xa0100010a000,
    0x402000204000,
    0x2040004020000,
    0x5080008050000,
    0xa1100110a0000,
    0x14220022140000,
    0x28440044280000,
    0x50880088500000,
    0xa0100010a00000,
    0x40200020400000,
    0x204000402000000,
    0x508000805000000,
    0xa1100110a000000,
    0x1422002214000000,
    0x2844004428000000,
    0x5088008850000000,
    0xa0100010a0000000,
    0x4020002040000000,
    0x400040200000000,
    0x800080500000000,
    0x1100110a00000000,
    0x2200221400000000,
    0x4400442800000000,
    0x8800885000000000,
    0x100010a000000000,
    0x2000204000000000,
    0x4020000000000,
    0x8050000000000,
    0x110a0000000000,
    0x22140000000000,
    0x44280000000000,
    0x88500000000000,
    0x10a00000000000,
    0x20400000000000,
];

pub const KING_ATTACKS: [u64; 64] = [
    0x302,
    0x705,
    0xe0a,
    0x1c14,
    0x3828,
    0x7050,
    0xe0a0,
    0xc040,
    0x30203,
    0x70507,
    0xe0a0e,
    0x1c141c,
    0x382838,
    0x705070,
    0xe0a0e0,
    0xc040c0,
    0x3020300,
    0x7050700,
    0xe0a0e00,
    0x1c141c00,
    0x38283800,
    0x70507000,
    0xe0a0e000,
    0xc040c000,
    0x302030000,
    0x705070000,
    0xe0a0e0000,
    0x1c141c0000,
    0x3828380000,
    0x7050700000,
    0xe0a0e00000,
    0xc040c00000,
    0x30203000000,
    0x70507000000,
    0xe0a0e000000,
    0x1c141c000000,
    0x382838000000,
    0x705070000000,
    0xe0a0e0000000,
    0xc040c0000000,
    0x3020300000000,
    0x7050700000000,
    0xe0a0e00000000,
    0x1c141c00000000,
    0x38283800000000,
    0x70507000000000,
    0xe0a0e000000000,
    0xc040c000000000,
    0x302030000000000,
    0x705070000000000,
    0xe0a0e0000000000,
    0x1c141c0000000000,
    0x3828380000000000,
    0x7050700000000000,
    0xe0a0e00000000000,
    0xc040c00000000000,
    0x203000000000000,
    0x507000000000000,
    0xa0e000000000000,
    0x141c000000000000,
    0x2838000000000000,
    0x5070000000000000,
    0xa0e0000000000000,
    0x40c0000000000000,
];

// Bitboard constants used for masking occupancy patterns
// that are not needed elsewhere.

const FILE_A: u64 = 0x0101010101010101u64;
const RANK_1: u64 = 0xffu64;
const MAJOR_DIAG: u64 = 0x8040201008040201u64;
const MINOR_DIAG: u64 = 0x0102040810204080u64;
const EDGES: u64 = FILE_A | (FILE_A << 7) | RANK_1 | (RANK_1 << 56);

// Constants used for the quick calculation of log2 of a 64-bit unsigned integer.

// Chosen to be the smallest de Bruijn sequence of order 6 on the binary digits.
const LOG_2_DE_BRUIJN: u64 = 0x218a392cd3d5dbf;

const LOG_2_TABLE: [u64; 64] = [
    0, 1, 2, 7, 3, 13, 8, 19, 4, 25, 14, 28, 9, 34, 20, 40, 5, 17, 26, 38, 15, 46, 29, 48, 10, 31,
    35, 54, 21, 50, 41, 57, 63, 6, 12, 18, 24, 27, 33, 39, 16, 37, 45, 47, 30, 53, 49, 56, 62, 11,
    23, 32, 36, 44, 52, 55, 61, 22, 43, 51, 60, 42, 59, 58,
];

pub fn rook_unblocked_attack_rays(square: u64) -> u64 {
    let rank = square >> 3;
    let file = square & 7;
    let mut rays = (FILE_A << file) | (RANK_1 << (rank << 3));
    if file != 0 {
        rays &= !FILE_A;
    }
    if file != 7 {
        rays &= !(FILE_A << 7);
    }
    if rank != 0 {
        rays &= !RANK_1;
    }
    if rank != 7 {
        rays &= !(RANK_1 << 56);
    }
    rays & !(1 << square)
}

pub fn bishop_unblocked_attack_rays(square: u64) -> u64 {
    let rank = square >> 3;
    let file = square & 7;
    let mut major: u64;
    let mut minor: u64;
    if file >= rank {
        let shift = file - rank;
        major = MAJOR_DIAG << shift;
        for excl_file in 0..shift {
            major &= !(FILE_A << excl_file);
        }
    } else {
        let shift = rank - file;
        major = MAJOR_DIAG >> shift;
        for excl_file in 0..shift {
            major &= !((FILE_A << 7) >> excl_file);
        }
    }
    if file + rank >= 7 {
        let shift = file + rank - 7;
        minor = MINOR_DIAG << shift;
        for excl_file in 0..shift {
            minor &= !(FILE_A << excl_file);
        }
    } else {
        let shift = 7 - file - rank;
        minor = MINOR_DIAG >> shift;
        for excl_file in 0..shift {
            minor &= !((FILE_A << 7) >> excl_file);
        }
    }
    (major | minor) & !(EDGES | (1 << square))
}

// Returning bitboard will be the inclusive attacks, unlike *_unblocked_attack_rays
pub fn rook_blocked_attack_rays(square: u64, blockers: u64) -> u64 {
    let rank = square >> 3;
    let file = square & 7;

    let mut attack_bb = 0u64;

    // `res` is used to capture the square that turns the `take_while` predicate false.
    // This square needs to be included in the bitboard of attacked squares.
    let mut res: u64 = 0;

    // towards west
    attack_bb |= (0..file)
        .rev()
        .take_while(|f| {
            res = 1 << ((rank << 3) | f);
            (blockers & res) == 0
        })
        .map(|f| 1 << ((rank << 3) | f))
        .fold(0u64, |acc, x| acc | x);
    attack_bb |= res;

    res = 0;
    // towards east
    attack_bb |= (file + 1..=7)
        .take_while(|f| {
            res = 1 << ((rank << 3) | f);
            (blockers & res) == 0
        })
        .map(|f| 1 << ((rank << 3) | f))
        .fold(0u64, |acc, x| acc | x);
    attack_bb |= res;

    res = 0;
    // towards south
    attack_bb |= (0..rank)
        .rev()
        .take_while(|r| {
            res = 1 << ((r << 3) | file);
            (blockers & res) == 0
        })
        .map(|r| 1 << ((r << 3) | file))
        .fold(0u64, |acc, x| acc | x);
    attack_bb |= res;

    res = 0;
    // towards north
    attack_bb |= (rank + 1..=7)
        .take_while(|r| {
            res = 1 << ((r << 3) | file);
            (blockers & res) == 0
        })
        .map(|r| 1 << ((r << 3) | file))
        .fold(0u64, |acc, x| acc | x);
    attack_bb |= res;

    attack_bb
}

pub fn bishop_blocked_attack_rays(square: u64, blockers: u64) -> u64 {
    let rank = square >> 3;
    let file = square & 7;
    let mut attack_bb = 0u64;

    let mut res: u64 = 0;

    // towards south-west
    attack_bb |= (0..rank)
        .rev()
        .zip((0..file).rev())
        .take_while(|(r, f)| {
            res = 1 << ((r << 3) | f);
            (blockers & res) == 0
        })
        .map(|(r, f)| 1 << ((r << 3) | f))
        .fold(0u64, |acc, x| acc | x);
    attack_bb |= res;

    res = 0;
    // towards south-east
    attack_bb |= (0..rank)
        .rev()
        .zip(file + 1..=7)
        .take_while(|(r, f)| {
            res = 1 << ((r << 3) | f);
            (blockers & res) == 0
        })
        .map(|(r, f)| 1 << ((r << 3) | f))
        .fold(0u64, |acc, x| acc | x);
    attack_bb |= res;

    res = 0;
    // towards north-west
    attack_bb |= (rank + 1..=7)
        .zip((0..file).rev())
        .take_while(|(r, f)| {
            res = 1 << ((r << 3) | f);
            (blockers & res) == 0
        })
        .map(|(r, f)| 1 << ((r << 3) | f))
        .fold(0u64, |acc, x| acc | x);
    attack_bb |= res;

    res = 0;
    // towards north-east
    attack_bb |= (rank + 1..=7)
        .zip(file + 1..=7)
        .take_while(|(r, f)| {
            res = 1 << ((r << 3) | f);
            (blockers & res) == 0
        })
        .map(|(r, f)| 1 << ((r << 3) | f))
        .fold(0u64, |acc, x| acc | x);
    attack_bb |= res;

    attack_bb
}

// The functions below are used during the generation of magic numbers,
// and can be used to test whether a given number is a magic number.

pub const fn popcount(mut bb: u64) -> u64 {
    let mut count: u64 = 0;
    while bb != 0 {
        count += 1;
        bb &= bb - 1;
    }
    count
}

fn bit_positions(mut bb: u64) -> Vec<u64> {
    let mut positions: Vec<u64> = Vec::new();
    while bb != 0 {
        positions.push(LOG_2_TABLE[(((bb & (!bb + 1)) * LOG_2_DE_BRUIJN) >> 58) as usize]);
        bb &= bb - 1;
    }
    positions
}

#[inline]
pub const fn log2(x: u64) -> u64 {
    LOG_2_TABLE[((x * LOG_2_DE_BRUIJN) >> 58) as usize]
}

pub fn bit_permutations(bb: u64) -> Vec<u64> {
    let mut permutations: Vec<u64> = Vec::new();
    let digits = bit_positions(bb);
    for perm_number in 0..1 << digits.len() {
        let mut value: u64 = 0;
        for (i, d) in digits.iter().enumerate() {
            if ((perm_number >> i) & 1) == 1 {
                value |= 1 << d;
            }
        }
        permutations.push(value);
    }
    permutations
}

pub fn test_rook_magic(square: u64, magic: u64) -> Option<([u64; 4096], u64)> {
    let mask = rook_unblocked_attack_rays(square);
    let shift = 64 - popcount(mask);
    let mut vision_table = [0u64; 4096];
    for blocker_pattern in bit_permutations(mask) {
        let index = (blocker_pattern * magic) >> shift;
        if vision_table[index as usize] == 0 {
            vision_table[index as usize] = rook_blocked_attack_rays(square, blocker_pattern);
        } else {
            return None;
        }
    }
    Some((vision_table, shift))
}

pub fn test_bishop_magic(square: u64, magic: u64) -> Option<([u64; 4096], u64)> {
    let mask = bishop_unblocked_attack_rays(square);
    let shift = 64 - popcount(mask);
    let mut vision_table = [0u64; 4096];
    for blocker_pattern in bit_permutations(mask) {
        let index = (blocker_pattern * magic) >> shift;
        if vision_table[index as usize] == 0 {
            vision_table[index as usize] = bishop_blocked_attack_rays(square, blocker_pattern);
        } else {
            return None;
        }
    }
    Some((vision_table, shift))
}
