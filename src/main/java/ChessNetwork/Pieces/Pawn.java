package ChessNetwork.Pieces;

import ChessNetwork.ChessboardHelper;
import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.*;

public class Pawn {



    public static Move[] getMoves(int x, int y, int color, MoveGenerator moveGenerator) {
        //Color is 1 for white and -1 for black, so we can use it to determine the direction of the pawn
        ArrayList<Move> moves = new ArrayList<>();
        //shift bitboard one and step in the direction of the pawn (*color to get the right direction)
        long oneStep = 1L << (x+8*(y+color));
        long twoStep = 1L << (x+16*(y+color));
        long captureLeft = 1L << (x-1+8*(y+color));
        long captureRight = 1L << (x+1+8*(y+color));

        //get if the move is blocked
        switch (color) {
            case WHITE -> {
                long pieces = moveGenerator.getWhitePieces();
                ChessboardHelper.printBitboardAsChessboard(pieces);
                //if the onestep bit is empty in the whitePieces bitboard, add it to the moves
                if ((pieces & oneStep) == 0) {
                    moves.add(new Move(x, y, x, y + color, PAWN*WHITE));
                    //if the pawn hasnt moved, check if the two step move is empty
                    if (y == 1) {
                        if ((pieces & twoStep) == 0) {
                            moves.add(new Move(x, y, x, y + color * 2, PAWN*WHITE));
                        }
                    }
                }
                //check if the capture moves are valid
                if ((moveGenerator.getBlackPieces() & captureLeft) != 0) {
                    moves.add(new Move(x, y, x - 1, y + color, PAWN*WHITE));
                }
                if ((moveGenerator.getBlackPieces() & captureRight) != 0) {
                    moves.add(new Move(x, y, x + 1, y + color, PAWN*WHITE));
                }
                //en passant
                long blackPiecesPrev = moveGenerator.getBlackPiecesPrevious();
                //if the pawn is on the 5th rank, check if the previous move was a double step move
                if (y == 4) {
                    //if the previous move was a double step move, check if the pawn is on the right side
                    enPassantCheck(x, y, color, moves, twoStep, blackPiecesPrev, WHITE);
                }
            }
            case BLACK -> {
                long pieces = moveGenerator.getBlackPieces();
                //if the onestep bit is empty in the blackPieces bitboard, add it to the moves
                if ((pieces & oneStep) == 0) {
                    moves.add(new Move(x, y, x, y + color, PAWN*BLACK));
                    //if the pawn hasnt moved, check if the two step move is empty
                    if (y == 6) {
                        if ((pieces & twoStep) == 0) {
                            moves.add(new Move(x, y, x, y + color * 2, PAWN*BLACK));
                        }
                    }
                }
                //check if the capture moves are valid
                if ((moveGenerator.getWhitePieces() & captureLeft) != 0) {
                    moves.add(new Move(x, y, x - 1, y + color, PAWN*BLACK));
                }
                if ((moveGenerator.getWhitePieces() & captureRight) != 0) {
                    moves.add(new Move(x, y, x + 1, y + color, PAWN*BLACK));
                }
                //en passant
                long whitePiecesPrev = moveGenerator.getWhitePiecesPrevious();
                //if the pawn is on the 5th rank, check if the previous move was a double step move
                if (y == 3) {
                    //if the previous move was a double step move, check if the pawn is on the right side
                    enPassantCheck(x, y, color, moves, twoStep, whitePiecesPrev, BLACK);
                }
            }
        }
        System.out.println("Pawn moves: " + moves);
        return moves.toArray(new Move[0]);
    }

    private static void enPassantCheck(int x, int y, int color, ArrayList<Move> moves, long twoStep, long whitePiecesPrev, int black) {
        if ((whitePiecesPrev & twoStep) != 0) {
            //if the pawn is on the right side, check if the pawn is on the right side
            if (x == 0) {
                moves.add(new Move(x, y, x + 1, y + color, PAWN* black));
            } else if (x == 7) {
                moves.add(new Move(x, y, x - 1, y + color, PAWN* black));
            } else {
                moves.add(new Move(x, y, x + 1, y + color, PAWN* black));
                moves.add(new Move(x, y, x - 1, y + color, PAWN* black));
            }
        }
    }

}
