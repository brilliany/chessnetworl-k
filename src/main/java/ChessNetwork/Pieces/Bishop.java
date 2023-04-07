package ChessNetwork.Pieces;


import ChessNetwork.Chessboard;

import java.util.ArrayList;

import static ChessNetwork.BoardUtils.BISHOP;
import static ChessNetwork.BoardUtils.addMovesInDirection;

public class Bishop {


    public static ArrayList<Move> getMoves(int x, int y, int color, Chessboard moveGenerator) {
        // The coordinates start from the top left corner (y = 0, x = 0), y is the vertical axis and x is the horizontal axis
        ArrayList<Move> moves = new ArrayList<>();
        long whitePieces = moveGenerator.getWhitePieces();
        long blackPieces = moveGenerator.getBlackPieces();

        // up right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, -1,BISHOP*color);
        // up left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, -1,BISHOP*color);
        // down right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, 1,BISHOP*color);
        // down left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, 1,BISHOP*color);

        return moves;
    }
}
