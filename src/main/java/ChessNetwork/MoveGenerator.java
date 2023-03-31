package ChessNetwork;


import ChessNetwork.Pieces.*;
import lombok.Getter;

import java.util.ArrayList;
import java.util.List;

import static ChessNetwork.ChessboardHelper.*;

public class MoveGenerator {
    public MoveGenerator() {
        initChessboard();
    }

    @Getter
    private long whitePieces;
    @Getter
    private long whitePiecesPrevious;
    @Getter
    private long blackPieces;
    @Getter
    private long blackPiecesPrevious;
    //Bitboards
    @Getter
    private long whitePawns;
    @Getter
    private long blackPawns;
    @Getter
    private long whiteKnights;
    @Getter
    private long blackKnights;
    @Getter
    private long whiteBishops;
    @Getter
    private long blackBishops;
    @Getter
    private long whiteRooks;
    @Getter
    private long blackRooks;
    @Getter
    private long whiteQueens;
    @Getter
    private long blackQueens;
    @Getter
    private long whiteKings;
    @Getter
    private long blackKings;

    private long whiteCastleRights = 0b11L;

    private long blackCastleRights = 0b11L;


    private void initChessboard() {
        //Initializes the chessboard to the starting position using bitboards
        //like this whitePawns |= (1L << 8);
        for (int i = 0; i < 8; i++) {
            blackPawns |= (1L << 48 + i);
            whitePawns |= (1L << (8 + i));
        }
        blackBishops |= (1L << 58) | (1L << 61);
        whiteBishops |= (1L << 2) | (1L << 5);
        blackKnights |= (1L << 57) | (1L << 62);
        whiteKnights |= (1L << 1) | (1L << 6);
        blackRooks |= (1L << 56) | (1L << 63);
        whiteRooks |= (1L << 0) | (1L << 7);
        blackQueens |= (1L << 59);
        whiteQueens |= (1L << 3);
        blackKings |= (1L << 60);
        whiteKings |= (1L << 4);
        blackPieces = whitePawns | whiteBishops | whiteKnights | whiteRooks | whiteQueens | whiteKings;
        whitePieces = blackPawns | blackBishops | blackKnights | blackRooks | blackQueens | blackKings;
        print();
    }


    public void makeMove(Move move) {
        int fromSquare = move.getFromY() * 8 + move.getFromX();
        int toSquare = move.getToY() * 8 + move.getToX();
        int color = move.getPiece() > 0 ? WHITE : BLACK;
        if (Math.abs(move.getPiece()) == KING && Math.abs(move.getToX() - move.getFromX()) == 2) {
            //Castling move
            int rookFromSquare = move.getToX() == 6 ? 63 : 56;
            int rookToSquare = move.getToX() == 6 ? 61 : 59;

            whiteKings = (whiteKings & ~(1L << 60)) | (1L << toSquare);
            whiteRooks = (whiteRooks & ~(1L << rookFromSquare)) | (1L << rookToSquare);

            // Update castle rights
            setCastleRights(color, move.getToX() == 6 ? 0 : 1, 0);
        } else {
            int piece = move.getPiece();
            long fromMask = 1L << fromSquare;
            long toMask = 1L << toSquare;

            if (piece > 0) { // White piece
                whitePawns &= ~fromMask & ~toMask;
                whiteKnights &= ~fromMask & ~toMask;
                whiteBishops &= ~fromMask & ~toMask;
                whiteRooks &= ~fromMask & ~toMask;
                whiteQueens &= ~fromMask & ~toMask;
                whiteKings &= ~fromMask | toMask;
                whitePieces = (whitePieces & ~fromMask) | toMask;
                blackPieces &= ~toMask;
            } else { // Black piece
                blackPawns &= ~fromMask & ~toMask;
                blackKnights &= ~fromMask & ~toMask;
                blackBishops &= ~fromMask & ~toMask;
                blackRooks &= ~fromMask & ~toMask;
                blackQueens &= ~fromMask & ~toMask;
                blackKings &= ~fromMask | toMask;
                blackPieces = (blackPieces & ~fromMask) | toMask;
                whitePieces &= ~toMask;
            }


            // Update castle rights for non-castling moves
            if (piece == KING * color) {
                setCastleRights(color, 0, 0);
                setCastleRights(color, 1, 0);
            } else if (piece == ROOK * color) {
                if (fromSquare == 56) {
                    setCastleRights(color, 1, 0);
                } else if (fromSquare == 63) {
                    setCastleRights(color, 0, 0);
                } else if (fromSquare == 0) {
                    setCastleRights(color, 1, 0);
                } else if (fromSquare == 7) {
                    setCastleRights(color, 0, 0);
                }
            }
        }
        callMoveListeners(move);
    }

