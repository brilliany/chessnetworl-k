use crate::chessboard::Chessboard;
use crate::r#move::Move;
use crate::{BISHOP, BLACK, BLACK_KINGSIDE_CASTLE, BLACK_QUEENSIDE_CASTLE, FILES, KING, KNIGHT, PAWN, QUEEN, RANKS, ROOK, WHITE, WHITE_KINGSIDE_CASTLE, WHITE_QUEENSIDE_CASTLE};

pub fn generate_moves(chessboard: &mut Chessboard, color: i8) -> Vec<Move> {
    let mut moves = Vec::new();

    get_pawn_moves(chessboard, color, &mut moves);
    get_knight_moves(chessboard, color, &mut moves);
    get_bishop_moves(chessboard, color, &mut moves, None);
    get_rook_moves(chessboard, color, &mut moves, None);
    get_queen_moves(chessboard, color, &mut moves);
    get_king_moves(chessboard, color, &mut moves);

    moves.retain(|mv| {
        chessboard.make_move(*mv);
        let king_mask = chessboard.get_piece_mask(KING, color);
        let legal = !is_square_attacked(chessboard, king_mask, -color);
        chessboard.undo_move();
        legal
    });

    moves
}

//todo castling, en pessant, promotion
pub fn get_pawn_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>) {
    // todo add an 'en pessant mask' and use that instead of checking the last move
    if color == WHITE {
        let pieces = chessboard.get_white_pieces();
        let opponent_pieces = chessboard.get_black_pieces();
        let pawns = chessboard.get_piece_mask(PAWN, WHITE);

        //mask for moves forward
        let mut move_mask =
            //we dont care about pawns on the last ranks
            (pawns & !RANKS[7]) << 8
                //dont generate when pieces are in front
                & !(pieces | opponent_pieces);

        let mut promotion_mask = move_mask & RANKS[7];

        move_mask &= !promotion_mask;

        //store mask for pawns that can move two steps on rank 6 (from white perspective)
        let mut two_step_pawns = ((move_mask & RANKS[2]) << 8) & !(pieces | opponent_pieces);

        //masks for capturing moves (normal captures only)
        let mut right_capture_mask = ((pawns & !FILES[0]) << 7) & opponent_pieces;
        let mut left_capture_mask = (pawns & !FILES[7]) << 9 & opponent_pieces;

        let mut right_promotion_capture_mask = right_capture_mask & RANKS[7];
        let mut left_promotion_capture_mask = left_capture_mask & RANKS[7];

        right_capture_mask &= !right_promotion_capture_mask;
        left_capture_mask &= !left_promotion_capture_mask;

        while move_mask != 0 {
            let to_square = 1u64 << move_mask.trailing_zeros();
            let from_square = to_square >> 8;
            moves.push(Move::new(from_square, to_square));
            move_mask &= move_mask - 1;
        }
        //two step moves
        while two_step_pawns != 0 {
            let to_square = 1u64 << two_step_pawns.trailing_zeros();
            let from_square = to_square >> 16;
            moves.push(Move::new(from_square, to_square));
            two_step_pawns &= two_step_pawns - 1;
        }
        //capturing moves
        while left_capture_mask != 0 {
            let to_square = 1u64 << left_capture_mask.trailing_zeros();
            let from_square = to_square >> 9;
            moves.push(Move::new(from_square, to_square));
            left_capture_mask &= left_capture_mask - 1;
        }
        while right_capture_mask != 0 {
            let to_square = 1u64 << right_capture_mask.trailing_zeros();
            let from_square = to_square >> 7;
            moves.push(Move::new(from_square, to_square));
            right_capture_mask &= right_capture_mask - 1;
        }

        let target_file = chessboard.get_en_passant();
        if target_file != 0 {
            let to_square = 1u64 << ((target_file - 1) as u32 + 40);
            let captured_square = to_square >> 8;

            // check if the right capture can perform en passant
            let right_capture = (captured_square & !FILES[0]) >> 1;
            if right_capture & pawns != 0 {
                let from_square = right_capture;
                moves.push(Move::en_passant(from_square, to_square, captured_square));
            }

            // check if the left capture can perform en passant
            let left_capture = (captured_square & !FILES[7]) << 1;
            if left_capture & pawns != 0 {
                let from_square = left_capture;
                moves.push(Move::en_passant(from_square, to_square, captured_square));
            }
        }

        while left_promotion_capture_mask != 0 {
            let to_square = 1u64 << left_promotion_capture_mask.trailing_zeros();
            let from_square = to_square >> 9;
            for piece_type in [QUEEN, ROOK, BISHOP, KNIGHT] {
                moves.push(Move::promotion(from_square, to_square, piece_type));
            }
            left_promotion_capture_mask &= left_promotion_capture_mask - 1;
        }

        while right_promotion_capture_mask != 0 {
            let to_square = 1u64 << right_promotion_capture_mask.trailing_zeros();
            let from_square = to_square >> 7;
            for piece_type in [QUEEN, ROOK, BISHOP, KNIGHT] {
                moves.push(Move::promotion(from_square, to_square, piece_type));
            }
            right_promotion_capture_mask &= right_promotion_capture_mask - 1;
        }

        while promotion_mask != 0 {
            let to_square = 1u64 << promotion_mask.trailing_zeros();
            let from_square = to_square >> 8;
            for piece_type in [QUEEN, ROOK, BISHOP, KNIGHT] {
                moves.push(Move::promotion(from_square, to_square, piece_type));
            }
            promotion_mask &= promotion_mask - 1;
        }
    } else {
        let pieces = chessboard.get_black_pieces();
        let opponent_pieces = chessboard.get_white_pieces();

        let pawns = chessboard.get_piece_mask(PAWN, BLACK);

        //mask for moves forward
        let mut move_mask =
            //we dont care about pawns on the last ranks
            (pawns & !RANKS[0]) >> 8
                //dont generate when pieces are in front
                & !((pieces | opponent_pieces));

        let mut promotion_mask = move_mask & RANKS[0];

        move_mask &= !promotion_mask;

        //store mask for pawns that can move two steps on rank 1 (from black perspective)
        let mut two_step_pawns = ((move_mask & RANKS[5]) >> 8) & !((pieces | opponent_pieces));

        //masks for capturing moves (normal captures only)
        let mut right_capture_mask = ((pawns & !FILES[0]) >> 9) & opponent_pieces;
        let mut left_capture_mask = (pawns & !FILES[7]) >> 7 & opponent_pieces;

        let mut right_promotion_capture_mask = right_capture_mask & RANKS[0];
        let mut left_promotion_capture_mask = left_capture_mask & RANKS[0];

        right_capture_mask &= !right_promotion_capture_mask;
        left_capture_mask &= !left_promotion_capture_mask;

        while move_mask != 0 {
            let to_square = 1u64 << move_mask.trailing_zeros();
            let from_square = to_square << 8;
            moves.push(Move::new(from_square, to_square));
            move_mask &= move_mask - 1;
        }
        //two step moves
        while two_step_pawns != 0 {
            let to_square = 1u64 << two_step_pawns.trailing_zeros();
            let from_square = to_square << 16;
            moves.push(Move::new(from_square, to_square));
            two_step_pawns &= two_step_pawns - 1;
        }
        //capturing moves
        while left_capture_mask != 0 {
            let to_square = 1u64 << left_capture_mask.trailing_zeros();
            let from_square = to_square << 7;
            moves.push(Move::new(from_square, to_square));
            left_capture_mask &= left_capture_mask - 1;
        }
        while right_capture_mask != 0 {
            let to_square = 1u64 << right_capture_mask.trailing_zeros();
            let from_square = to_square << 9;
            moves.push(Move::new(from_square, to_square));
            right_capture_mask &= right_capture_mask - 1;
        }


        // En passant captures
        let target_file = chessboard.get_en_passant();
        if target_file != 0 {
            let to_square = 1u64 << ((target_file - 1) as u32 + 16);
            let captured_square = to_square << 8;

            // check if the right capture can perform en passant
            let right_capture = (captured_square & !FILES[7]) << 1;
            if right_capture & pawns != 0 {
                let from_square = right_capture;
                moves.push(Move::en_passant(from_square, to_square, captured_square));
            }

            let left_capture = (captured_square & !FILES[0]) >> 1;
            if left_capture & pawns != 0 {
                let from_square = left_capture;
                moves.push(Move::en_passant(from_square, to_square, captured_square));
            }
        }

        while left_promotion_capture_mask != 0 {
            let to_square = 1u64 << left_promotion_capture_mask.trailing_zeros();
            let from_square = to_square << 7;
            for piece_type in [QUEEN, ROOK, BISHOP, KNIGHT] {
                moves.push(Move::promotion(from_square, to_square, piece_type));
            }
            left_promotion_capture_mask &= left_promotion_capture_mask - 1;
        }

        while right_promotion_capture_mask != 0 {
            let to_square = 1u64 << right_promotion_capture_mask.trailing_zeros();
            let from_square = to_square << 9;
            for piece_type in [QUEEN, ROOK, BISHOP, KNIGHT] {
                moves.push(Move::promotion(from_square, to_square, piece_type));
            }
            right_promotion_capture_mask &= right_promotion_capture_mask - 1;
        }

        while promotion_mask != 0 {
            let to_square = 1u64 << promotion_mask.trailing_zeros();
            let from_square = to_square << 8;
            for piece_type in [QUEEN, ROOK, BISHOP, KNIGHT] {
                moves.push(Move::promotion(from_square, to_square, piece_type));
            }
            promotion_mask &= promotion_mask - 1;
        }
    };
}


