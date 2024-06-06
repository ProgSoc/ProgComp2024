#![allow(arithmetic_overflow)]

use crate::bitboard::*;
use crate::magics::*;

pub const KING: u64 = 0;
pub const QUEEN: u64 = 1;
pub const ROOK: u64 = 2;
pub const BISHOP: u64 = 3;
pub const KNIGHT: u64 = 4;
pub const NO_PIECE: u64 = 5;

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
pub const fn make_move(origin: u64, dest: u64, piece_type: u64, capture: u64) -> u64 {
    (capture << 15) | (piece_type << 12) | (dest << 6) | origin
}

// Assumes that it does not capture opposing king.
#[inline(always)]
pub const fn play_move(mut pos: Position, m: u64) -> Position {
    let piece_type = (m >> 12) & 0x7;
    let destination = (m >> 6) & 0x3f;
    let origin = m & 0x3f;

    let mut own_king = pos.kings >> 0x3f;
    let other_king = (pos.kings >> 6) & 0x3f;

    // Vacate origin square for own piece
    pos.own &= !(1 << origin);

    // Fill in destination square for own piece
    pos.own |= 1 << destination;

    // Vacate destination square for other piece
    pos.other &= !(1 << destination);
    pos.ortho &= !(1 << destination);
    pos.diag &= !(1 << destination);

    match piece_type {
        KING => {
            own_king = destination;
        },
        QUEEN => {
            pos.ortho &= !(1 << origin);
            pos.ortho |= 1 << destination;
            pos.diag &= !(1 << origin);
            pos.diag |= 1 << destination;
        },
        ROOK => {
            pos.ortho &= !(1 << origin);
            pos.ortho |= 1 << destination;
        },
        BISHOP => {
            pos.diag &= !(1 << origin);
            pos.diag |= 1 << destination;
        },
        KNIGHT => {},
        _ => {},
    }

    pos.kings = (other_king << 6) | own_king;

    pos
}

#[inline(always)]
pub const fn flip_bb(mut bb: u64) -> u64 {
    bb = (bb & 0x00000000ffffffff) << 32 | (bb >> 32) & 0x00000000ffffffff;
    bb = (bb & 0x0000ffff0000ffff) << 16 | (bb >> 16) & 0x0000ffff0000ffff;
    bb = (bb & 0x00ff00ff00ff00ff) << 8 | (bb >> 8) & 0x00ff00ff00ff00ff;
    bb
}

#[inline(always)]
pub const fn flip_square(sq: u64) -> u64 {
    (!sq & 0x38) | (sq & 0x07)
}

#[inline(always)]
#[allow(dead_code)]
pub const fn flip_position(pos: Position) -> Position {
    Position {
        ortho: flip_bb(pos.ortho),
        diag: flip_bb(pos.diag),
        own: flip_bb(pos.other),
        other: flip_bb(pos.own),
        kings: (flip_square(pos.kings & 0x3f) << 6) | flip_square((pos.kings >> 6) & 0x3f),
    }
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
    let knight =
        !(pos.ortho | pos.diag | (1 << (pos.kings & 0x3f)) | (1 << ((pos.kings >> 6) & 0x3f)));

    let bishop = pos.diag & !pos.ortho;
    let rook = pos.ortho & !pos.diag;
    let queen = pos.diag & pos.ortho;

    let opposing_king = (pos.kings >> 6) & 0x3f;

    let blockers = pos.own | pos.other;

    let mut moves: Vec<u64> = Vec::new();

    let mut bb: u64;

    bb = KING_ATTACKS[(pos.kings & 0x3f) as usize]
        & !KING_ATTACKS[((pos.kings >> 6) & 0x3f) as usize]
        & !pos.own;

    while bb != 0 {
        let dest = log2(bb & (!bb + 1));
        let capture = if ((pos.other >> dest) & 1) == 0 { 0 } else { 1 };
        moves.push(make_move(pos.kings & 0x3f, dest, KING, capture));
        bb &= bb - 1;
    }

    for square in 0..64 {
        if ((pos.own >> square) & 1) == 1 {
            let piece_type: u64;

            bb = !pos.own
                & if ((knight >> square) & 1) == 1 {
                    piece_type = KNIGHT;
                    KNIGHT_ATTACKS[square as usize]
                } else if ((bishop >> square) & 1) == 1 {
                    piece_type = BISHOP;
                    sliding_attacks(blockers, &bishop_magics[square as usize])
                } else if ((rook >> square) & 1) == 1 {
                    piece_type = ROOK;
                    sliding_attacks(blockers, &rook_magics[square as usize])
                } else if ((queen >> square) & 1) == 1 {
                    piece_type = QUEEN;
                    sliding_attacks(blockers, &bishop_magics[square as usize])
                        | sliding_attacks(blockers, &rook_magics[square as usize])
                } else {
                    piece_type = NO_PIECE;
                    0
                };

            while bb != 0 {
                let dest = log2(bb & (!bb + 1));
                let capture = if ((pos.other >> dest) & 1) == 0 { 0 } else { 1 };
                moves.push(make_move(square, dest, piece_type, capture));
                bb &= bb - 1;
            }
        }
    }

    moves.into_iter().filter(move |&m| (m >> 6) & 0x3f != opposing_king)
}

pub fn perft(pos: Position, depth: usize, bishop_magics: &[SMagic], rook_magics: &[SMagic]) -> u64 {
    if depth == 1 {
        return pseudo_legal_moves(pos, bishop_magics, rook_magics).count() as u64;
    }
    pseudo_legal_moves(pos, bishop_magics, rook_magics)
        .map(|m| perft(play_move(pos, m), depth - 1, bishop_magics, rook_magics))
        .sum()
}