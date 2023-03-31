package ChessNetwork;

import ChessNetwork.Pieces.*;

import java.util.ArrayList;

import static java.lang.Math.abs;

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

    public static int getColor(int piece) {
        // Returns the color of a piece
        // 1 = white, -1 = black
        return piece > 0 ? WHITE : BLACK;
    }

    public static int getPieceType(int piece) {
        // Returns the type of a piece
        // 1 = pawn, 2 = knight, 3 = bishop, 4 = rook, 5 = queen, 6 = king
        return abs(piece);
    }

    public static String getPieceTypeAsStr(int piece) {
        // Returns the type of a piece as a string
        // 1 = pawn, 2 = knight, 3 = bishop, 4 = rook, 5 = queen, 6 = king
        String colorStr = getColor(piece) == WHITE ? "White" : "Black";
        int pieceType = getPieceType(piece);
        return switch (abs(pieceType)) {
            case PAWN -> colorStr + "_Pawn";
            case KNIGHT -> colorStr + "_Knight";
            case BISHOP -> colorStr + "_Bishop";
            case ROOK -> colorStr + "_Rook";
            case QUEEN -> colorStr + "_Queen";
            case KING -> colorStr + "_King";
            default -> "Empty";
        };
    }
    public static void addMovesInDirection(ArrayList<Move> moves, long blackPieces, long whitePieces, int color, int x, int y, int deltaX, int deltaY) {
        long direction = 0;
        for (int i = 1; i < 8; i++) {
            int newX = x + (i * deltaX);
            int newY = y + (i * deltaY);
            if (newX < 0 || newX >= 8 || newY < 0 || newY >= 8) {
                break; // Out of board
            }
            direction |= 1L << (newY * 8 + newX);
            if ((blackPieces & direction) != 0) {
                if ((whitePieces & direction) == 0) {
                    moves.add(new Move(x, y, newX, newY, color));
                }
                break;
            } else if ((whitePieces & direction) != 0) {
                break;
            } else {
                moves.add(new Move(x, y, newX, newY, color));
            }
        }
    }
    public static void printBitboardAsChessboard(long bitboard) {
        for (int y = 7; y >= 0; y--) {
            for (int x = 0; x < 8; x++) {
                System.out.print((bitboard & (1L << (y * 8 + x))) != 0 ? "1" : "0");
            }
            System.out.println();
        }
    }

}
