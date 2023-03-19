package ChessNetwork;

import ChessNetwork.Pieces.*;
import org.jetbrains.annotations.Nullable;

public class ChessboardHelper {
    /*
      This class is used to help with the inevitable pain of dealing with the chessboard array.
      All methods are static, so you don't need to instantiate an object to use them.
     */
    public static final int PAWN = 1;
    public static final int KNIGHT = 2;
    public static final int BISHOP = 3;
    public static final int ROOK = 4;
    public static final int QUEEN = 5;
    public static final int KING = 6;
    public static final int EMPTY = 0;
    public static final int WHITE = 1;
    public static final int BLACK = -1;
    public static final int HAS_MOVED = 1;
    public static final int HAS_NOT_MOVED = 0;
    public static final int[] EMPTY_SQUARE = {EMPTY, HAS_NOT_MOVED};

    public static int getColor(int[] piece){
        // Returns the color of a piece
        // 1 = white, -1 = black
        return piece[0] > 0 ? WHITE : BLACK;
    }
    public static boolean hasMoved(int[] piece){
        // Returns whether a piece has moved
        // 0 = has not moved, 1 = has moved
        return piece[1] == HAS_MOVED;
    }
    public static int getPieceType(int[] piece){
        // Returns the type of a piece
        // 1 = pawn, 2 = knight, 3 = bishop, 4 = rook, 5 = queen, 6 = king
        return Math.abs(piece[0]);
    }
    public static String getPieceTypeAsStr(int[] piece){
        // Returns the type of a piece as a string
        // 1 = pawn, 2 = knight, 3 = bishop, 4 = rook, 5 = queen, 6 = king

        int pieceType = getPieceType(piece);
        return switch (pieceType) {
            case PAWN -> "Pawn";
            case KNIGHT -> "Knight";
            case BISHOP -> "Bishop";
            case ROOK -> "Rook";
            case QUEEN -> "Queen";
            case KING -> "King";
            default -> "Empty";
        };
    }
    public static Move[] getPieceMoves(int[] piece, int x, int y, int[][][] boardState, MoveGenerator moveGenerator, @Nullable int[] pawnWhichIsEnPassantable){
        switch (getPieceType(piece)){
            case PAWN -> {
                return Pawn.getMoves(boardState, x, y, getColor(piece), moveGenerator, pawnWhichIsEnPassantable);
            }
            case KNIGHT -> {
                return Knight.getMoves(boardState, x, y, getColor(piece), moveGenerator);
            }
            case BISHOP -> {
                return Bishop.getMoves(boardState, x, y, getColor(piece), moveGenerator);
            }
            case ROOK -> {
                return Rook.getMoves(boardState, x, y, getColor(piece), moveGenerator);
            }
            case QUEEN -> {
                return Queen.getMoves(boardState, x, y, getColor(piece), moveGenerator);
            }
            case KING -> {
                return King.getMoves(boardState, x, y, getColor(piece), moveGenerator);
            }
            default -> {
                return new Move[0];
            }
        }

    }

    public static boolean isEmpty(int x, int y, int[][][] boardState){
        return boardState[y][x][0] == EMPTY;
    }

    public static String getPieceTypeAsSymbol(int[] piece) {
        //return unicode symbol for piece
        return switch (getPieceType(piece)) {
            case PAWN -> getColor(piece) == WHITE ? "♙" : "♟";
            case KNIGHT -> getColor(piece) == WHITE ? "♘" : "♞";
            case BISHOP -> getColor(piece) == WHITE ? "♗" : "♝";
            case ROOK -> getColor(piece) == WHITE ? "♖" : "♜";
            case QUEEN -> getColor(piece) == WHITE ? "♕" : "♛";
            case KING -> getColor(piece) == WHITE ? "♔" : "♚";
            default -> " ";
        };
    }

    public static int[][] getKings(int[][][] chessboard) {
        // Returns the kings of the board
        int[] whiteKing = null;
        int[] blackKing = null;
        for (int x = 0; x < 8; x++) {
            for (int y = 0; y < 8; y++) {
                if (getPieceType(chessboard[y][x]) == KING) {
                    if (getColor(chessboard[y][x]) == WHITE) {
                        if (hasMoved(chessboard[x][y])) {
                            whiteKing = chessboard[7][4] = new int[]{WHITE * KING, HAS_MOVED};
                        } else {
                            whiteKing = chessboard[7][4] = new int[]{WHITE * KING, HAS_NOT_MOVED};
                        }
                    } else {
                        if (hasMoved(chessboard[x][y])) {
                            blackKing = chessboard[0][4] = new int[]{BLACK * KING, HAS_MOVED};
                        } else {
                            blackKing = chessboard[0][4] = new int[]{BLACK * KING, HAS_NOT_MOVED};
                        }
                    }
                }
            }
        }
        return new int[][]{whiteKing, blackKing};
    }
}
