package ChessNetwork.Pieces;

import ChessNetwork.Chessboard;

import java.util.ArrayList;

import static ChessNetwork.BoardUtils.*;

public class Rook {

    public static ArrayList<Move> getMoves(int x, int y, int color, Chessboard moveGenerator, ArrayList<Move> moves) {

        long whitePieces = moveGenerator.getWhitePieces();
        long blackPieces = moveGenerator.getBlackPieces();

        // up
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 0, -1,ROOK*color);
        // down
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 0, 1,ROOK*color);
        // right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, 0,ROOK*color);
        // left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, 0,ROOK*color);

        return moves;
    }


}
