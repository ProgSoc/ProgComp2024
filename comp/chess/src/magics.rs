use crate::bitboard::*;

#[derive(Clone, Debug)]
pub struct SMagic {
    pub attack_table: Vec<u64>,
    pub mask: u64,
    pub magic: u64,
    pub shift: u64,
}

const BISHOP_MAGICS: [u64; 64] = [
    0xa4406882040823a0,
    0x5010011501020810,
    0x90010041100000,
    0x8060890104020102,
    0x26021000082100,
    0x901008410404,
    0x11090110300421e4,
    0x10820082202618,
    0x820106058212140,
    0x40600400808100,
    0x9410101108410100,
    0x802b844400825000,
    0x2082111040002140,
    0x60e882080c041000,
    0x8624044402601088,
    0x21820088880830,
    0x805005c054050420,
    0x8128202008010243,
    0x5010011501020810,
    0x401201c04028000,
    0x1000820281700,
    0x40120010088400a,
    0x10041402008e0900,
    0x2a00109011000,
    0x250204010e210,
    0x9410101108410100,
    0x810820010240010,
    0x108000502c100,
    0x180840110802000,
    0x10004008a41000,
    0x800850046211000,
    0x22902444d006808,
    0x8888824054100400,
    0x4048041100840101,
    0x44020100020410,
    0x10404880080201,
    0x40028022420020,
    0xa084821880041000,
    0x88a0402084101,
    0x1004204849020100,
    0x804211200880a000,
    0x80108420a005020,
    0x1060820802000500,
    0x1001044010480202,
    0x14104202000190,
    0x6004008083000200,
    0x9410101108410100,
    0x61840100480200,
    0x11090110300421e4,
    0x9044202200000,
    0x90010041100000,
    0xc011850a42020400,
    0x148b15002020000,
    0x80902001444134,
    0x1191100228014202,
    0x5010011501020810,
    0x10820082202618,
    0x21820088880830,
    0x1000004404c5000,
    0x80800000020a0a01,
    0x10d0020200,
    0x21820180220,
    0x820106058212140,
    0xa4406882040823a0,
];

const ROOK_MAGICS: [u64; 64] = [
    0x880088040011021,
    0x6140081000402000,
    0x2300200240281100,
    0x4100280410006100,
    0x80022c00800800,
    0x480120003800400,
    0x3880210002004180,
    0x1200088540220104,
    0x93800084204009,
    0x1400020100040,
    0x1001100c02009,
    0x42800804100080,
    0x20e1002800910015,
    0x8a0808006000c00,
    0x1004002108020410,
    0x10c0800a41000880,
    0x8080014000a0,
    0x8a8040022000,
    0x90008012200180,
    0x1050020100028,
    0x48008004000880,
    0x8a0808006000c00,
    0x40008102126,
    0x100460004440481,
    0x8480400080008020,
    0x1c00200080804000,
    0x200280500080,
    0x2000100080800800,
    0x1000500080010,
    0x6008200040811,
    0x8022013400081022,
    0x10808a00104104,
    0x420400022800380,
    0x1c00200080804000,
    0x1000302001004101,
    0x1010300083800801,
    0x10a3802402800800,
    0x440800600800c00,
    0x14010644001008,
    0x4a822843020005a4,
    0x20a24002848000,
    0x408482010004000,
    0x4328100020008080,
    0x419081001010020,
    0x20e1002800910015,
    0x8a28020004008080,
    0x812100c0001,
    0x84008104420034,
    0x420400022800380,
    0x2400e24001029100,
    0x200280500080,
    0x100108058080,
    0x48008004000880,
    0x4800200040080,
    0x8041085003020c00,
    0x2004281040600,
    0x321044480013023,
    0x1000d04001210481,
    0x200401100082001,
    0x804050020081001,
    0x840100121008008d,
    0x405005208040021,
    0x1025100800820c,
    0x440020844902,
];

pub fn generate_bishop_table() -> Vec<SMagic> {
    let mut bishop_table: Vec<SMagic> = Vec::with_capacity(64);
    for square in 0u64..64u64 {
        if let Some((vision_table, shift)) =
            test_bishop_magic(square, BISHOP_MAGICS[square as usize])
        {
            bishop_table.push(SMagic {
                attack_table: vision_table
                    .into_iter()
                    .take((1 << (64 - shift)) as usize)
                    .collect(),
                mask: bishop_unblocked_attack_rays(square),
                magic: BISHOP_MAGICS[square as usize],
                shift,
            });
        } else {
            panic!(
                "BISHOP PANIC: Square {}: Magic {} is invalid.",
                square, BISHOP_MAGICS[square as usize]
            );
        }
    }
    bishop_table
}

pub fn generate_rook_table() -> Vec<SMagic> {
    let mut rook_table: Vec<SMagic> = Vec::with_capacity(64);
    for square in 0u64..64u64 {
        if let Some((vision_table, shift)) = test_rook_magic(square, ROOK_MAGICS[square as usize]) {
            rook_table.push(SMagic {
                attack_table: vision_table
                    .into_iter()
                    .take((1 << (64 - shift)) as usize)
                    .collect(),
                mask: rook_unblocked_attack_rays(square),
                magic: ROOK_MAGICS[square as usize],
                shift,
            });
        } else {
            panic!(
                "ROOK PANIC: Square {}: Magic {} is invalid.",
                square, ROOK_MAGICS[square as usize]
            );
        }
    }
    rook_table
}