    //castling rights
    public void setCastleRights(int color, int side, int value) {
        // side 0 = short, side 1 = long
        // value 0 = no rights, value 1 = rights
        if (color == WHITE) {
            if (side == 0) {
                whiteCastleRights = (whiteCastleRights & 0b10) | value;
            } else {
                whiteCastleRights = (whiteCastleRights & 0b01) | ((long) value << 1);
            }
        } else {
            if (side == 0) {
                blackCastleRights = (blackCastleRights & 0b10) | value;
            } else {
                blackCastleRights = (blackCastleRights & 0b01) | ((long) value << 1);
            }
        }
    }

    public boolean getCastleRights(int color, int side) {
        // side 0 = short, side 1 = long
        if (color == WHITE) {
            if (side == 0) {
                return (whiteCastleRights & 0b01) == 1;
            } else {
                return (whiteCastleRights & 0b10) == 1;
            }
        } else {
            if (side == 0) {
                return (blackCastleRights & 0b01) == 1;
            } else {
                return (blackCastleRights & 0b10) == 1;
            }
        }
    }

    public void resetChessBoard() {
        initChessboard();
        //remove all listeners
        moveListeners.clear();
    }

    //then create a list of listeners
    private final List<MoveListener> moveListeners = new ArrayList<>();

    //then add a method to add a listener
    public MoveListener addMoveListener(MoveListener listener) {
        moveListeners.add(listener);
        return listener;
    }

    //then call the method on all listeners
    private void callMoveListeners(Move move) {
        for (MoveListener listener : moveListeners) {
            listener.onMove(move);
        }
    }

    private List<MoveListener> getMoveListeners() {
        return moveListeners;
    }

    //remove a listener
    public void removeMoveListener(MoveListener listener) {
        moveListeners.remove(listener);
    }

    public List<Move> getAllMoves(int color) {
        List<Move> allMoves = new ArrayList<>();

        long pieces = color == WHITE ? whitePieces : blackPieces;
        long pawns = color == WHITE ? whitePawns : blackPawns;
        long knights = color == WHITE ? whiteKnights : blackKnights;
        long bishops = color == WHITE ? whiteBishops : blackBishops;
        long rooks = color == WHITE ? whiteRooks : blackRooks;
        long queens = color == WHITE ? whiteQueens : blackQueens;
        long kings = color == WHITE ? whiteKings : blackKings;

        // Loop over all pieces of the given color
        long piece;
        while (pieces != 0) {
            piece = Long.highestOneBit(pieces);
            pieces ^= piece;

            int fromSquare = Long.numberOfTrailingZeros(piece);
            Move[] moves = null;
            // Get moves for the current piece type
            if ((pawns & piece) != 0) {
                moves = Pawn.getMoves(fromSquare % 8, fromSquare / 8, color, this);
            } else if ((knights & piece) != 0) {
                moves = Knight.getMoves(fromSquare % 8, fromSquare / 8, color, this);
            } else if ((bishops & piece) != 0) {
                moves = Bishop.getMoves(fromSquare % 8, fromSquare / 8, color, this);
            } else if ((rooks & piece) != 0) {
                moves = Rook.getMoves(fromSquare % 8, fromSquare / 8, color, this);
            } else if ((queens & piece) != 0) {
                moves = Queen.getMoves(fromSquare % 8, fromSquare / 8, color, this);
            } else if ((kings & piece) != 0) {
                moves = King.getMoves(fromSquare % 8, fromSquare / 8, color, this);
            }

            // Add the moves for the current piece to the list of all moves
            if (moves != null) {
                allMoves.addAll(List.of(moves));
            }
        }
        return allMoves;
    }

