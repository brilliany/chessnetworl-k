package ChessNetwork.Pieces;


import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.addMovesInDirection;

public class Bishop {


    public static Move[] getMoves(int x, int y, int color,MoveGenerator moveGenerator) {
        // The coordinates start from the top left corner (y = 0, x = 0), y is the vertical axis and x is the horizontal axis
        ArrayList<Move> moves = new ArrayList<>();
        long whitePieces = moveGenerator.getWhitePieces();
        long blackPieces = moveGenerator.getBlackPieces();

        // up right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, -1);
        // up left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, -1);
        // down right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, 1);
        // down left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, 1);

        return moves.toArray(new Move[0]);
    }
}
