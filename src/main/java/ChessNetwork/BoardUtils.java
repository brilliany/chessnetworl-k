package ChessNetwork;

import ChessNetwork.Pieces.*;

import java.util.ArrayList;

import static java.lang.Math.abs;

public class BoardUtils {
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
    public static void addMovesInDirection(ArrayList<Move> moves, long blackPieces, long whitePieces, int color, int x, int y, int deltaX, int deltaY, int pieceType) {
        long direction = 0;
        for (int i = 1; i < 8; i++) {
            int newX = x + (i * deltaX);
            int newY = y + (i * deltaY);
            if (newX < 0 || newX >= 8 || newY < 0 || newY >= 8) {
                break; // Out of board
            }
            direction |= 1L << (newY * 8 + newX);
            if (color == 1) { // Black pieces
                if ((blackPieces & direction) != 0) {
                    if ((whitePieces & direction) == 0) {
                        moves.add(new Move(x, y, newX, newY, pieceType));
                    }
                    break;
                } else if ((whitePieces & direction) != 0) {
                    break;
                } else {
                    moves.add(new Move(x, y, newX, newY, pieceType));
                }
            } else { // White pieces
                if ((whitePieces & direction) != 0) {
                    if ((blackPieces & direction) == 0) {
                        moves.add(new Move(x, y, newX, newY, pieceType));
                    }
                    break;
                } else if ((blackPieces & direction) != 0) {
                    break;
                } else {
                    moves.add(new Move(x, y, newX, newY, pieceType));
                }
            }
        }
    }
    public static void printBitboardAsChessboard(long bitboard) {
        System.out.println("\n");
        for (int y = 0; y < 8; y++) {
            for (int x = 0; x < 8; x++) {
                System.out.print((bitboard & (1L << (y * 8 + x))) != 0 ? "1" : "0");
            }
            System.out.println();
        }
    }
    public static boolean isAttacked(int x, int y, int color, Chessboard chessboard) {
        long opponentPawns = color == BLACK ? chessboard.getBlackPawns() : chessboard.getWhitePawns();
        long opponentKnights = color == BLACK ? chessboard.getBlackKnights() : chessboard.getWhiteKnights();
        long opponentBishops = color == BLACK ? chessboard.getBlackBishops() : chessboard.getWhiteBishops();
        long opponentRooks = color == BLACK ? chessboard.getBlackRooks() : chessboard.getWhiteRooks();
        long opponentQueens = color == BLACK ? chessboard.getBlackQueens() : chessboard.getWhiteQueens();
        long opponentKings = color == BLACK ? chessboard.getBlackKings() : chessboard.getWhiteKings();

        if (isAttackedByPawn(x, y, color == BLACK, opponentPawns)) {
            return true;
        }

        if (isAttackedByKnight(x, y, opponentKnights)) {
            return true;
        }

        if (isAttackedByBishopOrQueen(x, y, opponentBishops, opponentQueens,chessboard.getWhitePieces(),chessboard.getBlackPieces())) {
            return true;
        }

        if (isAttackedByRookOrQueen(x, y, opponentRooks, opponentQueens,chessboard.getWhitePieces(),chessboard.getBlackPieces())) {
            return true;
        }

        return isAttackedByKing(x, y, opponentKings);
    }

    private static boolean isAttackedByPawn(int x, int y, boolean isWhite, long opponentPawns) {
        // Check if the given square is attacked by a pawn in the opponentPawns bitboard
        int direction = isWhite ? -1 : 1;
        int newY = y + direction;
        if (newY >= 0 && newY < 8) {
            if (x > 0 && isPieceAtPosition(newY, x - 1, opponentPawns)) {
                return true;
            }
            if (x < 7 && isPieceAtPosition(newY, x + 1, opponentPawns)) {
                return true;
            }
        }
        return false;
    }

    private static boolean isPieceAtPosition(int rowIndex, int columnIndex, long bitboard) {
        return (bitboard & (1L << (rowIndex * 8 + columnIndex))) != 0;
    }

