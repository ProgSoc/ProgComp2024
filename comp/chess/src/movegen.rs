#![allow(arithmetic_overflow)]

use crate::bitboard::*;
use crate::magics::*;

// This struct is side agnostic, and does not track who is to move.
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub ortho: u64,
    pub diag: u64,
    // `own` and `other` will contain king positions as well.
    pub own: u64,
    pub other: u64,
    // own king position is at least significant 6 bits,
    // other king positions is at the next significant 6 bits.
    pub kings: u64,
}

// All bitboards are encoded in little endian,
// with indexes 0, 1, 2, ... referring to
// squares a1, a2, a3, etc.

#[inline(always)]
pub const fn make_move(origin: u64, dest: u64,) -> u64 {
    (dest << 6) | origin
}

// Assumes that it does not capture opposing king.
#[inline(always)]
pub const fn play_move(mut pos: Position, m: u64) -> Position {
    let destination = (m >> 6) & 0x3f;
    let origin = m & 0x3f;

    let mut own_king = pos.kings & 0x3f;
    let other_king = (pos.kings >> 6) & 0x3f;

    // Vacate origin square for own piece
    pos.own &= !(1 << origin);

    // Fill in destination square for own piece
    pos.own |= 1 << destination;

    // Vacate destination square for other piece
    pos.other &= !(1 << destination);
    pos.ortho &= !(1 << destination);
    pos.diag &= !(1 << destination);

    if origin == own_king {
        own_king = destination;
    } else if (((pos.ortho & pos.diag) >> origin) & 1) == 1 {
        pos.ortho &= !(1 << origin);
        pos.ortho |= 1 << destination;
        pos.diag &= !(1 << origin);
        pos.diag |= 1 << destination;
    } else if ((pos.ortho >> origin) & 1) == 1 {
        pos.ortho &= !(1 << origin);
        pos.ortho |= 1 << destination;
    } else if ((pos.diag >> origin) & 1) == 1 {
        pos.diag &= !(1 << origin);
        pos.diag |= 1 << destination;
    }

    pos.kings = (other_king << 6) | own_king;

    pos
}

#[inline(always)]
fn sliding_attacks(blockers: u64, m: &SMagic) -> u64 {
    m.attack_table[(((blockers & m.mask) * m.magic) >> m.shift) as usize]
}

pub fn pseudo_legal_moves(
    pos: Position,
    bishop_magics: &[SMagic],
    rook_magics: &[SMagic],
) -> impl Iterator<Item = u64> {

    let opposing_king = (pos.kings >> 6) & 0x3f;

    let bishop = pos.diag & !pos.ortho;
    let rook = pos.ortho & !pos.diag;
    let queen = pos.diag & pos.ortho;
    let knight = !(pos.ortho | pos.diag | (1 << (pos.kings & 0x3f)) | (1 << opposing_king));

    let blockers = pos.own | pos.other;

    let mut moves: Vec<u64> = Vec::new();

    let mut bb: u64;

    bb = KING_ATTACKS[(pos.kings & 0x3f) as usize] & !pos.own;

    while bb != 0 {
        let dest = log2(bb & (!bb + 1));
        moves.push(make_move(pos.kings & 0x3f, dest));
        bb &= bb - 1;
    }

    for square in 0..64 {
        if ((pos.own >> square) & 1) == 1 {
            bb = !pos.own
                & if ((knight >> square) & 1) == 1 {
                    KNIGHT_ATTACKS[square as usize]
                } else if ((bishop >> square) & 1) == 1 {
                    sliding_attacks(blockers, &bishop_magics[square as usize])
                } else if ((rook >> square) & 1) == 1 {
                    sliding_attacks(blockers, &rook_magics[square as usize])
                } else if ((queen >> square) & 1) == 1 {
                    sliding_attacks(blockers, &bishop_magics[square as usize])
                        | sliding_attacks(blockers, &rook_magics[square as usize])
                } else {
                    0
                };

            while bb != 0 {
                let dest = log2(bb & (!bb + 1));
                moves.push(make_move(square, dest));
                bb &= bb - 1;
            }
        }
    }

    moves.into_iter().filter(move |&m| (m >> 6) & 0x3f != opposing_king)
}

pub fn perft(pos: Position, depth: usize, bishop_magics: &[SMagic], rook_magics: &[SMagic]) -> u64 {
    if depth == 0 {
        return 1;
    } else if depth == 1 {
        return pseudo_legal_moves(pos, bishop_magics, rook_magics).count() as u64;
    }
    pseudo_legal_moves(pos, bishop_magics, rook_magics)
        .map(|m| perft(play_move(pos, m), depth - 1, bishop_magics, rook_magics))
        .sum()
}