    public boolean putsKingInCheck(Move move, int color) {
        // Make a copy of the current board state
        long whitePiecesCopy = whitePieces;
        long whitePawnsCopy = whitePawns;
        long whiteKnightsCopy = whiteKnights;
        long whiteBishopsCopy = whiteBishops;
        long whiteRooksCopy = whiteRooks;
        long whiteQueensCopy = whiteQueens;
        long whiteKingsCopy = whiteKings;
        long blackPiecesCopy = blackPieces;
        long blackPawnsCopy = blackPawns;
        long blackKnightsCopy = blackKnights;
        long blackBishopsCopy = blackBishops;
        long blackRooksCopy = blackRooks;
        long blackQueensCopy = blackQueens;
        long blackKingsCopy = blackKings;

        long castleRightsWhite = whiteCastleRights;
        long castleRightsBlack = blackCastleRights;


        // Update the board state with the move
        makeMoveSilent(move);

// Check if the move puts the opponent's king in check
        boolean inCheck = isCheck(color);

// Restore the previous board state
        whitePieces = whitePiecesCopy;
        whitePawns = whitePawnsCopy;
        whiteKnights = whiteKnightsCopy;
        whiteBishops = whiteBishopsCopy;
        whiteRooks = whiteRooksCopy;
        whiteQueens = whiteQueensCopy;
        whiteKings = whiteKingsCopy;
        blackPieces = blackPiecesCopy;
        blackPawns = blackPawnsCopy;
        blackKnights = blackKnightsCopy;
        blackBishops = blackBishopsCopy;
        blackRooks = blackRooksCopy;
        blackQueens = blackQueensCopy;
        blackKings = blackKingsCopy;
        whiteCastleRights = castleRightsWhite;
        blackCastleRights = castleRightsBlack;

        return inCheck;
    }

    private void makeMoveSilent(Move move) {
        int fromSquare = move.getFromY() * 8 + move.getFromX();
        int toSquare = move.getToY() * 8 + move.getToX();
        int color = move.getPiece() > 0 ? WHITE : BLACK;
        if (Math.abs(move.getPiece()) == KING && Math.abs(move.getToX() - move.getFromX()) == 2) {
            //Castling move
            int rookFromSquare = move.getToX() == 6 ? 63 : 56;
            int rookToSquare = move.getToX() == 6 ? 61 : 59;

            whiteKings = (whiteKings & ~(1L << 60)) | (1L << toSquare);
            whiteRooks = (whiteRooks & ~(1L << rookFromSquare)) | (1L << rookToSquare);

            // Update castle rights
            setCastleRights(color, move.getToX() == 6 ? 0 : 1, 0);
        } else {
            int piece = move.getPiece();
            long fromMask = 1L << fromSquare;
            long toMask = 1L << toSquare;

            if (piece > 0) { // White piece
                whitePawns &= ~fromMask & ~toMask;
                whiteKnights &= ~fromMask & ~toMask;
                whiteBishops &= ~fromMask & ~toMask;
                whiteRooks &= ~fromMask & ~toMask;
                whiteQueens &= ~fromMask & ~toMask;
                whiteKings &= ~fromMask | toMask;
                whitePieces = (whitePieces & ~fromMask) | toMask;
                blackPieces &= ~toMask;
            } else { // Black piece
                blackPawns &= ~fromMask & ~toMask;
                blackKnights &= ~fromMask & ~toMask;
                blackBishops &= ~fromMask & ~toMask;
                blackRooks &= ~fromMask & ~toMask;
                blackQueens &= ~fromMask & ~toMask;
                blackKings &= ~fromMask | toMask;
                blackPieces = (blackPieces & ~fromMask) | toMask;
                whitePieces &= ~toMask;
            }


            // Update castle rights for non-castling moves
            if (piece == KING * color) {
                setCastleRights(color, 0, 0);
                setCastleRights(color, 1, 0);
            } else if (piece == ROOK * color) {
                if (fromSquare == 56) {
                    setCastleRights(color, 1, 0);
                } else if (fromSquare == 63) {
                    setCastleRights(color, 0, 0);
                } else if (fromSquare == 0) {
                    setCastleRights(color, 1, 0);
                } else if (fromSquare == 7) {
                    setCastleRights(color, 0, 0);
                }
            }
        }
    }