    private static boolean isAttackedByKnight(int x, int y, long opponentKnights) {
        int[] deltaX = {-2, -1, 1, 2, 2, 1, -1, -2};
        int[] deltaY = {-1, -2, -2, -1, 1, 2, 2, 1};

        for (int i = 0; i < deltaX.length; i++) {
            int newX = x + deltaX[i];
            int newY = y + deltaY[i];

            if (newX >= 0 && newX < 8 && newY >= 0 && newY < 8 && isPieceAtPosition(newY, newX, opponentKnights)) {
                return true;
            }
        }

        return false;
    }

    private static boolean isAttackedByBishopOrQueen(int x, int y, long opponentBishops, long opponentQueens, long whitePieces, long blackPieces) {
        if (isAttackedByBishop(x, y, opponentBishops, whitePieces, blackPieces)) {
            return true;
        }
        if (isAttackedByQueen(x, y, opponentQueens, whitePieces, blackPieces)) {
            return true;
        }
        return false;
    }

    private static boolean isAttackedByRookOrQueen(int x, int y, long opponentRooks, long opponentQueens, long whitePieces, long blackPieces) {
        if (isAttackedByRook(x, y, opponentRooks, whitePieces, blackPieces)) {
            return true;
        }
        if (isAttackedByQueen(x, y, opponentQueens, whitePieces, blackPieces)) {
            return true;
        }
        return false;
    }

    private static boolean isAttackedByKing(int x, int y, long opponentKings) {
        int[] dx = {-1, -1, -1, 0, 0, 1, 1, 1};
        int[] dy = {-1, 0, 1, -1, 1, -1, 0, 1};

        for (int i = 0; i < dx.length; i++) {
            int nx = x + dx[i];
            int ny = y + dy[i];

            if (nx >= 0 && nx < 8 && ny >= 0 && ny < 8 && (opponentKings & (1L << (ny * 8 + nx))) != 0) {
                return true;
            }
        }

        return false;
    }

    private static boolean isAttackedByBishop(int x, int y, long opponentBishops, long whitePieces, long blackPieces) {
        return isAttackedByRay(x, y, opponentBishops, -1, 1, whitePieces, blackPieces) ||
                isAttackedByRay(x, y, opponentBishops, -1, -1, whitePieces, blackPieces) ||
                isAttackedByRay(x, y, opponentBishops, 1, 1, whitePieces, blackPieces) ||
                isAttackedByRay(x, y, opponentBishops, 1, -1, whitePieces, blackPieces);
    }

    private static boolean isAttackedByRook(int x, int y, long opponentRooks, long whitePieces, long blackPieces) {
        return isAttackedByRay(x, y, opponentRooks, -1, 0, whitePieces, blackPieces) ||
                isAttackedByRay(x, y, opponentRooks, 1, 0, whitePieces, blackPieces ) ||
                isAttackedByRay(x, y, opponentRooks, 0, -1, whitePieces, blackPieces) ||
                isAttackedByRay(x, y, opponentRooks, 0, 1, whitePieces, blackPieces);
    }

    private static boolean isAttackedByQueen(int x, int y, long opponentQueens, long whitePieces, long blackPieces) {
        return isAttackedByBishop(x, y, opponentQueens, whitePieces, blackPieces) || isAttackedByRook(x, y, opponentQueens, whitePieces, blackPieces);
    }

