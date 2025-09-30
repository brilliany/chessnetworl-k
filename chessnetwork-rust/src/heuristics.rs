use crate::chessboard::Chessboard;
use crate::pieces::Color::White;
use crate::WHITE;

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
pub(crate) struct Heuristics<'a> {
    position: &'a Chessboard,
    color: i8,

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

impl<'a> Heuristics<'a> {
    pub fn new(position: &'a Chessboard, color: i8) -> Heuristics<'a> {
        Heuristics { position, color,
            white_pieces: position.get_white_pieces(),
            black_pieces: position.get_black_pieces(),
            white_pawns: position.get_white_pawns(),
            black_pawns: position.get_black_pawns(),
            white_knights: position.get_white_knights(),
            black_knights: position.get_black_knights(),
            white_bishops: position.get_white_bishops(),
            black_bishops: position.get_black_bishops(),
            white_rooks: position.get_white_rooks(),
            black_rooks: position.get_black_rooks(),
            white_queens: position.get_white_queens(),
            black_queens: position.get_black_queens(),
            white_kings: position.get_white_kings(),
            black_kings: position.get_black_kings()
        }
    }

    pub fn two_middle_pawns(&self) -> i32 {
        let four_center_squares: u64 = 1 << 27 | 1 << 28 | 1 << 35 | 1 << 36;
        let mut score: i32 = 0;
        if self.color == White {
            score += (self.white_pawns & four_center_squares).count_ones() as i32 * TWO_MIDDLE_PAWN_BONUS;
        } else {
            score += (self.black_pawns & four_center_squares).count_ones() as i32 * TWO_MIDDLE_PAWN_BONUS;
        }
        score
    }

    // castling score, give bonus if castling is still possible, give bonus * 2 if castled
    pub fn castling(&self) -> i32 {
        let mut score: i32 = 0;
        if self.color == White {
            let white_king_short: u64 = self.white_kings & (1 << 62);
            let white_rook_short: u64 = self.white_rooks & (1 << 61);
            let white_king_long: u64 = self.white_kings & (1 << 58);
            let white_rook_long: u64 = self.white_rooks & (1 << 59);
            if white_king_short != 0 && white_rook_short != 0 {
                score += CASTLING_SCORE;
            }
            if white_king_long != 0 && white_rook_long != 0 {
                score += CASTLING_SCORE;
            }
        } else {
            let black_king_short: u64 = self.black_kings & (1 << 6);
            let black_rook_short: u64 = self.black_rooks & (1 << 5);
            let black_king_long: u64 = self.black_kings & (1 << 2);
            let black_rook_long: u64 = self.black_rooks & (1 << 3);
            if black_king_short != 0 && black_rook_short != 0 {
                score += CASTLING_SCORE;
            }
            if black_king_long != 0 && black_rook_long != 0 {
                score += CASTLING_SCORE;
            }
        }
        score
    }

    pub fn development(&self) -> i32 {
        let mut score: i32 = 0;
        if self.color == White {
            score += (self.white_knights & !(1 << 57 | 1 << 62)).count_ones() as i32 * DEVELOPMENT_SCORE;
            // println!("score after knights: {}", score);
            score += (self.white_bishops & !(1 << 58 | 1 << 61)).count_ones() as i32 * DEVELOPMENT_SCORE;
            // println!("score after bishops: {}", score);
            score += (self.white_rooks & !(1 << 56 | 1 << 63)).count_ones() as i32 * DEVELOPMENT_SCORE;
            // println!("score after rooks: {}", score);
            score += (self.white_queens & !(1 << 59)).count_ones() as i32 * DEVELOPMENT_SCORE;
            // println!("score after queens: {}", score);
        } else {
            score += (self.black_knights & !(1 << 1 | 1 << 6)).count_ones() as i32 * DEVELOPMENT_SCORE;
            // println!("score after knights: {}", score);
            score += (self.black_bishops & !(1 << 2 | 1 << 5)).count_ones() as i32 * DEVELOPMENT_SCORE;
            // println!("score after bishops: {}", score);
            score += (self.black_rooks & !(1 << 0 | 1 << 7)).count_ones() as i32 * DEVELOPMENT_SCORE;
            // println!("score after rooks: {}", score);
            score += (self.black_queens & !(1 << 3)).count_ones() as i32 * DEVELOPMENT_SCORE;
            // println!("score after queens: {}", score);
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
        if self.color == White {
            let center_knights: u64 = self.white_knights & SIXTEEN_CENTER_SQUARES;
            let defended_squares: u64 = (self.white_pawns & !LEFT_BOARD_EDGE) << 7 | (self.white_pawns & !RIGHT_BOARD_EDGE) << 9;
            score += (center_knights & defended_squares).count_ones() as i32 * KNIGHT_OUTPOST_SCORE*2;
            score += (center_knights & !defended_squares).count_ones() as i32 * KNIGHT_OUTPOST_SCORE;
            score -= (self.white_knights & RIGHT_BOARD_EDGE).count_ones() as i32 * KNIGHT_OUTPOST_SCORE;
            score -= (self.white_knights & LEFT_BOARD_EDGE).count_ones() as i32 * KNIGHT_OUTPOST_SCORE;
        } else {
            let center_knights: u64 = self.black_knights & SIXTEEN_CENTER_SQUARES;
            let defended_squares: u64 = (self.black_pawns & !LEFT_BOARD_EDGE) >> 9 | (self.black_pawns & !RIGHT_BOARD_EDGE) >> 7;
            score += (center_knights & defended_squares).count_ones() as i32 * KNIGHT_OUTPOST_SCORE*2;
            score += (center_knights & !defended_squares).count_ones() as i32 * KNIGHT_OUTPOST_SCORE;
            score -= (self.black_knights & RIGHT_BOARD_EDGE).count_ones() as i32 * KNIGHT_OUTPOST_SCORE;
            score -= (self.black_knights & LEFT_BOARD_EDGE).count_ones() as i32 * KNIGHT_OUTPOST_SCORE;
        }
        score
    }
}
