package ChessNetwork;

import ChessNetwork.Pieces.Move;
import lombok.Getter;
import lombok.Setter;

import java.util.ArrayList;
import java.util.List;

import static ChessNetwork.BoardUtils.*;

public class Chessboard {
    @Getter
    @Setter
    private long whitePieces;

    @Getter
    @Setter
    private long blackPieces;

    //Bitboards
    @Getter
    @Setter
    private long whitePawns;

    @Getter
    @Setter
    private long blackPawns;

    @Getter
    @Setter
    private long whiteKnights;

    @Getter
    @Setter
    private long blackKnights;

    @Getter
    @Setter
    private long whiteBishops;

    @Getter
    @Setter
    private long blackBishops;

    @Getter
    @Setter
    private long whiteRooks;

    @Getter
    @Setter
    private long blackRooks;

    @Getter
    @Setter
    private long whiteQueens;

    @Getter
    @Setter
    private long blackQueens;

    @Getter
    @Setter
    private long whiteKings;

    @Getter
    @Setter
    private long blackKings;

    private long rights = 0b11;

    @Getter
    private List<long[]> history;
    final static int QUEEN_SIDE=0;
    final static int KING_SIDE=1;

    public Chessboard() {
        initChessboard();
    }

    private void initChessboard() {
        //Initializes the chessboard to the starting position using bitboards
        //like this whitePawns |= (1L << 8);
        for (int i = 0; i < 8; i++) {
            whitePawns |= (1L << 48 + i);
            blackPawns |= (1L << (8 + i));
        }
        whiteBishops |= (1L << 58) | (1L << 61);
        blackBishops |= (1L << 2) | (1L << 5);
        whiteKnights |= (1L << 57) | (1L << 62);
        blackKnights |= (1L << 1) | (1L << 6);
        whiteRooks |= (1L << 56) | (1L << 63);
        blackRooks |= (1L << 0) | (1L << 7);
        whiteQueens |= (1L << 59);
        blackQueens |= (1L << 3);
        whiteKings |= (1L << 60);
        blackKings |= (1L << 4);
        whitePieces = whitePawns | whiteBishops | whiteKnights | whiteRooks | whiteQueens | whiteKings;
        blackPieces = blackPawns | blackBishops | blackKnights | blackRooks | blackQueens | blackKings;
        rights = 0b1111;
        history = new ArrayList<>();
        savePosition();
    }

    public boolean getCastleRights(int color, int side) {
        //side 0 = queenside, side 1 = kingside
        if (color == 1) {
            return (rights & (1L << (2 * side))) != 0;
        } else {
            return (rights & (1L << (2 * side + 1))) != 0;
        }
    }

    public void setCastleRights(int color, int side, boolean value) {
        if (color == 1) {
            if (value) {
                rights |= (1L << (2 * side));
            } else {
                rights &= ~(1L << (2 * side));
            }
        } else {
            if (value) {
                rights |= (1L << (2 * side + 1));
            } else {
                rights &= ~(1L << (2 * side + 1));
            }
        }
    }

    public void resetChessBoard() {
        // Clear all pieces
        whitePawns = 0;
        blackPawns = 0;
        whiteKnights = 0;
        blackKnights = 0;
        whiteBishops = 0;
        blackBishops = 0;
        whiteRooks = 0;
        blackRooks = 0;
        whiteQueens = 0;
        blackQueens = 0;
        whiteKings = 0;
        blackKings = 0;
        whitePieces = 0;
        blackPieces = 0;
        rights = 0b1111;
        history.clear(); // clear the history list
        initChessboard();
    }

    public void print() {
        System.out.println("  A B C D E F G H");
        for (int y = 0; y < 8; y++) {
            System.out.print(y + 1 + " ");
            for (int x = 0; x < 8; x++) {
                int piece = getPieceType(x, y);
                if (piece == EMPTY) {
                    System.out.print(" 0 ");
                } else {
                    System.out.print((piece > 0 ? "W" : "B") + (Math.abs(piece) == PAWN ? "P" : Math.abs(piece) == KNIGHT ? "N" : Math.abs(piece) == BISHOP ? "B" : Math.abs(piece) == ROOK ? "R" : "Q") + " ");
                }
            }
            System.out.println();
        }

    }

    public long getAllCastleRights() {
        return rights;
    }

