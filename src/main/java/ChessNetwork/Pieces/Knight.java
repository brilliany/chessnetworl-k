package ChessNetwork.Pieces;

import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.getColor;
import static ChessNetwork.ChessboardHelper.isEmpty;

public class Knight {

    public static Move[] getMoves(int[][][] chessboard, int x, int y, int color,MoveGenerator moveGenerator) {
        int[] thisPiece = chessboard[y][x];
        // Knight can move in L shape
        ArrayList<Move> moves = new ArrayList<>();
        // Move up and right
        Move move = new Move(x, y, x + 1, y - 2, thisPiece);
        if (isValidMove(move, chessboard, x, y,color)) {
            moves.add(move);
        }
        // Move up and left
        move = new Move(x, y, x - 1, y - 2, thisPiece);
        if (isValidMove(move, chessboard, x, y,color)) {
            moves.add(move);
        }
        // Move down and right
        move = new Move(x, y, x + 1, y + 2, thisPiece);
        if (isValidMove(move, chessboard, x, y,color)) {
            moves.add(move);
        }
        // Move down and left
        move = new Move(x, y, x - 1, y + 2, thisPiece);
        if (isValidMove(move, chessboard, x, y,color)) {
            moves.add(move);
        }
        // Move right and up
        move = new Move(x, y, x + 2, y - 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color)) {
            moves.add(move);
        }
        // Move right and down
        move = new Move(x, y, x + 2, y + 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color)) {
            moves.add(move);
        }
        // Move left and up
        move = new Move(x, y, x - 2, y - 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color)) {
            moves.add(move);
        }
        // Move left and down
        move = new Move(x, y, x - 2, y + 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color)) {
            moves.add(move);
        }
            if (moveGenerator != null) {
                moves.removeIf(move1 -> moveGenerator.putsKingInCheck(move1, chessboard));
            }
        return moves.toArray(new Move[0]);

    }

    public static boolean isValidMove(Move move, int[][][] chessboard, int x, int y, int color) {
        //The knight can jump over other pieces
        if (move.getToX() < 0 || move.getToX() > 7 || move.getToY() < 0 || move.getToY() > 7) {
            return false;
        }
        //check starting square
        if (move.getFromX() != x || move.getFromY() != y) {
            return false;
        }
        //check if the move is to one of the 8 possible squares (or less if the move is out of bounds)
        if (move.getToX() != x + 1 && move.getToX() != x - 1 && move.getToX() != x + 2 && move.getToX() != x - 2) {
            return false;
        }
        if (move.getToY() != y + 2 && move.getToY() != y - 2 && move.getToY() != y + 1 && move.getToY() != y - 1) {
            return false;
        }
        //check if the move is to an empty square
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