package ChessNetwork.Pieces;

import ChessNetwork.Chessboard;

import java.util.ArrayList;

import static ChessNetwork.BoardUtils.*;

public class Queen {


    public static ArrayList<Move> getMoves(int x, int y, int color, Chessboard moveGenerator) {
        ArrayList<Move> moves = new ArrayList<>();
        long whitePieces = moveGenerator.getWhitePieces();
        long blackPieces = moveGenerator.getBlackPieces();

        // up
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 0, -1,QUEEN*color);
        // down
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 0, 1,QUEEN*color);
        // right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, 0,QUEEN*color);
        // left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, 0,QUEEN*color);
        // up right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, -1,QUEEN*color);
        // up left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, -1,QUEEN*color);
        // down right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, 1,QUEEN*color);
        // down left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, 1,QUEEN*color);
        return moves;
    }

}