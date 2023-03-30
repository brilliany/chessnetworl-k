package ChessNetwork;

import ChessNetwork.Pieces.*;

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

    public static int getColor(int[] piece) {
        // Returns the color of a piece
        // 1 = white, -1 = black
        return piece[0] > 0 ? WHITE : BLACK;
    }

    public static boolean hasMoved(int[] piece) {
        // Returns whether a piece has moved
        // 0 = has not moved, 1 = has moved
        return piece[1] == HAS_MOVED;
    }

    public static int getPieceType(int[] piece) {
        // Returns the type of a piece
        // 1 = pawn, 2 = knight, 3 = bishop, 4 = rook, 5 = queen, 6 = king
        return Math.abs(piece[0]);
    }

    public static String getPieceTypeAsStr(int[] piece) {
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

    public static Move[] getPieceMoves(int[] piece, int x, int y, int[][][] boardState, MoveGenerator moveGenerator) {
        switch (getPieceType(piece)) {
            case PAWN -> {
                return Pawn.getMoves(boardState, x, y, getColor(piece), moveGenerator);
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

    public static boolean isEmpty(int x, int y, int[][][] boardState) {
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

    public static String boardToString(int[][][] chessboard) {
        // Returns a string representation of the board
        StringBuilder boardStr = new StringBuilder();
        for (int y = 0; y < 8; y++) {
            for (int x = 0; x < 8; x++) {
                boardStr.append(getPieceTypeAsSymbol(chessboard[y][x]));
            }
            boardStr.append("\n");
        }
        return boardStr.toString();
    }
    public static int[][][] makeMoveSilent(Move move, int[][][] chessboard) {
        //handle castling, promotion
        int[][][] newChessboard = new int[8][8][2];
        for (int y = 0; y < 8; y++) {
            for (int x = 0; x < 8; x++) {
                System.arraycopy(chessboard[y][x], 0, newChessboard[y][x], 0, 2);
            }
        }
        int color = ChessboardHelper.getColor(newChessboard[move.getFromY()][move.getFromX()]);
        // The pieces check themselves if the move is valid
        // This is the method that updates the chessboard
        if (move.isCastle()) {
            // coordinates are stored like chessboard[y][x]
            // so the rook is at the same y as the king
            // and the y is either 0 or 7

            int y = move.getToY();

            // short castling
            if (move.getToX() == 6) {
                // the move is valid since the piece checked it
                // so we can just update the chessboard
                setSquare(4, y, EMPTY_SQUARE, HAS_NOT_MOVED, newChessboard);
                setSquare(6, y, new int[]{KING*color}, HAS_MOVED, newChessboard);
                setSquare(7, y, EMPTY_SQUARE, HAS_NOT_MOVED, newChessboard);
                setSquare(5, y, new int[]{ROOK*color}, HAS_MOVED, newChessboard);
            } else {
                // the move is valid since the piece checked it
                // so we can just update the chessboard
                setSquare(4, y, EMPTY_SQUARE, HAS_NOT_MOVED, newChessboard);
                setSquare(2, y, new int[]{KING*color}, HAS_MOVED, newChessboard);
                setSquare(0, y, EMPTY_SQUARE, HAS_NOT_MOVED, newChessboard);
                setSquare(3, y, new int[]{ROOK*color}, HAS_MOVED, newChessboard);
            }

            return newChessboard;
        }
        int pawnDestination = color == WHITE ? 7 : 0;
        //promotion
        if (move.getPiece()[0] == PAWN && move.getToY() == pawnDestination) {
            // the move is valid since the piece checked it
            // so we can just update the chessboard
            setSquare(move.getFromX(), move.getFromY(), EMPTY_SQUARE, HAS_NOT_MOVED, newChessboard);
            setSquare(move.getToX(), move.getToY(), new int[]{QUEEN}, HAS_MOVED, newChessboard);
        }
        // the move is valid since the piece checked it
        // so we can just update the chessboard
        setSquare(move.getFromX(), move.getFromY(), EMPTY_SQUARE, HAS_NOT_MOVED, newChessboard);
        setSquare(move.getToX(), move.getToY(), move.getPiece(), HAS_MOVED, newChessboard);
        return newChessboard;
    }

    public static void setSquare(int x, int y, int[] piece, int hasMoved, int[][][] boardState) {
        boardState[y][x] = new int[]{piece[0], hasMoved};
    }

    public static Move[] addMoves(Move[] allMoves, Move[] moves) {
        Move[] newMoves = new Move[allMoves.length + moves.length];
        System.arraycopy(allMoves, 0, newMoves, 0, allMoves.length);
        System.arraycopy(moves, 0, newMoves, allMoves.length, moves.length);
        return newMoves;
    }

    public static int[][][] copyChessboard(int[][][] makeMoveSilent) {
        int[][][] newChessboard = new int[8][8][2];
        for (int y = 0; y < 8; y++) {
            for (int x = 0; x < 8; x++) {
                System.arraycopy(makeMoveSilent[y][x], 0, newChessboard[y][x], 0, 2);
            }
        }
        return newChessboard;
    }
}