pub fn get_knight_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>) {
    let mut knight_mask = chessboard.get_piece_mask(KNIGHT, color);
    let own_pieces = if color == 1 { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };

    while knight_mask != 0 {
        let from = 1u64 << knight_mask.trailing_zeros();

        let mut move_mask = 0u64;

        //from whites perspective (A1 is bit 0)
        //moves up and right (shift 8 + 8 and 1), dont if were on the rightmost file
        move_mask |= (from & !FILES[0])  >> 17;
        //up and left 8 + 8 - 1 and so on
        move_mask |= (from & !FILES[7])  >> 15;
        move_mask |= (from & !(FILES[1] | FILES[0])) >> 10;
        move_mask |= (from & !(FILES[7] | FILES[6])) >> 6;

        //same thing but shift right for the moves down the board
        move_mask |= (from & !FILES[0])  << 15;
        move_mask |= (from & !FILES[7])  << 17;
        move_mask |= (from & !(FILES[1] | FILES[0])) << 6;
        move_mask |= (from & !(FILES[7] | FILES[6])) << 10;

        // remove own pieces
        move_mask &= !own_pieces;

        // convert attacks to move list
        let mut targets = move_mask;
        while targets != 0 {
            let to = 1u64 << targets.trailing_zeros();
            moves.push(Move::new(from, to));
            targets &= targets - 1;
        }
        knight_mask &= knight_mask - 1;
    }
}