    public void setAllCastleRights(int castleRights) {
        rights = castleRights;
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
    public void makeMove(Move move) {

        //makes a move on the chessboard and calls the move listeners
        if (move == null) {
            System.out.println("Move is null");
            return;
        }
        if (move.getPiece() == EMPTY) {
            System.out.println("Move is empty");
            callMoveListeners(move);
            return;
        }
        move(move);
        callMoveListeners(move);
    }
    public void silentMove(Move move) {
        move(move);
    }
    public void move(Move move) {

        // Make the move
        int piece = move.getPiece();
        int x = move.getFromX();
        int y = move.getFromY();
        int newX = move.getToX();
        int newY = move.getToY();

        // Handle castling
        if (piece == WHITE * KING && x == 4 && y == 0) {
            if (newX == 2 && newY == 0 && getCastleRights(WHITE, QUEEN_SIDE)) {
                // Queen-side castling
                whiteKings &= ~(1L << 4);
                whiteKings |= (1L << 2);
                whiteRooks &= ~(1L << 0);
                whiteRooks |= (1L << 3);
                whitePieces &= ~(1L << 4);
                whitePieces |= (1L << 2) | (1L << 3);
            } else if (newX == 6 && newY == 0 && getCastleRights(WHITE, KING_SIDE)) {
                // King-side castling
                whiteKings &= ~(1L << 4);
                whiteKings |= (1L << 6);
                whiteRooks &= ~(1L << 7);
                whiteRooks |= (1L << 5);
                whitePieces &= ~(1L << 4);
                whitePieces |= (1L << 6) | (1L << 5);
            }
        } else if (piece == BLACK * KING && x == 4 && y == 7) {
            if (newX == 2 && newY == 7 && getCastleRights(BLACK, QUEEN_SIDE)) {
                // Queen-side castling
                blackKings &= ~(1L << 60);
                blackKings |= (1L << 58);
                blackRooks &= ~(1L << 56);
                blackRooks |= (1L << 59);
                blackPieces &= ~(1L << 60);
                blackPieces |= (1L << 58) | (1L << 59);
            } else if (newX == 6 && newY == 7 && getCastleRights(BLACK, KING_SIDE)) {
                // King-side castling
                blackKings &= ~(1L << 60);
                blackKings |= (1L << 62);
                blackRooks &= ~(1L << 63);
                blackRooks |= (1L << 61);
                blackPieces &= ~(1L << 60);
                blackPieces |= (1L << 62) | (1L << 61);
            }
        }

        // Handle promotion
        if (piece == WHITE * PAWN && newY == 7) {
            // Promote white pawn to queen
            whitePawns &= ~(1L << y * 8 + x);
            whiteQueens |= (1L << newY * 8 + newX);
            whitePieces &= ~(1L << y * 8 + x);
            whitePieces |= (1L << newY * 8 + newX);
        } else if (piece == BLACK * PAWN && newY == 0) {
            // Promote black pawn to queen
            blackPawns &= ~(1L << y * 8 + x);
            blackQueens |= (1L << newY * 8 + newX);
            blackPieces
                    &= ~(1L << y * 8 + x);
            blackPieces |= (1L << newY * 8 + newX);
        } else {
// Update the bitboards to reflect the move
            if ((whitePawns & (1L << y * 8 + x)) != 0) {
                whitePawns &= ~(1L << y * 8 + x);
                whitePawns |= (1L << newY * 8 + newX);
                whitePieces &= ~(1L << y * 8 + x);
                whitePieces |= (1L << newY * 8 + newX);
            } else if ((whiteKnights & (1L << y * 8 + x)) != 0) {
                whiteKnights &= ~(1L << y * 8 + x);
                whiteKnights |= (1L << newY * 8 + newX);
                whitePieces &= ~(1L << y * 8 + x);
                whitePieces |= (1L << newY * 8 + newX);
            } else if ((whiteBishops & (1L << y * 8 + x)) != 0) {
                whiteBishops &= ~(1L << y * 8 + x);
                whiteBishops |= (1L << newY * 8 + newX);
                whitePieces &= ~(1L << y * 8 + x);
                whitePieces |= (1L << newY * 8 + newX);
            } else if ((whiteRooks & (1L << y * 8 + x)) != 0) {
                whiteRooks &= ~(1L << y * 8 + x);
                whiteRooks |= (1L << newY * 8 + newX);
                whitePieces &= ~(1L << y * 8 + x);
                whitePieces |= (1L << newY * 8 + newX);
                if (x == 0) {
                    setCastleRights(WHITE, QUEEN_SIDE, false);
                } else if (x == 7) {
                    setCastleRights(WHITE, KING_SIDE, false);
                }
            } else if ((whiteQueens & (1L << y * 8 + x)) != 0) {
                whiteQueens &= ~(1L << y * 8 + x);
                whiteQueens |= (1L << newY * 8 + newX);
                whitePieces &= ~(1L << y * 8 + x);
                whitePieces |= (1L << newY * 8 + newX);
            } else if ((whiteKings & (1L << y * 8 + x)) != 0) {
                whiteKings &= ~(1L << y * 8 + x);
                whiteKings |= (1L << newY * 8 + newX);
                whitePieces &= ~(1L << y * 8 + x);
                whitePieces |= (1L << newY * 8 + newX);
                if (Math.abs(x - newX) == 2) {
                    if (x < newX) {
// King-side castling
                        whiteRooks &= ~(1L << 7);
                        whiteRooks |= (1L << 5);
                        whitePieces &= ~(1L << 7);
                        whitePieces |= (1L << 5);
                    } else {
// Queen-side castling
                        whiteRooks &= ~(1L << 0);
                        whiteRooks |= (1L << 3);
                        whitePieces &= ~(1L << 0);
                        whitePieces |= (1L << 3);
                    }
                }
                setCastleRights(WHITE, QUEEN_SIDE, false);
                setCastleRights(WHITE, KING_SIDE, false);
            } else if ((blackPawns & (1L << y * 8 + x)) != 0) {
                blackPawns &= ~(1L << y * 8 + x);
                blackPawns |= (1L << newY * 8 + newX);
                blackPieces &= ~(1L << y * 8 + x);
                blackPieces |= (1L << newY * 8 + newX);
            } else if ((blackKnights & (1L << y * 8 + x)) != 0) {
                blackKnights &= ~(1L << y * 8 + x);
                blackKnights |= (1L << newY * 8 + newX);
                blackPieces &= ~(1L << y * 8 + x);
                blackPieces |= (1L << newY * 8 + newX);
            } else if ((blackBishops & (1L << y * 8 + x)) != 0) {
                blackBishops &= ~(1L << y * 8 + x);
                blackBishops |= (1L << newY * 8 + newX);
                blackPieces &= ~(1L << y * 8 + x);
                blackPieces |= (1L << newY * 8 + newX);
            } else if ((blackRooks & (1L << y * 8 + x)) != 0) {
                blackRooks &= ~(1L << y * 8 + x);
                blackRooks |= (1L << newY * 8 + newX);
                blackPieces &= ~(1L << y * 8 + x);
                blackPieces |= (1L << newY * 8 + newX);
                if (x == 0) {
                    setCastleRights(BLACK, QUEEN_SIDE, false);
                } else if (x == 7) {
                    setCastleRights(BLACK, KING_SIDE, false);
                }
            } else if ((blackQueens & (1L << y * 8 + x)) != 0) {
                blackQueens &= ~(1L << y * 8 + x);
                blackQueens |= (1L << newY * 8 + newX);
                blackPieces &= ~(1L << y * 8 + x);
                blackPieces |= (1L << newY * 8 + newX);
            } else if ((blackKings & (1L << y * 8 + x)) != 0) {
                blackKings &= ~(1L << y * 8 + x);
                blackKings |= (1L << newY * 8 + newX);
                blackPieces &= ~(1L << y * 8 + x);
                blackPieces |= (1L << newY * 8 + newX);
                if (Math.abs(x - newX) == 2) {
                    if (x < newX) {
// King-side castling
                        blackRooks &= ~(1L << 7);
                        blackRooks |= (1L << 5);
                        blackPieces &= ~(1L << 7);
                        blackPieces |= (1L << 5);
                    } else {
// Queen-side castling
                        blackRooks &= ~(1L << 0);
                        blackRooks |= (1L << 3);
                        blackPieces &= ~(1L << 0);
                        blackPieces |= (1L << 3);
                    }
                }
                setCastleRights(BLACK, QUEEN_SIDE, false);
                setCastleRights(BLACK, KING_SIDE, false);
            }
        }

        // Save the previous board state before modifying it
        savePosition();
    }

    public void restorePrevious() {
        if (history.size() <= 1) {
            System.out.println("Cannot restore previous position");
            return;
        }
        history.remove(history.size() - 1);
        long[] position = history.get(history.size() - 1);
        whitePawns = position[0];
        blackPawns = position[1];
        whiteKnights = position[2];
        blackKnights = position[3];
        whiteBishops = position[4];
        blackBishops = position[5];
        whiteRooks = position[6];
        blackRooks = position[7];
        whiteQueens = position[8];
        blackQueens = position[9];
        whiteKings = position[10];
        blackKings = position[11];
        whitePieces = position[12];
        blackPieces = position[13];
        rights = position[14];
    }
    public void savePosition() {
        long[] position = {
                whitePawns, blackPawns, whiteKnights, blackKnights, whiteBishops,
                blackBishops, whiteRooks, blackRooks, whiteQueens, blackQueens,
                whiteKings, blackKings, whitePieces, blackPieces, rights
        };
        history.add(position);
    }

    //Listener section
    private final List<MoveListener> moveListeners = new ArrayList<>();

    //then add a method to add a listener
    public MoveListener addMoveListener(MoveListener listener) {
        moveListeners.add(listener);
        return listener;
    }

    //then call the method on all listeners
    void callMoveListeners(Move move) {
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

}
