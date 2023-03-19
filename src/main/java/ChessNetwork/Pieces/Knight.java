package ChessNetwork.Pieces;

import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.getColor;

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
        //check that the starting square is right
        if (move.getFromX() != x || move.getFromY() != y) {
            return false;
        }
        if (move.getToX() == x && move.getToY() == y) {
            return false;
        }
        //if the move is not in L shape
        if (Math.abs(move.getToX() - x) + Math.abs(move.getToY() - y) != 3) {
            return false;
        }
        if (chessboard[move.getToY()][move.getToX()] != null) {
            return getColor(chessboard[move.getToY()][move.getToX()]) != color;
        }
        return true;
    }
}