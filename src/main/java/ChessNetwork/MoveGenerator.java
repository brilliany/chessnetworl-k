package ChessNetwork;


import ChessNetwork.Pieces.*;

import java.util.ArrayList;
import java.util.List;

import static ChessNetwork.BoardUtils.*;

public class MoveGenerator {


    public static List<Move> getAllMoves(int color, Chessboard chessboard) {
        List<Move> allMoves = new ArrayList<>();

        long pieces = color == WHITE ? chessboard.getWhitePieces() : chessboard.getBlackPieces();
        long pawns = color == WHITE ? chessboard.getWhitePawns() : chessboard.getBlackPawns();
        long knights = color == WHITE ? chessboard.getWhiteKnights() : chessboard.getBlackKnights();
        long bishops = color == WHITE ? chessboard.getWhiteBishops() : chessboard.getBlackBishops();
        long rooks = color == WHITE ? chessboard.getWhiteRooks() : chessboard.getBlackRooks();
        long queens = color == WHITE ? chessboard.getWhiteQueens() : chessboard.getBlackQueens();
        long kings = color == WHITE ? chessboard.getWhiteKings() : chessboard.getBlackKings();

        // Loop over all pieces of the given color
        long piece;
        while (pieces != 0) {
            piece = Long.highestOneBit(pieces);
            pieces ^= piece;

            int fromSquare = Long.numberOfTrailingZeros(piece);
            ArrayList<Move> moves = null;
            // Get moves for the current piece type
            if ((pawns & piece) != 0) {
                moves = Pawn.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard);
            } else if ((knights & piece) != 0) {
                moves = Knight.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard);
            } else if ((bishops & piece) != 0) {
                moves = Bishop.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard);
            } else if ((rooks & piece) != 0) {
                moves = Rook.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard);
            } else if ((queens & piece) != 0) {
                moves = Queen.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard);
            } else if ((kings & piece) != 0) {
                moves = King.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard);
            }
            if (moves != null) {

                // if puts king in check, remove move
                if (moves.size() != 0) {
                    moves.removeIf(move -> MoveGenerator.putsKingInCheck(move, chessboard));
                }
                // Add the moves for the current piece to the list of all moves

                allMoves.addAll(moves);
            }
        }
        return allMoves;
    }

    public static boolean putsKingInCheck(Move move, Chessboard chessboard) {
        // Make the move
        chessboard.silentMove(move);
        // Check if the move puts the own king in check
        boolean inCheck = isCheck(move.getPiece() > 0 ? WHITE : BLACK, chessboard);
        // Restore the previous state
        chessboard.restorePrevious();
        return inCheck;
    }

    public static boolean isCheckmate(int color, Chessboard chessboard) {
        // Check if the given color is in check
        if (isCheck(color, chessboard)) {
            List<Move> allMoves = getAllMoves(color, chessboard);
            for (Move move : allMoves) {
                if (!putsKingInCheck(move, chessboard)) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }

    public static boolean isStalemate(int color, Chessboard chessboard) {
        // Check if the given color is not in check
        if (!isCheck(color, chessboard)) {
            List<Move> allMoves = getAllMoves(color, chessboard);
            for (Move move : allMoves) {
                if (!putsKingInCheck(move, chessboard)) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }


    public static boolean isCheck(int color, Chessboard chessboard) {
        // Check if the given color is in check
        //get the position of the king
        long king = color == WHITE ? chessboard.getWhiteKings() : chessboard.getBlackKings();

        //check if the king is attacked
        int kingX = Long.numberOfTrailingZeros(king) % 8;
        int kingY = Long.numberOfTrailingZeros(king) / 8;

        return BoardUtils.isAttacked(kingX, kingY, -color, chessboard);

    }

}



