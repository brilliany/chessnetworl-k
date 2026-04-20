use crate::chessboard::Chessboard;
use crate::{BISHOP, BLACK, KNIGHT, PAWN, WHITE};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct HeuristicParams {
    pub two_middle_pawns_weight: i32,
    pub castling_weight: i32,
    pub development_weight: i32,
    pub mobility_weight: i32,
    pub knight_outpost_weight: i32,
}

impl Default for HeuristicParams {
    fn default() -> Self {
        Self {
            two_middle_pawns_weight: 10,
            castling_weight: 10,
            development_weight: 10,
            mobility_weight: 10,
            knight_outpost_weight: 10,
        }
    }
}

//pawn structure score
const PAWN_STRUCTURE_SCORE: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
// two middle pawn bonus
const TWO_MIDDLE_PAWN_BONUS: i32 = 10;
//castling score
const CASTLING_SCORE: i32 = 10;
//development score
const DEVELOPMENT_SCORE: i32 = 10;
//mobility score
const MOBILITY_SCORE: i32 = 10;
//knight outpost score
const KNIGHT_OUTPOST_SCORE: i32 = 10;
//bishop outpost score
const BISHOP_OUTPOST_SCORE: i32 = 10;

const SIXTEEN_CENTER_SQUARES: u64 = 1 << 27 | 1 << 28 | 1 << 35 | 1 << 36 | 1 << 19 | 1 << 20 | 1 << 27 | 1 << 28 | 1 << 35 | 1 << 36 | 1 << 19 | 1 << 20 | 1 << 27 | 1 << 28 | 1 << 35 | 1 << 36;

const LEFT_BOARD_EDGE : u64 = 1 << 0 | 1 << 8 | 1 << 16 | 1 << 24 | 1 << 32 | 1 << 40 | 1 << 48 | 1 << 56;
const RIGHT_BOARD_EDGE : u64 = 1 << 7 | 1 << 15 | 1 << 23 | 1 << 31 | 1 << 39 | 1 << 47 | 1 << 55 | 1 << 63;
pub(crate) struct Heuristics{
    color: u8,

    white_pieces: u64,
    black_pieces: u64,

    white_pawns: u64,
    black_pawns: u64,
    white_knights: u64,
    black_knights: u64,
    white_bishops: u64,
    black_bishops: u64,
    white_rooks: u64,
    black_rooks: u64,
    white_queens: u64,
    black_queens: u64,
    white_kings: u64,
    black_kings: u64,
}

impl Heuristics {
    pub fn new(position: &Chessboard, color: u8) -> Heuristics {
        Heuristics { color,
            white_pieces: position.get_white_pieces(),
            black_pieces: position.get_black_pieces(),
            white_pawns: position.get_piece_mask(PAWN, WHITE),
            black_pawns: position.get_piece_mask(PAWN, BLACK),
            white_knights: position.get_piece_mask(KNIGHT, WHITE),
            black_knights: position.get_piece_mask(KNIGHT, BLACK),
            white_bishops: position.get_piece_mask(BISHOP, WHITE),
            black_bishops: position.get_piece_mask(BISHOP, BLACK),
            white_rooks: position.get_piece_mask(crate::ROOK, WHITE),
            black_rooks: position.get_piece_mask(crate::ROOK, BLACK),
            white_queens: position.get_piece_mask(crate::QUEEN, WHITE),
            black_queens: position.get_piece_mask(crate::QUEEN, BLACK),
            white_kings: position.get_piece_mask(crate::KING, WHITE),
            black_kings: position.get_piece_mask(crate::KING, BLACK),
        }
    }

    pub fn two_middle_pawns(&self) -> i32 {
        let four_center_squares: u64 = 1 << 27 | 1 << 28 | 1 << 35 | 1 << 36;
        let mut score: i32 = 0;
        if self.color == WHITE {
            score += (self.white_pawns & four_center_squares).count_ones() as i32;
        } else {
            score += (self.black_pawns & four_center_squares).count_ones() as i32;
        }
        score
    }

    // castling score, give bonus if castling is still possible, give bonus * 2 if castled
    pub fn castling(&self) -> i32 {
        let mut score: i32 = 0;
        if self.color == WHITE {
            let white_king_long: u64 = self.white_kings & (1 << 6);
            let white_rook_long: u64 = self.white_rooks & (1 << 5);
            let white_king_short: u64 = self.white_kings & (1 << 1);
            let white_rook_short: u64 = self.white_rooks & (1 << 2);
            if white_king_short != 0 && white_rook_short != 0 {
                score += 1;
            }
            if white_king_long != 0 && white_rook_long != 0 {
                score += 1;
            }
        } else {
            let black_king_long: u64 = self.black_kings & (1 << 61);
            let black_rook_long: u64 = self.black_rooks & (1 << 60);
            let black_king_short: u64 = self.black_kings & (1 << 57);
            let black_rook_short: u64 = self.black_rooks & (1 << 58);
            if black_king_short != 0 && black_rook_short != 0 {
                score += 1;
            }
            if black_king_long != 0 && black_rook_long != 0 {
                score += 1;
            }
        }
        score
    }

    pub fn development(&self) -> i32 {
        let mut score: i32 = 0;
        if self.color == WHITE {
            score += (self.white_knights & !(1 << 1 | 1 << 6)).count_ones() as i32;
            score += (self.white_bishops & !(1 << 2 | 1 << 5)).count_ones() as i32;
            score += (self.white_rooks & !(1 << 0 | 1 << 7)).count_ones() as i32;
            score += (self.white_queens & !(1 << 4)).count_ones() as i32;
        } else {
            score += (self.black_knights & !(1 << 57 | 1 << 62)).count_ones() as i32;
            score += (self.black_bishops & !(1 << 58 | 1 << 61)).count_ones() as i32;
            score += (self.black_rooks & !(1 << 56 | 1 << 63)).count_ones() as i32;
            score += (self.black_queens & !(1 << 59)).count_ones() as i32;
        }
        score
    }

    // mobility score, give bonus for each legal move
    pub fn mobility(&self) -> i32 {
        0
    }

    // knight outpost score, give bonus if knight is in middle 16 squares and double if protected by pawn
    pub fn knight_outpost(&self) -> i32 {
        let mut score: i32 = 0;
        if self.color == WHITE {
            let center_knights: u64 = self.white_knights & SIXTEEN_CENTER_SQUARES;
            let defended_squares: u64 = (self.white_pawns & !LEFT_BOARD_EDGE) << 7 | (self.white_pawns & !RIGHT_BOARD_EDGE) << 9;
            score += (center_knights & defended_squares).count_ones() as i32 * 2;
            score += (center_knights & !defended_squares).count_ones() as i32;
            score -= (self.white_knights & RIGHT_BOARD_EDGE).count_ones() as i32;
            score -= (self.white_knights & LEFT_BOARD_EDGE).count_ones() as i32;
        } else {
            let center_knights: u64 = self.black_knights & SIXTEEN_CENTER_SQUARES;
            let defended_squares: u64 = (self.black_pawns & !LEFT_BOARD_EDGE) >> 9 | (self.black_pawns & !RIGHT_BOARD_EDGE) >> 7;
            score += (center_knights & defended_squares).count_ones() as i32 * 2;
            score += (center_knights & !defended_squares).count_ones() as i32;
            score -= (self.black_knights & RIGHT_BOARD_EDGE).count_ones() as i32;
            score -= (self.black_knights & LEFT_BOARD_EDGE).count_ones() as i32;
        }
        score
    }
}