    public void print() {
        System.out.println("  A B C D E F G H");
        for (int y = 7; y >= 0; y--) {
            System.out.print(y + 1 + " ");
            for (int x = 0; x < 8; x++) {
                int piece = getPieceType(x, y);
                if (piece == EMPTY) {
                    System.out.print("  ");
                } else {
                    System.out.print((piece > 0 ? "W" : "B") + (Math.abs(piece) == PAWN ? "P" : Math.abs(piece) == KNIGHT ? "N" : Math.abs(piece) == BISHOP ? "B" : Math.abs(piece) == ROOK ? "R" : "Q") + " ");
                }
            }
            System.out.println();
        }

    }

    public int getPieceType(int x, int y) {
        int piece = EMPTY;
        long mask = 1L << (y * 8 + x);
        if ((whitePawns & mask) != 0) {
            piece = WHITE * PAWN; // White Pawn
        } else if ((blackPawns & mask) != 0) {
            piece = BLACK * PAWN; // Black Pawn
        } else if ((whiteKnights & mask) != 0) {
            piece = WHITE * KNIGHT; // White Knight
        } else if ((blackKnights & mask) != 0) {
            piece = BLACK * KNIGHT; // Black Knight
        } else if ((whiteBishops & mask) != 0) {
            piece = WHITE * BISHOP; // White Bishop
        } else if ((blackBishops & mask) != 0) {
            piece = BLACK * BISHOP; // Black Bishop
        } else if ((whiteRooks & mask) != 0) {
            piece = WHITE * ROOK; // White Rook
        } else if ((blackRooks & mask) != 0) {
            piece = BLACK * ROOK; // Black Rook
        } else if ((whiteQueens & mask) != 0) {
            piece = WHITE * QUEEN; // White Queen
        } else if ((blackQueens & mask) != 0) {
            piece = BLACK * QUEEN; // Black Queen
        } else if ((whiteKings & mask) != 0) {
            piece = WHITE * KING; // White King
        } else if ((blackKings & mask) != 0) {
            piece = BLACK * KING; // Black King
        }
        return piece;
    }