pub fn get_bishop_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>, queen_mask: Option<u64>) {
    // if queen_mask is Some, use that as the bishop mask, otherwise use the bishops from the chessboard
    let mut bishop_mask = if let Some(q_mask) = queen_mask {
        queen_mask.unwrap()
    } else if color == 1 {
        chessboard.get_piece_mask(BISHOP, WHITE).clone()
    } else {
        chessboard.get_piece_mask(BISHOP, BLACK).clone()
    };

    let own_pieces = if color == 1 { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };
    let all_pieces = chessboard.get_white_pieces() | chessboard.get_black_pieces();

    let mut move_mask:u64 = 0;

    while bishop_mask != 0 {
        let from = 1u64 << bishop_mask.trailing_zeros();
        if (from & (RANKS[7] | FILES[7])) == 0 {
            for i in 1..8 {
                let dest = from << 9 * i;
                move_mask |= dest &! own_pieces;
                if (dest & ((RANKS[7] | FILES[7]) | all_pieces)) != 0 {
                    break;
                }
            }
        }

        if (from & (RANKS[0] | FILES[7])) == 0 {
            for i in 1..8 {
                let dest = from >> 7 * i;
                move_mask |= dest & !own_pieces;
                if (dest & ((RANKS[0] | FILES[7]) | all_pieces)) != 0 {
                    break;
                }
            }
        }
        if (from & (RANKS[7] | FILES[0])) == 0 {
            for i in 1..8 {
                let dest = from << 7 * i;
                move_mask |= dest & !own_pieces;
                if (dest & ((FILES[0] | RANKS[7]) | all_pieces)) != 0 {
                    break;
                }
            }
        }
        if (from & (RANKS[0] | FILES[0])) == 0 {
            for i in 1..8 {
                let dest = from >> 9 * i;
                move_mask |= dest & !own_pieces;
                if (dest & ((FILES[0] | RANKS[0]) | all_pieces)) != 0 {
                    break;
                }
            }
        }

        move_mask &= !own_pieces;
        while move_mask != 0 {
            let to = 1u64 << move_mask.trailing_zeros();
            moves.push(Move::new(from, to));
            move_mask &= move_mask - 1;
        }
        bishop_mask &= bishop_mask - 1;
    }
}