    private static boolean isAttackedByRay(int x, int y, long pieces, int dx, int dy, long whitePieces, long blackPieces) {
        //checks if the square (x, y) is attacked by a ray in the direction (dx, dy) by any of the pieces in the bitboard
        //takes into account the fact that the ray can be blocked by any other piece
        long allPieces = whitePieces | blackPieces;
        int nx = x + dx;
        int ny = y + dy;
        while (nx >= 0 && nx <= 7 && ny >= 0 && ny <= 7) {
            //cant be out of bounds
            if ((pieces & (1L << (ny * 8 + nx))) != 0) {
                return true;
            }
            if ((allPieces & (1L << (ny * 8 + nx))) != 0) {
                return false;
            }
            nx += dx;
            ny += dy;
        }
        return false;
    }
    public static void engineMove(Move move, Chessboard chessboard) {

        int fromSquare = move.getFromY() * 8 + move.getFromX();
        int toSquare = move.getToY() * 8 + move.getToX();
        int color = move.getPiece() > 0 ? WHITE : BLACK;

        int piece = move.getPiece();
        long fromMask = 1L << fromSquare;
        long toMask = 1L << toSquare;
        if ((chessboard.getWhitePieces() & toMask) != 0) {
            chessboard.setWhitePieces(chessboard.getWhitePieces() & ~toMask);
            if ((chessboard.getWhitePawns() & toMask) != 0) {
                chessboard.setWhitePawns(chessboard.getWhitePawns() & ~toMask);
            } else if ((chessboard.getWhiteKnights() & toMask) != 0) {
                chessboard.setWhiteKnights(chessboard.getWhiteKnights() & ~toMask);
            } else if ((chessboard.getWhiteBishops() & toMask) != 0) {
                chessboard.setWhiteBishops(chessboard.getWhiteBishops() & ~toMask);
            } else if ((chessboard.getWhiteRooks() & toMask) != 0) {
                chessboard.setWhiteRooks(chessboard.getWhiteRooks() & ~toMask);
            } else if ((chessboard.getWhiteQueens() & toMask) != 0) {
                chessboard.setWhiteQueens(chessboard.getWhiteQueens() & ~toMask);
            }
        } else if ((chessboard.getBlackPieces() & toMask) != 0) {
            chessboard.setBlackPieces(chessboard.getBlackPieces() & ~toMask);
            if ((chessboard.getBlackPawns() & toMask) != 0) {
                chessboard.setBlackPawns(chessboard.getBlackPawns() & ~toMask);
            } else if ((chessboard.getBlackKnights() & toMask) != 0) {
                chessboard.setBlackKnights(chessboard.getBlackKnights() & ~toMask);
            } else if ((chessboard.getBlackBishops() & toMask) != 0) {
                chessboard.setBlackBishops(chessboard.getBlackBishops() & ~toMask);
            } else if ((chessboard.getBlackRooks() & toMask) != 0) {
                chessboard.setBlackRooks(chessboard.getBlackRooks() & ~toMask);
            } else if ((chessboard.getBlackQueens() & toMask) != 0) {
                chessboard.setBlackQueens(chessboard.getBlackQueens() & ~toMask);
            }
        }
        if (piece > 0) { // White piece
            chessboard.setWhitePieces((chessboard.getWhitePieces() & ~fromMask) | toMask);
            switch (piece) {
                case PAWN -> chessboard.setWhitePawns((chessboard.getWhitePawns() & ~fromMask) | toMask);
                case KNIGHT -> chessboard.setWhiteKnights((chessboard.getWhiteKnights() & ~fromMask) | toMask);
                case BISHOP -> chessboard.setWhiteBishops((chessboard.getWhiteBishops() & ~fromMask) | toMask);
                case ROOK -> chessboard.setWhiteRooks((chessboard.getWhiteRooks() & ~fromMask) | toMask);
                case QUEEN -> chessboard.setWhiteQueens((chessboard.getWhiteQueens() & ~fromMask) | toMask);
                case KING -> chessboard.setWhiteKings((chessboard.getWhiteKings() & ~fromMask) | toMask);
            }
        } else { // Black piece
            chessboard.setBlackPieces((chessboard.getBlackPieces() & ~fromMask) | toMask);
            switch (piece) {
                case -PAWN -> chessboard.setBlackPawns((chessboard.getBlackPawns() & ~fromMask) | toMask);
                case -KNIGHT -> chessboard.setBlackKnights((chessboard.getBlackKnights() & ~fromMask) | toMask);
                case -BISHOP -> chessboard.setBlackBishops((chessboard.getBlackBishops() & ~fromMask) | toMask);
                case -ROOK -> chessboard.setBlackRooks((chessboard.getBlackRooks() & ~fromMask) | toMask);
                case -QUEEN -> chessboard.setBlackQueens((chessboard.getBlackQueens() & ~fromMask) | toMask);
                case -KING -> chessboard.setBlackKings((chessboard.getBlackKings() & ~fromMask) | toMask);
            }
        }
    }

}
