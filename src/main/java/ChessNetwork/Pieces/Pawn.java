package ChessNetwork.Pieces;

import ChessNetwork.Chessboard;

import java.util.ArrayList;

import static ChessNetwork.BoardUtils.*;

public class Pawn {



    public static ArrayList<Move> getMoves(int x, int y, int color, Chessboard chessboard, ArrayList<Move> moves) {
        //Color is 1 for white and -1 for black, so we can use it to determine the direction of the pawn
        //shift bitboard one and step in the direction of the pawn (*direction to get the right direction)
        int direction = -color;
        long oneStep = 1L << (x + (y + direction) * 8);
        long twoStep = 1L << (x + (y + direction * 2) * 8);
        long captureLeft = (x > 0) ? 1L << (x - 1 + (y + direction) * 8) : 0;
        long captureRight = (x < 7) ? 1L << (x + 1 + (y + direction) * 8) : 0;
        //get if the move is blocked
        switch (color) {
            case WHITE -> {
                long pieces = chessboard.getWhitePieces();
                long opponentPieces = chessboard.getBlackPieces();
                //if the onestep bit is empty in the whitePieces bitboard, add it to the moves
                if ((pieces & oneStep) == 0 && (opponentPieces & oneStep) == 0) {
                    moves.add(new Move(x, y, x, y + direction, PAWN*WHITE));
                    //if the pawn hasnt moved, check if the two step move is empty
                    if (y==6)
                        if ((pieces & twoStep) == 0 && (opponentPieces & twoStep) == 0) {
                        moves.add(new Move(x, y, x, y + direction * 2, PAWN*WHITE));
                    }
                }
                //check if the capture moves are valid
                if ((opponentPieces & captureLeft) != 0) {
                    moves.add(new Move(x, y, x - 1, y + direction, PAWN*WHITE));
                }
                if ((opponentPieces & captureRight) != 0) {
                    moves.add(new Move(x, y, x + 1, y + direction, PAWN*WHITE));
                }
                //en passant
                long blackPiecesPrev = chessboard.getHistory().get((chessboard.getHistory().size() - 1))[1];
                //if the pawn is on the 5th rank, check if the previous move was a double step move
                if (y == 4) {
                    //if the previous move was a double step move, check if the pawn is on the right side
                    enPassantCheck(x, y, color, moves, twoStep, blackPiecesPrev);
                }
            }
            case BLACK -> {
                long pieces = chessboard.getBlackPieces();
                long opponentPieces = chessboard.getWhitePieces();
                //if the onestep bit is empty in the whitePieces bitboard, add it to the moves
                if ((pieces & oneStep) == 0 && (opponentPieces & oneStep) == 0) {
                    moves.add(new Move(x, y, x, y + direction, PAWN*BLACK));
                    //if the pawn hasnt moved, check if the two step move is empty
                    if (y==1)
                        if ((pieces & twoStep) == 0) {
                            moves.add(new Move(x, y, x, y + direction * 2, PAWN*BLACK));
                        }
                }
                //check if the capture moves are valid
                if ((opponentPieces & captureLeft) != 0) {
                    moves.add(new Move(x, y, x - 1, y + direction, PAWN*BLACK));
                }
                if ((opponentPieces & captureRight) != 0) {
                    moves.add(new Move(x, y, x + 1, y + direction, PAWN*BLACK));
                }
                //en passant
                long whitePiecesPrev = chessboard.getHistory().get((chessboard.getHistory().size() - 1))[0];
                //if the pawn is on the 5th rank, check if the previous move was a double step move
                if (y == 3) {
                    //if the previous move was a double step move, check if the pawn is on the right side
                    enPassantCheck(x, y, color, moves, twoStep, whitePiecesPrev);
                }
            }
        }

        return moves;
    }

    private static void enPassantCheck(int x, int y, int color, ArrayList<Move> moves, long twoStep, long whitePiecesPrev) {
        //todo
    }

}
