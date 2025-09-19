use std::io::stdout;
use crate::chessboard::Chessboard;
use crate::{print_bitboard_as_chessboard, WHITE};
use crate::r#move::Move;

pub(crate) fn generate_moves(chessboard: &Chessboard, color: i8) -> Vec<Move> {
    let mut moves = Vec::new();

    // color = 1 for white, -1 for black
    get_pawn_moves(&chessboard, color, &mut moves);
    get_knight_moves(&chessboard, color, &mut moves);
    get_bishop_moves(&chessboard, color, &mut moves, None);
    get_rook_moves(&chessboard, color, &mut moves, None);
    get_queen_moves(&chessboard, color, &mut moves);
    get_king_moves(&chessboard, color, &mut moves);

    //remove moves that have no piece on from square
    moves.retain(|m| chessboard.get_piece_at(m.get_from_x()+ m.get_from_y() * 8) != 0);
    moves
}

pub(crate) fn get_pawn_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>) {
    let direction = -color;
    let pawn_mask;
    let pieces;
    let opponent_pieces;
    // todo add an 'en pessant mask' and use that instead of checking the last move
    if color == 1 { 
        pawn_mask = chessboard.get_white_pawns();
        pieces = chessboard.get_white_pieces();
        opponent_pieces = chessboard.get_black_pieces();
    } else { 
        pawn_mask = chessboard.get_black_pawns();
        pieces = chessboard.get_black_pieces();
        opponent_pieces = chessboard.get_white_pieces();
    };

    for square in 0..64 {
        if (pawn_mask & (1 << square)) == 0 { continue; }
        let x = square % 8;
        let y = square / 8;
        //long line of ifs, just check if pawn is on last rank
        let one_step = if y != (if color == WHITE {0} else { 7 }) { 1 << (x + (y + direction) * 8)} else { 0 };
        //check sixth and seventh for white and zeroth and first for black for two_step
        let two_step = if y != (if color == WHITE {0} else { 7 }) && y == (if color == WHITE {6} else { 1 }) { 1 << (x + (y + direction * 2) * 8)} else { 0 };

        let mut capture_left = 0;
        let mut capture_right = 0;
        if y != (if color == WHITE { 0 } else { 7 }) {
             capture_left = if x != 0 { 1 << (x - 1 + (y + direction) * 8) } else { 0 };
            capture_right = if x != 7 { 1 << (x + 1 + (y + direction) * 8) } else { 0 };
        }

        if (pieces & one_step) == 0 && (opponent_pieces & one_step) == 0 {
            moves.push(Move::new(x as u8, y as u8, x as u8, (y + direction) as u8));
            if y == if color == 1 { 6 } else { 1 } && (pieces & two_step) == 0 && (opponent_pieces & two_step) == 0 {
                moves.push(Move::new(x as u8, y as u8, x as u8, (y + direction * 2) as u8));
            }
        }

        if (opponent_pieces & capture_left) != 0 {
            moves.push(Move::new(x as u8, y as u8, (x - 1) as u8, (y + direction) as u8));
        }
        if (opponent_pieces & capture_right) != 0 {
            moves.push(Move::new(x as u8, y as u8, (x + 1) as u8, (y + direction) as u8));
        }

        /*if let Some(last_move) = chessboard.get_history().last() {
            let black_pieces_prev = last_move.get_black_pieces();
            if y == if color == 1 { 4 } else { 3 } && (black_pieces_prev & two_step) != 0 {
                if x != 0 && (opponent_pieces & (1 << (x - 1 + (y + direction) * 8))) != 0 {
                    moves.push(Move::new(x as u8, y as u8, (x - 1) as u8, (y + direction) as u8));
                }
                if x != 7 && (opponent_pieces & (1 << (x + 1 + (y + direction) * 8))) != 0 {
                    moves.push(Move::new(x as u8, y as u8, (x + 1) as u8, (y + direction) as u8));
                }
            }
        }*/
    }
}

