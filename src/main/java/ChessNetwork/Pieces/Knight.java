package ChessNetwork.Pieces;

import ChessNetwork.Chessboard;

import java.util.ArrayList;

import static ChessNetwork.BoardUtils.KNIGHT;
import static ChessNetwork.BoardUtils.WHITE;

public class Knight {

    public static ArrayList<Move> getMoves(int x, int y, int color, Chessboard moveGenerator) {
        long[] squares = {
                1L << (x + 2 + (y + 1) * 8),
                1L << (x + 2 + (y - 1) * 8),
                1L << (x - 2 + (y + 1) * 8),
                1L << (x - 2 + (y - 1) * 8),
                1L << (x + 1 + (y + 2) * 8),
                1L << (x + 1 + (y - 2) * 8),
                1L << (x - 1 + (y + 2) * 8),
                1L << (x - 1 + (y - 2) * 8)
        };
        ArrayList<Move> moves = new ArrayList<>();
        for (long square : squares) {
            long pieces = color == WHITE ? moveGenerator.getWhitePieces() : moveGenerator.getBlackPieces();
            long opponentPieces = color == WHITE ? moveGenerator.getBlackPieces() : moveGenerator.getWhitePieces();
            if ((pieces & square) == 0) {
                int destX = Long.numberOfTrailingZeros(square) % 8;
                int destY = Long.numberOfTrailingZeros(square) / 8;
                // if the destination square is more than 3 squares away, then the knight is at the edge of the board and
                // the bitboard will have handled it as a move at the other side of the board
                if (Math.abs(destX - x) > 3 || Math.abs(destY - y) > 3) {
                    continue;
                }

                moves.add(new Move(x, y, destX, destY, KNIGHT*color));
            }
        }

        return moves;
    }
}