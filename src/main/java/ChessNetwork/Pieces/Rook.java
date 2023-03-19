package ChessNetwork.Pieces;

import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.getColor;
import static ChessNetwork.ChessboardHelper.isEmpty;

public class Rook {

    public static Move[] getMoves(int[][][] chessboard, int x, int y, int color, MoveGenerator moveGenerator) {
        int[] thisPiece = chessboard[y][x];
        ArrayList<Move> moves = new ArrayList<>();
        // Move up
        for (int i = y - 1; i >= 0; i--) {
            Move move = new Move(x, y, x, i, thisPiece);
            if (isValidMove(move, chessboard, x, y,color)) {
                moves.add(move);
            } else {
                break;
            }
        }
        // Move down
        for (int i = y + 1; i < 8; i++) {
            Move move = new Move(x, y, x, i, thisPiece);
            if (isValidMove(move, chessboard, x, y,color)) {
                moves.add(move);
            } else {
                break;
            }
        }
        // Move right
        for (int i = x + 1; i < 8; i++) {
            Move move = new Move(x, y, i, y, thisPiece);
            if (isValidMove(move, chessboard, x, y,color)) {
                moves.add(move);
            } else {
                break;
            }
        }
        // Move left
        for (int i = x - 1; i >= 0; i--) {
            Move move = new Move(x, y, i, y, thisPiece);
            if (isValidMove(move, chessboard, x, y,color)) {
                moves.add(move);
            } else {
                break;
            }
        }
        if (moveGenerator != null) {
            moves.removeIf(move1 -> moveGenerator.putsKingInCheck(move1, chessboard));
        }
        return moves.toArray(new Move[0]);
    }

    public static boolean isValidMove(Move move, int[][][] chessboard, int x, int y, int color) {
        if (move.getToX() < 0 || move.getToX() > 7 || move.getToY() < 0 || move.getToY() > 7) {
            return false;
        }
        if (move.getToX() == x && move.getToY() == y) {
            return false;
        }
        //check that the move is straight forward or backward ect
        if (move.getToX() != x && move.getToY() != y) {
            return false;
        }
        //Check if there are any pieces between the current position and the destination
        if (move.getToX() == x) {
            // Move up
            if (move.getToY() < y) {
                for (int i = y - 1; i > move.getToY(); i--) {
                    if (!isEmpty(x, i, chessboard)) {
                        return false;
                    }
                }
            }
            // Move down
            if (move.getToY() > y) {
                for (int i = y + 1; i < move.getToY(); i++) {
                    if (!isEmpty(x, i, chessboard)) {
                        return false;
                    }
                }
            }
        }
        if (move.getToY() == y) {
            // Move right
            if (move.getToX() > x) {
                for (int i = x + 1; i < move.getToX(); i++) {
                    if (!isEmpty(i, y, chessboard)) {
                        return false;
                    }
                }
            }
            // Move left
            if (move.getToX() < x) {
                for (int i = x - 1; i > move.getToX(); i--) {
                    if (!isEmpty(i, y, chessboard)) {
                        return false;
                    }
                }
            }
        }
        if (!isEmpty(move.getToX(), move.getToY(), chessboard)) {
            return getColor(chessboard[move.getToY()][move.getToX()]) != color;
        }
        return true;
    }
}