pub(crate) fn get_knight_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>){
    let knight_mask = if color == 1 { chessboard.get_white_knights() } else { chessboard.get_black_knights() };
    let own_pieces = if color == 1 { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };
    // important to check if the to square will be outside the board BEFORE making any mask, otherwise it will overflow
    for square in 0..64 {
        if (knight_mask & (1 << square)) == 0 { continue; }
        let x = square % 8;
        let y = square / 8;
        let mut move_mask:u64 = 0;
        if x > 0 && y > 1 { move_mask |= 1 << (x - 1 + (y - 2) * 8); }
        if x > 1 && y > 0 { move_mask |= 1 << (x - 2 + (y - 1) * 8); }
        if x > 1 && y < 7 { move_mask |= 1 << (x - 2 + (y + 1) * 8); }
        if x > 0 && y < 6 { move_mask |= 1 << (x - 1 + (y + 2) * 8); }
        if x < 7 && y < 6 { move_mask |= 1 << (x + 1 + (y + 2) * 8); }
        if x < 6 && y < 7 { move_mask |= 1 << (x + 2 + (y + 1) * 8); }
        if x < 6 && y > 0 { move_mask |= 1 << (x + 2 + (y - 1) * 8); }
        if x < 7 && y > 1 { move_mask |= 1 << (x + 1 + (y - 2) * 8); }
        move_mask &= !own_pieces;
        for i in 0..64 {
            if (move_mask & (1 << i)) != 0 {
                moves.push(Move::new(x as u8, y as u8, i % 8 as u8, i / 8 as u8));
            }
        }
    }
}

pub(crate) fn get_bishop_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>, queen_mask: Option<u64>) {

    // if queen_mask is Some, use that as the bishop mask, otherwise use the bishops from the chessboard
    let bishop_mask = if let Some(q_mask) = queen_mask {
        queen_mask.unwrap()
    } else if color == 1 {
        chessboard.get_white_bishops()
    } else {
        chessboard.get_black_bishops()
    };

    let own_pieces = if color == 1 { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };
    let all_pieces = chessboard.get_white_pieces() | chessboard.get_black_pieces();

    for square in 0..64 {
        if (bishop_mask & (1 << square)) == 0 { continue; }
        let x = square % 8;
        let y = square / 8;
        let mut move_mask:u64 = 0;
        for i in 1..8 {
            //break at board end
            if x + i > 7 || y + i > 7 { break; }
            //add i to both x and y for this clause because were moving diagonally down
            move_mask |= 1 << (x + i + (y + i) * 8);
            //break when we find a piece but we also include the piece
            if (all_pieces & (1 << (x + i + (y + i) * 8))) != 0 { break; }
        }
        // repeat for rest of diagonals
        for i in 1..8 {
            if x - i < 0 || y - i < 0 { break; }
            move_mask |= 1 << (x - i + (y - i) * 8);
            if (all_pieces & (1 << (x - i + (y - i) * 8))) != 0 { break; }
        }
        for i in 1..8 {
            if x + i > 7 || y - i < 0 { break; }
            move_mask |= 1 << (x + i + (y - i) * 8);
            if (all_pieces & (1 << (x + i + (y - i) * 8))) != 0 { break; }
        }
        for i in 1..8 {
            if x - i < 0 || y + i > 7 { break; }
            move_mask |= 1 << (x - i + (y + i) * 8);
            if (all_pieces & (1 << (x - i + (y + i) * 8))) != 0 { break; }
        }

        move_mask &= !own_pieces; // dont eat own pieces
        /*for i in 0..64 {
            if (move_mask & (1 << i)) != 0 {
                moves.push(Move::new(x as u8, y as u8, i % 8 as u8, i / 8 as u8));
            }
        }*/
        // instead of looping through all 64 squares, we can just loop through the bits that are set in move_mask
        let mut temp_mask = move_mask;
        while temp_mask != 0 {
            let lsb = temp_mask & (!temp_mask + 1);
            let target_square = lsb.trailing_zeros() as u8;
            moves.push(Move::new(x as u8, y as u8, target_square % 8, target_square / 8));
            temp_mask &= temp_mask - 1;
        }
    }
}