// same logic as bishop but for straight lines
pub fn get_rook_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>, queen_mask: Option<u64>) {
    let mut rook_mask = if let Some(q_mask) = queen_mask {
        queen_mask.unwrap()
    } else if color == 1 {
        chessboard.get_piece_mask(ROOK, WHITE)
    } else {
        chessboard.get_piece_mask(ROOK, BLACK)
    };

    let own_pieces = if color == 1 { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };
    let all_pieces = chessboard.get_white_pieces() | chessboard.get_black_pieces();
    if color == 1 { chessboard.get_black_pieces() } else { chessboard.get_white_pieces() };


    let mut move_mask:u64 = 0;
    while rook_mask != 0 {
        let from = 1u64 << rook_mask.trailing_zeros();
        if (from & RANKS[7]) == 0 {
            //upwards from white perspective
            for i in 1..8 {
                let dest = from << 8 * i;
                move_mask |= dest &! own_pieces;
                //stop on own pieces and last rank
                if dest & (RANKS[7] | all_pieces) != 0 {
                    break;
                }
            }
        }

        if (from & RANKS[0]) == 0 {
            //down
            for i in 1..8 {
                let dest = from >> 8 * i;
                move_mask |= dest & !own_pieces;
                //stop on own pieces and last rank
                if dest & (RANKS[0] | all_pieces) != 0 {
                    break;
                }
            }
        }
        if (from & FILES[7]) == 0 {
            //left
            for i in 1..8 {
                let dest = from << 1 * i;
                move_mask |= dest & !own_pieces;
                //stop on own pieces and last rank
                if dest & (FILES[7] | all_pieces) != 0 {
                    break;
                }
            }
        }
        if (from & FILES[0]) == 0 {
            for i in 1..8 {
                let dest = from >> 1 * i;
                move_mask |= dest & !own_pieces;
                //stop on own pieces and last rank
                if dest & (FILES[0] | all_pieces) != 0 {
                    break;
                }
            }
        }

        move_mask &= !own_pieces;
        while move_mask != 0 {
            let to = 1u64 << move_mask.trailing_zeros();
            moves.push(Move::new(from, to));
            move_mask &= move_mask - 1;
        }
        rook_mask &= rook_mask - 1;
    }
}

pub fn get_queen_moves(chessboard: &Chessboard, color: i8, moves: &mut Vec<Move>) {
    let queen_mask = chessboard.get_piece_mask(QUEEN, color).clone();
    // conveniently reuse bishop and rook move generation for queen moves
    get_bishop_moves(chessboard, color, moves, Some(queen_mask));
    get_rook_moves(chessboard, color, moves, Some(queen_mask));
}