    public boolean isCheckmate(int color) {
        // Check if the given color is in check
        if (isCheck(color)) {
            List<Move> allMoves = getAllMoves(color);
            for (Move move : allMoves) {
                if (!putsKingInCheck(move, color)) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }

    public boolean isStalemate(int color) {
        // Check if the given color is not in check
        if (!isCheck(color)) {
            List<Move> allMoves = getAllMoves(color);
            for (Move move : allMoves) {
                if (!putsKingInCheck(move, color)) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }


    public boolean isCheck(int color) {
        // Check if the given color is in check
        //get the position of the king
        long king = color == WHITE ? whiteKings : blackKings;
        int x = Long.numberOfTrailingZeros(king) % 8;
        int y = Long.numberOfTrailingZeros(king) / 8;
        return isAttacked(x, y, color);
    }


    public boolean isAttacked(int x, int y, int color) {
        long opponentPawns = color == WHITE ? blackPawns : whitePawns;
        long opponentKnights = color == WHITE ? blackKnights : whiteKnights;
        long opponentBishops = color == WHITE ? blackBishops : whiteBishops;
        long opponentRooks = color == WHITE ? blackRooks : whiteRooks;
        long opponentQueens = color == WHITE ? blackQueens : whiteQueens;
        long opponentKings = color == WHITE ? blackKings : whiteKings;

        if (isAttackedByPawn(x, y, opponentPawns)) {
            return true;
        }

        if (isAttackedByKnight(x, y, opponentKnights)) {
            return true;
        }

        if (isAttackedByBishopOrQueen(x, y, opponentBishops, opponentQueens)) {
            return true;
        }

        if (isAttackedByRookOrQueen(x, y, opponentRooks, opponentQueens)) {
            return true;
        }

        if (isAttackedByKing(x, y, opponentKings)) {
            return true;
        }

        return false;
    }

    private boolean isAttackedByPawn(int x, int y, long opponentPawns) {
        if (x > 0 && y > 0 && (opponentPawns & (1L << ((y - 1) * 8 + x - 1))) != 0) {
            return true;
        }
        if (x < 7 && y > 0 && (opponentPawns & (1L << ((y - 1) * 8 + x + 1))) != 0) {
            return true;
        }
        return false;
    }

    private boolean isAttackedByKnight(int x, int y, long opponentKnights) {
        int[] dx = {-2, -1, 1, 2, 2, 1, -1, -2};
        int[] dy = {-1, -2, -2, -1, 1, 2, 2, 1};

        for (int i = 0; i < dx.length; i++) {
            int nx = x + dx[i];
            int ny = y + dy[i];

            if (nx >= 0 && nx < 8 && ny >= 0 && ny < 8 && (opponentKnights & (1L << (ny * 8 + nx))) != 0) {
                return true;
            }
        }

        return false;
    }

    private boolean isAttackedByBishopOrQueen(int x, int y, long opponentBishops, long opponentQueens) {
        if (isAttackedByBishop(x, y, opponentBishops)) {
            return true;
        }
        if (isAttackedByQueen(x, y, opponentQueens)) {
            return true;
        }
        return false;
    }

    private boolean isAttackedByRookOrQueen(int x, int y, long opponentRooks, long opponentQueens) {
        if (isAttackedByRook(x, y, opponentRooks)) {
            return true;
        }
        if (isAttackedByQueen(x, y, opponentQueens)) {
            return true;
        }
        return false;
    }

    private boolean isAttackedByKing(int x, int y, long opponentKings) {
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

    private boolean isAttackedByBishop(int x, int y, long opponentBishops) {
        return isAttackedByRay(x, y, opponentBishops, -9, 7) ||
                isAttackedByRay(x, y, opponentBishops, -7, 9) ||
                isAttackedByRay(x, y, opponentBishops, 7, -9) ||
                isAttackedByRay(x, y, opponentBishops, 9, -7);
    }

    private boolean isAttackedByRook(int x, int y, long opponentRooks) {
        return isAttackedByRay(x, y, opponentRooks, -1, 0) ||
                isAttackedByRay(x, y, opponentRooks, 1, 0) ||
                isAttackedByRay(x, y, opponentRooks, 0, -1) ||
                isAttackedByRay(x, y, opponentRooks, 0, 1);
    }

    private boolean isAttackedByQueen(int x, int y, long opponentQueens) {
        return isAttackedByBishop(x, y, opponentQueens) || isAttackedByRook(x, y, opponentQueens);
    }

    private boolean isAttackedByRay(int x, int y, long pieces, int dx, int dy) {
        int nx = x + dx;
        int ny = y + dy;

        while (nx >= 0 && nx < 8 && ny >= 0 && ny < 8) {
            if ((pieces & (1L << (ny * 8 + nx))) != 0) {
                return true;
            }
            nx += dx;
            ny += dy;
        }

        return false;
    }
}