// same logic as bishop but for straight lines
pub(crate) fn get_rook_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>, queen_mask: Option<u64>) {
    let rook_mask = if let Some(q_mask) = queen_mask {
        queen_mask.unwrap()
    } else if color == 1 {
        chessboard.get_white_rooks()
    } else {
        chessboard.get_black_rooks()
    };

    let own_pieces = if color == 1 { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };
    let all_pieces = chessboard.get_white_pieces() | chessboard.get_black_pieces();
    if color == 1 { chessboard.get_black_pieces() } else { chessboard.get_white_pieces() };

    for square in 0..64 {
        if (rook_mask & (1 << square)) == 0 { continue; }
        let x = square % 8;
        let y = square / 8;
        let mut move_mask:u64 = 0;
        for i in 1..8 {
            if x + i > 7 { break; }
            move_mask |= 1 << (x + i + y * 8);
            if all_pieces & (1 << (x + i + y * 8)) != 0 { break; }
        }
        for i in 1..8 {
            if x - i < 0 { break; }
            move_mask |= 1 << (x - i + y * 8);
            if all_pieces & (1 << (x - i + y * 8)) != 0 { break; }
        }
        for i in 1..8 {
            if y + i > 7 { break; }
            move_mask |= 1 << (x + (y + i) * 8);
            if all_pieces & (1 << (x + (y + i) * 8)) != 0 { break; }

        }
        for i in 1..8 {
            if y - i < 0 { break; }
            move_mask |= 1 << (x + (y - i) * 8);
            if (all_pieces & (1 << (x + (y - i) * 8))) != 0 { break; }
        }

        move_mask &= !own_pieces;
        while move_mask != 0 {
            let lsb = move_mask & (!move_mask + 1);
            let target_square = lsb.trailing_zeros() as u8;
            moves.push(Move::new(x as u8, y as u8, target_square % 8, target_square / 8));
            move_mask &= move_mask - 1;
        }
    }
}

pub(crate) fn get_queen_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>) {
    let queen_mask = if color == 1 { chessboard.get_white_queens() } else { chessboard.get_black_queens() };
    if color == 1 { chessboard.get_black_pieces() } else { chessboard.get_white_pieces() };

    // conveniently reuse bishop and rook move generation for queen moves
    get_bishop_moves(chessboard, color, moves, Some(queen_mask));
    get_rook_moves(chessboard, color, moves, Some(queen_mask));
}

pub(crate) fn get_king_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>){
    let king_mask = if color == WHITE { chessboard.get_white_kings() } else { chessboard.get_black_kings() };
    let pieces = if color == WHITE { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };

    for square in 0..64 {
        if (king_mask & (1u64 << square)) == 0 {
            continue;
        }

        let x = square % 8;
        let y = square / 8;
        let mut move_mask: u64 = 0;

        if x + 1 < 8 && y + 1 < 8 {
            move_mask |= 1u64 << (x + 1 + (y + 1) * 8);
        }

        if x + 1 < 8 {
            move_mask |= 1u64 << (x + 1 + y * 8);
        }

        if x + 1 < 8 && y - 1 >= 0 {
            move_mask |= 1u64 << (x + 1 + (y - 1) * 8);
        }

        if y + 1 < 8 {
            move_mask |= 1u64 << (x + (y + 1) * 8);
        }

        if y - 1 >= 0 {
            move_mask |= 1u64 << (x + (y - 1) * 8);
        }

        if x - 1 >= 0 && y + 1 < 8 {
            move_mask |= 1u64 << (x - 1 + (y + 1) * 8);
        }

        if x - 1 >= 0 {
            move_mask |= 1u64 << (x - 1 + y * 8);
        }

        if x - 1 >= 0 && y - 1 >= 0 {
            move_mask |= 1u64 << (x - 1 + (y - 1) * 8);
        }

        let target_squares = move_mask & !pieces;
        for i in 0..64 {
            if (target_squares & (1u64 << i)) != 0 {
                moves.push(Move::new(x as u8, y as u8, i % 8 as u8, i / 8 as u8));
            }
        }
    }
}

