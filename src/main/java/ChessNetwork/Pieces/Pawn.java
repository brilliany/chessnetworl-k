package ChessNetwork.Pieces;

import ChessNetwork.ChessboardHelper;
import ChessNetwork.MoveGenerator;
import org.jetbrains.annotations.Nullable;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.*;

public class Pawn {

    boolean canBeEnPassanted = false;


    public static Move[] getMoves(int[][][] chessboard, int x, int y, int color, MoveGenerator moveGenerator, @Nullable int[] pawnWhichIsEnPassantable) {
        int[] thisPiece = chessboard[y][x];
        //if the pawn is at the end of the board it cant move
        int direction = color == WHITE ? -1 : 1;
        if (y == 0 || y == 7) {
            return new Move[0];
        }

        ArrayList<Move> moves = new ArrayList<>();
        // Move up
        Move move = new Move(x, y, x, y + direction, thisPiece);
        if (isValidMove(move, chessboard, x, y, color)) {
            moves.add(move);
        }
        // Move up 2
        move = new Move(x, y, x, y + direction * 2, thisPiece);

        if (isValidMove(move, chessboard, x,y,color)) {
            moves.add(move);
        }
        // Move up right
        move = new Move(x, y, x + 1, y + direction, thisPiece);

        if (isValidCapture(move, chessboard, x,y,color)) {
            moves.add(move);
        }

        // Move up left
        move = new Move(x, y, x - 1, y + direction, thisPiece);

        if (isValidCapture(move, chessboard, x,y,color)) {
            moves.add(move);
        }

        // En passant
        if (pawnWhichIsEnPassantable != null) {
            move = new Move(x, y, x + 1, y + direction, thisPiece);
            if (canEnPassant(move, chessboard, x, y, color, pawnWhichIsEnPassantable)) {
                moves.add(move);
            }
            move = new Move(x, y, x - 1, y + direction, thisPiece);

            if (canEnPassant(move, chessboard, x, y, color, pawnWhichIsEnPassantable)) {
                moves.add(move);
            }
        }
        if (moveGenerator != null) {
            moves.removeIf(move1 -> moveGenerator.putsKingInCheck(move1, chessboard));
        }

        return moves.toArray(new Move[0]);
    }

    public static boolean isValidMove(Move move, int[][][] chessboard, int x, int y, int color) {
        //check out of bounds
        if (move.getToX() < 0 || move.getToX() > 7 || move.getToY() < 0 || move.getToY() > 7) {
            return false;
        }
        //check that the starting square is right
        if (move.getFromX() != x || move.getFromY() != y) {
            return false;
        }
        //Check that the move isnt the same as the starting square
        int direction = color == WHITE ? -1 : 1;
        if (move.getToX() == x && move.getToY() == y) {
            return false;
        }
        String moveType;
        //check if the move is a single or a double push
        if (move.getToY() == y + direction && move.getToX() == x) {
            moveType = "single";
        } else if (move.getToY() == y + direction * 2 && move.getToX() == x) {
            moveType = "double";
        } else {
            return false;
        }
        //check if the move is blocked
        if (moveType.equals("double")) {
            if (ChessboardHelper.isEmpty(x, y + direction, chessboard)) {
                boolean hasMoved = hasMoved(chessboard[y][x]);
                if (hasMoved) {
                    return false;
                }
                return ChessboardHelper.isEmpty(x, y + direction * 2, chessboard);
            }
        } else {
            return isEmpty(x, y + direction, chessboard);
        }
        return false;
    }
    public static boolean isValidCapture(Move move, int[][][] chessboard, int x, int y, int color) {
        int direction = color == WHITE ? -1 : 1;
        //check out of bounds
        if (move.getToY() < 0 || move.getToY() > 7 || move.getToX() < 0 || move.getToX() > 7) {
            return false;
        }

        //check if there is a piece to capture
        if (move.getToY() == y + direction) {
            if (move.getToX() == x + 1 || move.getToX() == x - 1) {
                if (!isEmpty(move.getToX(), move.getToY(), chessboard)) {
                    return getColor(chessboard[move.getToY()][move.getToX()]) != color;
                }
            }
        }
        return false;
    }
    public static boolean canEnPassant(Move move, int[][][] chessboard, int x, int y, int color, int[] pawnWhichIsEnPassantable) {
        int direction = color == WHITE ? -1 : 1;
        int deltaX = move.getToX() - x;
        //check out of bounds
        if (move.getToY() < 0 || move.getToY() > 7 || move.getToX() < 0 || move.getToX() > 7) {
            return false;
        }

        boolean empty = isEmpty(move.getToX(), move.getToY(), chessboard);
        boolean notBlocked = getColor(chessboard[move.getToY()][move.getToX()]) != color;
        boolean isEnPassant = move.getToY() == y + direction && move.getToX() == x + deltaX;
        boolean isEnPassantable = pawnWhichIsEnPassantable != null && pawnWhichIsEnPassantable[0] == move.getToX() && pawnWhichIsEnPassantable[1] == move.getToY();
        return empty && notBlocked && isEnPassant && isEnPassantable;
    }

}
