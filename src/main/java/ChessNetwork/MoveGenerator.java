package ChessNetwork;


import ChessNetwork.Pieces.*;

import java.util.ArrayList;
import java.util.List;

import static ChessNetwork.BoardUtils.*;

public class MoveGenerator {


    public static List<Move> getAllMoves(int color, Chessboard chessboard) {
        ArrayList<Move> allMoves = new ArrayList<>();

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
            // Get moves for the current piece type
            if ((pawns & piece) != 0) {
                Pawn.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard, allMoves);
            } else if ((knights & piece) != 0) {
                Knight.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard, allMoves);
            } else if ((bishops & piece) != 0) {
                Bishop.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard, allMoves);
            } else if ((rooks & piece) != 0) {
                Rook.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard, allMoves);
            } else if ((queens & piece) != 0) {
                Queen.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard, allMoves);
            } else if ((kings & piece) != 0) {
                King.getMoves(fromSquare % 8, fromSquare / 8, color, chessboard, allMoves);
            }
            // if puts king in check, remove move

        }
        allMoves.removeIf(move -> MoveGenerator.putsKingInCheck(move, chessboard));
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

        return BoardUtils.isAttacked(kingX, kingY, color, chessboard);

    }

}



