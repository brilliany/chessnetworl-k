use crate::chessboard::Chessboard;
use crate::r#move::Move;
use crate::{print_bitboard_as_chessboard, BISHOP, BLACK, FILE_A, FILE_H, KING, KNIGHT, PAWN, QUEEN, RANK_0, RANK_1, RANK_2, RANK_6, RANK_7, ROOK, WHITE};

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
    /*moves.retain(|m| chessboard.get_piece_at(m.get_from_x()+ m.get_from_y() * 8) != 0);*/
    
    moves
}

//todo castling, en pessant, promotion

pub(crate) fn get_pawn_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>) {
    // todo add an 'en pessant mask' and use that instead of checking the last move
    if color == WHITE {
        let pieces = chessboard.get_white_pieces();
        let opponent_pieces = chessboard.get_black_pieces();
        
        let pawns = chessboard.get_piece_mask(PAWN, WHITE);

        //mask for moves forward
        let mut move_mask =
            //we dont care about pawns on the last ranks
            (pawns &! RANK_7)
            //dont generate when pieces are in front
            &! ((pieces | opponent_pieces) >> 8);

        //store mask for pawns that can move two steps on rank 6 (from white perspective)
        let two_step_pawns = pawns & RANK_1;
        

        //masks for capturing moves
        let mut right_capture_mask = ((pawns &! FILE_H) >> 7) & opponent_pieces;
        let mut left_capture_mask = (pawns &! FILE_A) >> 9 & opponent_pieces;
        
        while move_mask != 0 {
            let from_square = 1u64 << move_mask.trailing_zeros();
            let to_square = from_square << 8;
            moves.push(Move::new(from_square, to_square));
            move_mask &=! from_square;
        }
        //two step moves
        while two_step_pawns != 0 {
            let from_square = 1u64 << two_step_pawns.trailing_zeros();
            let to_square = from_square << 16;
            moves.push(Move::new(from_square, to_square));
            move_mask &=! from_square;
        }
        //capturing moves
        while left_capture_mask != 0 {
            let to_square = 1u64 << left_capture_mask.trailing_zeros();
            let from_square = to_square << 9;
            moves.push(Move::new(from_square, to_square));
            left_capture_mask &=! to_square;
        }
        while right_capture_mask != 0 {
            let to_square = 1u64 << right_capture_mask.trailing_zeros();
            let from_square = to_square << 7;
            moves.push(Move::new(from_square, to_square));
            right_capture_mask &=! to_square;
        }
    } else {
        let pieces = chessboard.get_black_pieces();
        let opponent_pieces = chessboard.get_white_pieces();

        let pawns = chessboard.get_piece_mask(PAWN, BLACK);

        //mask for moves forward
        let mut move_mask =
            //we dont care about pawns on the last ranks
            (pawns &! RANK_0)
            //dont generate when pieces are in front
            &! ((pieces | opponent_pieces) >> 8);

        //store mask for pawns that can move two steps on rank 1 (from black perspective)
        let two_step_pawns = move_mask & RANK_1;

        //masks for capturing moves
        let mut right_capture_mask = ((pawns &! FILE_H) << 9) & opponent_pieces;
        let mut left_capture_mask = (pawns &! FILE_A) << 7 & opponent_pieces;

        while move_mask !=0 {
            let to_square = 1u64 << move_mask.trailing_zeros();
            let from_square = to_square >> 8;
            moves.push(Move::new(from_square, to_square));
            move_mask &=! to_square;
        }
        //two step moves
        while two_step_pawns != 0 {
            let from_square = 1u64 << two_step_pawns.trailing_zeros();
            let to_square = from_square << 16;
            moves.push(Move::new(from_square, to_square));
            move_mask &=! from_square;
        }
        //capturing moves
        while left_capture_mask != 0 {
            let to_square = 1u64 << left_capture_mask.trailing_zeros();
            let from_square = to_square >> 7;
            moves.push(Move::new(from_square, to_square));
            left_capture_mask &=! to_square;
        }
        while right_capture_mask != 0 {
            let to_square = 1u64 << right_capture_mask.trailing_zeros();
            let from_square = to_square >> 9;
            moves.push(Move::new(from_square, to_square));
            right_capture_mask &=! to_square;
        }
    };
    
    //todo en passant
        //en passant
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

pub(crate) fn get_knight_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>){
    let mut knight_mask = if color == 1 { chessboard.get_piece_mask(KNIGHT, WHITE) } else { chessboard.get_piece_mask(KNIGHT, BLACK) };
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
                moves.push(Move::new(1u64 << square, 1u64 << i));
            }
        }
    }
}

pub(crate) fn get_bishop_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>, queen_mask: Option<u64>) {

    // if queen_mask is Some, use that as the bishop mask, otherwise use the bishops from the chessboard
    let bishop_mask = if let Some(q_mask) = queen_mask {
        queen_mask.unwrap()
    } else if color == 1 {
        chessboard.get_piece_mask(BISHOP, WHITE)
    } else {
        chessboard.get_piece_mask(BISHOP, BLACK)
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

        let mut temp_mask = move_mask;
        while temp_mask != 0 {
            let from_square = 1u64 << square;
            let to_square = 1u64 << temp_mask.trailing_zeros();
            moves.push(Move::new(from_square, to_square));
            temp_mask &= temp_mask - 1;
        }
    }
}

// same logic as bishop but for straight lines
pub(crate) fn get_rook_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>, queen_mask: Option<u64>) {
    let rook_mask = if let Some(q_mask) = queen_mask {
        queen_mask.unwrap()
    } else if color == 1 {
        chessboard.get_piece_mask(ROOK, WHITE)
    } else {
        chessboard.get_piece_mask(ROOK, BLACK)
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
        let mut temp_mask = move_mask;
        while temp_mask != 0 {
            let from_square = 1u64 << square;
            let to_square = 1u64 << temp_mask.trailing_zeros();
            moves.push(Move::new(from_square, to_square));
            temp_mask &= temp_mask - 1;
        }
    }
}

pub(crate) fn get_queen_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>) {
    let queen_mask = chessboard.get_piece_mask(QUEEN, color);
    // conveniently reuse bishop and rook move generation for queen moves
    get_bishop_moves(chessboard, color, moves, Some(queen_mask));
    get_rook_moves(chessboard, color, moves, Some(queen_mask));
}

pub(crate) fn get_king_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>){
    let king_mask = chessboard.get_piece_mask(KING, color);
    let pieces = if color == WHITE { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };

    //todo
}