pub fn get_king_moves(chessboard: &mut Chessboard, color: i8, moves: &mut Vec<Move>) {
    let mut king_mask = chessboard.get_piece_mask(KING, color);

    let own_pieces = if color == WHITE { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };
    let castling_rights = chessboard.get_castling_rights();

    while king_mask != 0 {
        let from = 1u64 << king_mask.trailing_zeros();

        let mut move_mask = 0u64;

        //left
        move_mask |= ((from & !FILES[0]) >> 1)
            | ((from & !(FILES[0] | RANKS[7])) >> 9)
            | ((from & !(FILES[7] | RANKS[7])) >> 7)
            | ((from & !RANKS[0]) >> 8)
            | ((from & !RANKS[7]) << 8);

        //etc
        move_mask |= ((from & !FILES[7]) << 1)
            | ((from & !(FILES[0] | RANKS[7])) << 7)
            | ((from & !(FILES[7] | RANKS[7])) << 9)
            | ((from & !RANKS[0]) >> 8)
            | ((from & !RANKS[7]) << 8);

        move_mask &= !own_pieces;

        while move_mask != 0 {
            let to = 1u64 << move_mask.trailing_zeros();
            chessboard.make_move(Move::new(from, to));
            if !is_square_attacked(chessboard, to, -color) {
                moves.push(Move::new(from, to));
            }
            chessboard.undo_move();
            move_mask &= move_mask - 1;
        }
        king_mask &= king_mask - 1;
    }

    //castling moves
    let occupied = chessboard.get_white_pieces() | chessboard.get_black_pieces();
    if color == WHITE {
        if (castling_rights & WHITE_KINGSIDE_CASTLE) != 0 {
            if (occupied & ((1 << 1) | (1 << 2))) == 0 {
                chessboard.make_move(Move::castling(1 << 3, 1 << 1, 1 << 0, 1 << 2));
                if !is_square_attacked(chessboard, 1 << 2, -color) && !is_square_attacked(chessboard, 1 << 1, -color) {
                    moves.push(Move::castling(1 << 3, 1 << 1, 1 << 0, 1 << 2));
                }
                chessboard.undo_move();
            }
        }
        if (castling_rights & WHITE_QUEENSIDE_CASTLE) != 0 {
            if (occupied & ((1 << 4) | (1 << 5) | (1 << 6))) == 0 {
                chessboard.make_move(Move::castling(1 << 3, 1 << 5, 1 << 7, 1 << 4));
                if !is_square_attacked(chessboard, 1 << 4, -color) && !is_square_attacked(chessboard, 1 << 5, -color) {
                    moves.push(Move::castling(1 << 3, 1 << 5, 1 << 7, 1 << 4));
                }
                chessboard.undo_move();
            }
        }
    } else {
        if (castling_rights & BLACK_KINGSIDE_CASTLE) != 0 {
            if (occupied & ((1 << 57) | (1 << 58))) == 0 {
                chessboard.make_move(Move::castling(1 << 59, 1 << 57, 1 << 56, 1 << 58));
                if !is_square_attacked(chessboard, 1 << 58, -color) && !is_square_attacked(chessboard, 1 << 57, -color) {
                    moves.push(Move::castling(1 << 59, 1 << 57, 1 << 56, 1 << 58));
                }
                chessboard.undo_move();
            }
        }
        if (castling_rights & BLACK_QUEENSIDE_CASTLE) != 0 {
            if (occupied & ((1 << 60) | (1 << 61) | (1 << 62))) == 0 {
                chessboard.make_move(Move::castling(1 << 59, 1 << 61, 1 << 63, 1 << 60));
                if !is_square_attacked(chessboard, 1 << 60, -color) && !is_square_attacked(chessboard, 1 << 61, -color) {
                    moves.push(Move::castling(1 << 59, 1 << 61, 1 << 63, 1 << 60));
                }
                chessboard.undo_move();
            }
        }
    }
}

pub fn is_square_attacked(chessboard: &Chessboard, square_mask: u64, attacker_color: i8) -> bool {
    let mut moves = Vec::new();

    get_pawn_moves(chessboard, attacker_color, &mut moves);
    get_knight_moves(chessboard, attacker_color, &mut moves);
    get_bishop_moves(chessboard, attacker_color, &mut moves, None);
    get_rook_moves(chessboard, attacker_color, &mut moves, None);
    get_queen_moves(chessboard, attacker_color, &mut moves);

    for mv in moves {
        if (mv.get_to_mask() & square_mask) != 0 {
            return true;
        }
    }

    //we cant use the king moves function because it uses this function
    let enemy_king = chessboard.get_piece_mask(KING, attacker_color);
    let mut king_attacks = 0u64;

    //left
    king_attacks |= ((enemy_king &! FILES[0]) >> 1)
        | ((enemy_king &! (FILES[0] | RANKS[7])) >> 9)
        | ((enemy_king &! (FILES[7] | RANKS[7])) >> 7)
        | ((enemy_king &! RANKS[0]) >> 8)
        | ((enemy_king &! RANKS[7]) << 8);

    //ect
    king_attacks |= ((enemy_king &! FILES[7]) << 1)
        | ((enemy_king &! (FILES[0] | RANKS[7])) << 7)
        | ((enemy_king &! (FILES[7] | RANKS[7])) << 9)
        | ((enemy_king &! RANKS[0]) >> 8)
        | ((enemy_king &! RANKS[7]) << 8);

    let attacker_pieces = if attacker_color == WHITE { chessboard.get_white_pieces() } else { chessboard.get_black_pieces() };
    king_attacks &= !attacker_pieces;

    if (king_attacks & square_mask) != 0 {
        return true;
    }

    false
}




