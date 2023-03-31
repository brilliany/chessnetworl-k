package ChessNetwork.Pieces;

import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.*;


public class King {

    public static Move[] getMoves(int x, int y, int color,MoveGenerator moveGenerator) {

        ArrayList<Move> moves = new ArrayList<>();
        long whitePieces = moveGenerator.getWhitePieces();
        long blackPieces = moveGenerator.getBlackPieces();
        int opponentColor = color == WHITE ? BLACK : WHITE;

        // up
        long up = (y+1L)*8 + x;
        if ((whitePieces & up) == 0 && (blackPieces & up) == 0) {
            moves.add(new Move(x, y, x, y+1, KING));
        } else if ((whitePieces & up) == 0 && color == WHITE) {
            moves.add(new Move(x, y, x, y+1, KING));
        } else if ((blackPieces & up) == 0 && color == BLACK) {
            moves.add(new Move(x, y, x, y+1, KING));
        }
        // down
        long down = (y-1L)*8 + x;
        if ((whitePieces & down) == 0 && (blackPieces & down) == 0) {
            moves.add(new Move(x, y, x, y-1, KING));
        } else if ((whitePieces & down) == 0 && color == WHITE) {
            moves.add(new Move(x, y, x, y-1, KING));
        } else if ((blackPieces & down) == 0 && color == BLACK) {
            moves.add(new Move(x, y, x, y-1, KING));
        }
        // right
        long right = y* 8L + x+1L;
        if ((whitePieces & right) == 0 && (blackPieces & right) == 0) {
            moves.add(new Move(x, y, x+1, y, KING));
        } else if ((whitePieces & right) == 0 && color == WHITE) {
            moves.add(new Move(x, y, x+1, y, KING));
        } else if ((blackPieces & right) == 0 && color == BLACK) {
            moves.add(new Move(x, y, x+1, y, KING));
        }
        // left
        long left = y* 8L + x-1L;
        if ((whitePieces & left) == 0 && (blackPieces & left) == 0) {
            moves.add(new Move(x, y, x-1, y, KING));
        } else if ((whitePieces & left) == 0 && color == WHITE) {
            moves.add(new Move(x, y, x-1, y, KING));
        } else if ((blackPieces & left) == 0 && color == BLACK) {
            moves.add(new Move(x, y, x-1, y, KING));
        }
        // up right
        long upRight = (y+1L)*8 + x+1L;
        if ((whitePieces & upRight) == 0 && (blackPieces & upRight) == 0) {
            moves.add(new Move(x, y, x+1, y+1, KING));
        } else if ((whitePieces & upRight) == 0 && color == WHITE) {
            moves.add(new Move(x, y, x+1, y+1, KING));
        } else if ((blackPieces & upRight) == 0 && color == BLACK) {
            moves.add(new Move(x, y, x+1, y+1, KING));
        }
        // up left
        long upLeft = (y+1L)*8 + x-1L;
        if ((whitePieces & upLeft) == 0 && (blackPieces & upLeft) == 0) {
            moves.add(new Move(x, y, x-1, y+1, KING));
        } else if ((whitePieces & upLeft) == 0 && color == WHITE) {
            moves.add(new Move(x, y, x-1, y+1, KING));
        } else if ((blackPieces & upLeft) == 0 && color == BLACK) {
            moves.add(new Move(x, y, x-1, y+1, KING));
        }
        // down right
        long downRight = (y-1L)*8 + x+1L;
        if ((whitePieces & downRight) == 0 && (blackPieces & downRight) == 0) {
            moves.add(new Move(x, y, x+1, y-1, KING));
        } else if ((whitePieces & downRight) == 0 && color == WHITE) {
            moves.add(new Move(x, y, x+1, y-1, KING));
        } else if ((blackPieces & downRight) == 0 && color == BLACK) {
            moves.add(new Move(x, y, x+1, y-1, KING));
        }
        // down left
        long downLeft = (y-1L)*8 + x-1L;
        if ((whitePieces & downLeft) == 0 && (blackPieces & downLeft) == 0) {
            moves.add(new Move(x, y, x-1, y-1, KING));
        } else if ((whitePieces & downLeft) == 0 && color == WHITE) {
            moves.add(new Move(x, y, x-1, y-1, KING));
        } else if ((blackPieces & downLeft) == 0 && color == BLACK) {
            moves.add(new Move(x, y, x-1, y-1, KING));
        }
        // castling
        handleCastling(moves, x, y, color, moveGenerator);

        return moves.toArray(new Move[0]);
    }

    private static void handleCastling(ArrayList<Move> moves, int x, int y, int color, MoveGenerator moveGenerator) {
        if (moveGenerator.isCheck(color)){
            return;
        }
        boolean rightsShort = moveGenerator.getCastleRights(color, 0);
        boolean rightsLong = moveGenerator.getCastleRights(color, 1);
        long whitePieces = moveGenerator.getWhitePieces();
        long blackPieces = moveGenerator.getBlackPieces();
        int opponentColor = color == WHITE ? BLACK : WHITE;
        if (color == WHITE) {
            if (rightsShort) {
                //if the squares between the king and the rook are empty
                if ((whitePieces & (1L << 61)) == 0 && (whitePieces & (1L << 62)) == 0) {
                    //if the squares are not attacked by the opponent
                    int firstSquareX = 5;
                    int secondSquareX = 6;
                    if (!moveGenerator.isAttacked(firstSquareX, 0, opponentColor) && !moveGenerator.isAttacked(secondSquareX, 0, opponentColor)) {
                        moves.add(new Move(x, y, 6, y, KING));
                    }
                }
            }
            if (rightsLong) {
                //if the squares between the king and the rook are empty
                if ((whitePieces & (1L << 57)) == 0 && (whitePieces & (1L << 58)) == 0 && (whitePieces & (1L << 59)) == 0) {
                    //if the squares are not attacked by the opponent
                    int firstSquareX = 3;
                    int secondSquareX = 2;
                    int thirdSquareX = 1;
                    if (!moveGenerator.isAttacked(firstSquareX, 0, opponentColor) && !moveGenerator.isAttacked(secondSquareX, 0, opponentColor) && !moveGenerator.isAttacked(thirdSquareX, 0, opponentColor)) {
                        moves.add(new Move(x, y, 2, y, KING));
                    }
                }
            }
        }
        if (color == BLACK) {
            if (rightsShort) {
                //if the squares between the king and the rook are empty
                if ((blackPieces & (1L << 5)) == 0 && (blackPieces & (1L << 6)) == 0) {
                    //if the squares are not attacked by the opponent
                    int firstSquareX = 5;
                    int secondSquareX = 6;
                    if (!moveGenerator.isAttacked(firstSquareX, 7, opponentColor) && !moveGenerator.isAttacked(secondSquareX, 7, opponentColor)) {
                        moves.add(new Move(x, y, 6, y, KING));
                    }
                }
            }
            if (rightsLong) {
                //if the squares between the king and the rook are empty
                if ((blackPieces & (1L << 1)) == 0 && (blackPieces & (1L << 2)) == 0 && (blackPieces & (1L << 3)) == 0) {
                    //if the squares are not attacked by the opponent
                    int firstSquareX = 3;
                    int secondSquareX = 2;
                    int thirdSquareX = 1;
                    if (!moveGenerator.isAttacked(firstSquareX, 7, opponentColor) && !moveGenerator.isAttacked(secondSquareX, 7, opponentColor) && !moveGenerator.isAttacked(thirdSquareX, 7, opponentColor)) {
                        moves.add(new Move(x, y, 2, y, KING));
                    }
                }
            }
        }
    }
}