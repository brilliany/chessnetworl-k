package ChessNetwork.Pieces;


import ChessNetwork.ChessboardHelper;
import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.getColor;
import static ChessNetwork.ChessboardHelper.isEmpty;

public class Bishop {


    public static Move[] getMoves(int[][][] chessboard, int x, int y, int color,MoveGenerator moveGenerator) {
        int[] thisPiece = chessboard[y][x];
        // The coordinates start from the top left corner (y = 0, x = 0), y is the vertical axis and x is the horizontal axis
        ArrayList<Move> moves = new ArrayList<>();
        // Move up and right
        for (int i = x + 1, j = y - 1; i < 8 && j >= 0; i++, j--) {
            Move move = new Move(x, y, i, j, thisPiece);
            if (isValidMove(move, chessboard, x, y,color)) {
                moves.add(move);
            } else {
                break;
            }
        }
        // Move up and left
        for (int i = x - 1, j = y - 1; i >= 0 && j >= 0; i--, j--) {
            Move move = new Move(x, y, i, j, thisPiece);
            if (isValidMove(move, chessboard, x, y,color)) {
                moves.add(move);
            } else {
                break;
            }
        }
        // Move down and right
        for (int i = x + 1, j = y + 1; i < 8 && j < 8; i++, j++) {
            Move move = new Move(x, y, i, j, thisPiece);
            if (isValidMove(move, chessboard, x, y,color)) {
                moves.add(move);
            } else {
                break;
            }
        }
        // Move down and left
        for (int i = x - 1, j = y + 1; i >= 0 && j < 8; i--, j++) {
            Move move = new Move(x, y, i, j, thisPiece);
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
        // Check if move is out of bounds
        if (move.getToX() < 0 || move.getToX() > 7 || move.getToY() < 0 || move.getToY() > 7) {

            return false;
        }
        //check that the starting square is right
        if (move.getFromX() != x || move.getFromY() != y) {

            return false;
        }

        // Check if move is to the same square
        if (move.getToX() == x && move.getToY() == y) {

            return false;
        }
        // Check if move is diagonal
        if (Math.abs(move.getToX() - x) != Math.abs(move.getToY() - y)) {

            return false;
        }


        // Check if there are any pieces in the way
        // Check up and right
        if (move.getToX() > x && move.getToY() < y) {
            for (int i = x + 1, j = y - 1; i < move.getToX() && j > move.getToY(); i++, j--) {
                if (!ChessboardHelper.isEmpty(i, j, chessboard)) {

                    return false;
                }
            }
        }
        // Check up and left
        if (move.getToX() < x && move.getToY() < y) {
            for (int i = x - 1, j = y - 1; i > move.getToX() && j > move.getToY(); i--, j--) {
                if (!ChessboardHelper.isEmpty(i, j, chessboard)) {

                    return false;
                }
            }
        }
        // Check down and right
        if (move.getToX() > x && move.getToY() > y) {
            for (int i = x + 1, j = y + 1; i < move.getToX() && j < move.getToY(); i++, j++) {
                if (!ChessboardHelper.isEmpty(i, j, chessboard)) {

                    return false;
                }
            }
        }
        // Check down and left
        if (move.getToX() < x && move.getToY() > y) {
            for (int i = x - 1, j = y + 1; i > move.getToX() && j < move.getToY(); i--, j++) {
                if (!ChessboardHelper.isEmpty(i, j, chessboard)) {

                    return false;
                }
            }
        }

        //if get to is a piece of the same color, return false
        if (!isEmpty(move.getToX(), move.getToY(), chessboard)) {
            if (getColor(chessboard[move.getToY()][move.getToX()]) != color) {
                move.setCapture(true);
            } else {
                return false;
            }
        }
        return true;
    }
}
