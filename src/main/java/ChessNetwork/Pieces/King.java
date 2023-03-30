package ChessNetwork.Pieces;

import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.*;


public class King {

    public static Move[] getMoves(int[][][] chessboard, int x, int y, int color,MoveGenerator moveGenerator) {
        int[] thisPiece = chessboard[y][x];
        ArrayList<Move> moves = new ArrayList<>();
        // Move up
        Move move = new Move(x, y, x, y - 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
            moves.add(move);
        }
        // Move down
        move = new Move(x, y, x, y + 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
            moves.add(move);
        }
        // Move right
        move = new Move(x, y, x + 1, y, thisPiece);
        if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
            moves.add(move);
        }
        // Move left
        move = new Move(x, y, x - 1, y, thisPiece);
        if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
            moves.add(move);
        }
        // Move up right
        move = new Move(x, y, x + 1, y - 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
            moves.add(move);
        }
        // Move up left
        move = new Move(x, y, x - 1, y - 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
            moves.add(move);
        }
        // Move down right
        move = new Move(x, y, x + 1, y + 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
            moves.add(move);
        }
        // Move down left
        move = new Move(x, y, x - 1, y + 1, thisPiece);
        if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
            moves.add(move);
        }
        // Castling

            // Short castling
            move = new Move(x, y, x + 2, y, thisPiece);
            if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
                move.setCastle(true);
                moves.add(move);
            }
            // Long castling
            move = new Move(x, y, x - 2, y, thisPiece);
            if (isValidMove(move, chessboard, x, y,color, moveGenerator)) {
                move.setCastle(true);
                moves.add(move);
            }


            if (moveGenerator != null) {
                moves.removeIf(move1 -> moveGenerator.putsKingInCheck(move1, chessboard));
            }
        return moves.toArray(new Move[0]);
    }

    public static boolean isValidMove(Move move, int[][][] chessboard, int x, int y, int color, MoveGenerator moveGenerator) {
        //check out of bounds
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
        if (Math.abs(move.getToX() - x) > 1 || Math.abs(move.getToY() - y) > 1) {
            return isValidCastle(move, chessboard, x, y, color, moveGenerator);
        }
        if (getColor(chessboard[move.getToY()][move.getToX()]) == color) {
            return false;
        }
        if (!isEmpty(move.getToX(), move.getToY(), chessboard)) {
            if (getColor(chessboard[move.getToY()][move.getToX()]) != color) {
                move.setCapture(true);
            }
        }
        return true;
    }

    private static boolean isValidCastle(Move move, int[][][] chessboard, int x, int y, int color, MoveGenerator moveGenerator) {
        if (move.getToX() == 6) {
            // Castling to the right
            if (move.getToY() != y) {
                return false;
            }
            //if not out of bounds
            if(x + 2 > 7){
                return false;
            }
            if (hasMoved(chessboard[y][x])) {
                return false;
            }
            if (!isEmpty(x + 1, y, chessboard) || !isEmpty(x + 2, y, chessboard)) {
                return false;
            }
            if (isInCheck(chessboard, x, y, color, moveGenerator)) {
                return false;
            }
            if (isInCheck(chessboard, x + 1, y, color, moveGenerator)) {
                return false;
            }
            return !isInCheck(chessboard, x + 2, y, color, moveGenerator);
        } else if (move.getToX() == 3) {
            // Castling to the left
            if (move.getToY() != y) {
                return false;
            }
            //if not out of bounds
            if (x - 3 < 0) {
                return false;
            }
            if (hasMoved(chessboard[y][x])) {
                return false;
            }
            if (!isEmpty(x - 1, y, chessboard) || !isEmpty(x - 2, y, chessboard) || !isEmpty(x - 3, y, chessboard)) {
                return false;
            }
            if (isInCheck(chessboard, x, y, color, moveGenerator)) {
                return false;
            }
            if (isInCheck(chessboard, x - 1, y, color, moveGenerator)) {
                return false;
            }
            return !isInCheck(chessboard, x - 2, y, color, moveGenerator);
        }
        return false;
    }

    private static boolean isInCheck(int[][][] chessboard, int x, int y, int color, MoveGenerator moveGenerator) {
        // Check if the king is in check
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                //in this case i is the y and j is the x
                if (!isEmpty(j, i, chessboard)) {
                    if (getColor(chessboard[i][j]) != color) {
                        int[] piece = chessboard[i][j];
                        int pieceType = getPieceType(piece);
                        if (pieceType == KING) {
                            //skip the king
                            continue;
                        }
                        Move move = new Move(j, i, x, y, piece);
                        //for each piece call the static isValidMove method in their respective classes
                        switch (pieceType) {
                            case PAWN -> {
                                if (Pawn.isValidCapture(move, chessboard, j, i, color)) {
                                    return true;
                                }
                            }
                            case KNIGHT -> {
                                if (Knight.isValidMove(move, chessboard, j, i, color)) {
                                    return true;
                                }
                            }
                            case BISHOP -> {
                                if (Bishop.isValidMove(move, chessboard, j, i, color)) {
                                    return true;
                                }
                            }
                            case ROOK -> {
                                if (Rook.isValidMove(move, chessboard, j, i, color)) {
                                    return true;
                                }
                            }
                            case QUEEN -> {
                                if (Queen.isValidMove(move, chessboard, j, i, color)) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        return false;
    }
}