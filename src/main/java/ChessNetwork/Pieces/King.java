package ChessNetwork.Pieces;

import ChessNetwork.BoardUtils;
import ChessNetwork.Chessboard;
import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.BoardUtils.*;


public class King {

    public static void getMoves(int x, int y, int color, Chessboard moveGenerator, ArrayList<Move> moves) {

        long whitePieces = moveGenerator.getWhitePieces();
        long blackPieces = moveGenerator.getBlackPieces();

        long[] squares = new long[]{
                (1L << (x + 1)) << (8 * (y - 1)), // up right
                (1L << (x + 1)) << (8 * y), // right
                (1L << (x + 1)) << (8 * (y + 1)), // down right
                (1L << x) << (8 * (y + 1)), // down
                (1L << (x - 1)) << (8 * (y + 1)), // down left
                (1L << (x - 1)) << (8 * y), // left
                (1L << (x - 1)) << (8 * (y - 1)), // up left
                (1L << x) << (8 * (y - 1)) // up
        };


        for (long square : squares) {
            int squareX = Long.numberOfTrailingZeros(square) % 8;
            int squareY = Long.numberOfTrailingZeros(square) / 8;
            //if square is within 1 square of king (squares flip to the other side of the board when king is on the edge)
            int deltaX = Math.abs(x - squareX);
            int deltaY = Math.abs(y - squareY);
            if (deltaX > 1 || deltaY > 1) {
                continue;
            }

            if (color == WHITE) {
                if ((whitePieces & square) == 0) {
                    moves.add(new Move(x, y, squareX, squareY, KING*color));
                }
            } else {
                if ((blackPieces & square) == 0) {
                    moves.add(new Move(x, y, squareX, squareY, KING*color));
                }
            }
        }

        // castling
        handleCastling(moves, x, y, color, moveGenerator);

    }

    private static void handleCastling(ArrayList<Move> moves, int x, int y, int color, Chessboard chessboard) {
        if (MoveGenerator.isCheck(color, chessboard)){
            return;
        }
        boolean rightsShort = chessboard.getCastleRights(color, 1);
        boolean rightsLong = chessboard.getCastleRights(color, 0);

        long whitePieces = chessboard.getWhitePieces();
        long blackPieces = chessboard.getBlackPieces();
        if (color == WHITE) {
            if (rightsShort) {
                //if the squares between the king and the rook are empty
                if ((whitePieces & (1L << 61)) == 0 && (whitePieces & (1L << 62)) == 0) {
                    //if the squares are not attacked by the opponent
                    int firstSquareX = 5;
                    int secondSquareX = 6;
                    if (!BoardUtils.isAttacked(firstSquareX, 0, -color, chessboard) && !BoardUtils.isAttacked(secondSquareX, 0, -color, chessboard)) {
                        moves.add(new Move(x, y, 6, y, KING*color));
                    }
                }
            }
            if (rightsLong) {
                //if the squares between the king and the rook are empty
                if ((whitePieces & (1L << 57)) == 0 && (whitePieces & (1L << 58)) == 0 && (whitePieces & (1L << 59)) == 0) {
                    //if the squares are not attacked by the opponent
                    int firstSquareX = 3;
                    int secondSquareX = 2;
                    int thirdSquareX = 1;
                    if (!BoardUtils.isAttacked(firstSquareX, 0, -color,chessboard) && !BoardUtils.isAttacked(secondSquareX, 0, -color, chessboard) && !BoardUtils.isAttacked(thirdSquareX, 0, -color, chessboard)) {
                        moves.add(new Move(x, y, 2, y, KING*color));
                    }
                }
            }
        }
        if (color == BLACK) {
            if (rightsShort) {
                //if the squares between the king and the rook are empty
                if ((blackPieces & (1L << 5)) == 0 && (blackPieces & (1L << 6)) == 0) {
                    //if the squares are not attacked by the opponent
                    int firstSquareX = 5;
                    int secondSquareX = 6;
                    if (!BoardUtils.isAttacked(firstSquareX, 7, -color, chessboard) && !BoardUtils.isAttacked(secondSquareX, 7, -color, chessboard)) {
                        moves.add(new Move(x, y, 6, y, KING*color));
                    }
                }
            }
            if (rightsLong) {
                //if the squares between the king and the rook are empty
                if ((blackPieces & (1L << 1)) == 0 && (blackPieces & (1L << 2)) == 0 && (blackPieces & (1L << 3)) == 0) {
                    //if the squares are not attacked by the opponent
                    int firstSquareX = 3;
                    int secondSquareX = 2;
                    int thirdSquareX = 1;
                    if (!BoardUtils.isAttacked(firstSquareX, 7, -color, chessboard) && !BoardUtils.isAttacked(secondSquareX, 7, -color, chessboard) && !BoardUtils.isAttacked(thirdSquareX, 7, -color, chessboard)) {
                        moves.add(new Move(x, y, 2, y, KING*color));
                    }
                }
            }
        }
    }
}