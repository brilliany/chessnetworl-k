package ChessNetwork.Pieces;

import ChessNetwork.MoveGenerator;

import java.util.ArrayList;

import static ChessNetwork.ChessboardHelper.*;

public class Rook {

    public static Move[] getMoves(int x, int y, int color,MoveGenerator moveGenerator) {

        ArrayList<Move> moves = new ArrayList<>();
        long whitePieces = moveGenerator.getWhitePieces();
        long blackPieces = moveGenerator.getBlackPieces();

        // up
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 0, -1);
        // down
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 0, 1);
        // right
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, 1, 0);
        // left
        addMovesInDirection(moves, blackPieces, whitePieces, color, x, y, -1, 0);

        return moves.toArray(new Move[0]);
    }


}
