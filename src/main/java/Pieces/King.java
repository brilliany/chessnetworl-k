package Pieces;

import lombok.Getter;
import lombok.Setter;

import java.util.ArrayList;
import java.util.List;

public class King {
    @Getter
    @Setter
    private int x;
    @Getter
    @Setter
    private int y;
    private final int color;
    public final int WHITE = 0;
    public final int BLACK = 1;
    private boolean hasMoved = false;

    public King(int x, int y, int color) {
        this.x = x;
        this.y = y;
        this.color = color;
    }

    public int getColor() {
        return color;
    }

    public List<int[]> getMoves(int[][] chessboard) {
        List<int[]> moves = new ArrayList<>();
        for (int i = -1; i <= 1; i++) {
            for (int j = -1; j <= 1; j++) {
                if(i==0 && j==0) continue; //skip the case where i and j are both zero
                int[] move = {x + i, y + j};
                if (isValidMove(move, chessboard)) {
                    moves.add(new int[] {x, y, x + i, y + j});
                }
            }
        }
// Handle castling
        int[] castleKingSide = {x, y + 2};
        if (isValidCastleKingSide(castleKingSide, chessboard)) {
            moves.add(new int[] {x, y, x, y + 2});
        }
        int[] castleQueenSide = {x, y - 2};
        int[] castleQueenSide2 = {x, y - 3};
        if (isValidCastleQueenSide(castleQueenSide, chessboard) && isValidCastleQueenSide(castleQueenSide2, chessboard)) {
            moves.add(new int[] {x, y, x, y - 2});
        }
        return moves;
    }

    private boolean isValidMove(int[] move, int[][] chessboard) {
        if (move[0] < 0 || move[0] > 7 || move[1] < 0 || move[1] > 7) {
            return false;
        }
        return chessboard[move[0]][move[1]] == 0 || Math.signum(chessboard[move[0]][move[1]]) != color;
    }

    private boolean isValidCastleKingSide(int[] move, int[][] chessboard) {
        // check if the squares in between the king and the rook are empty and the rook has not moved
        return move[1] == y + 2 && chessboard[x][y + 1] == 0 && chessboard[x][y + 2] == 0 && chessboard[x][7] > 0 && Math.signum(chessboard[x][7]) == color && !hasMoved;
    }

    private boolean isValidCastleQueenSide(int[] move, int[][] chessboard) {
        // check if the squares in between the king and the rook are empty and the rook has not moved
        return move[1] == y - 2 && chessboard[x][y - 1] == 0 && chessboard[x][y - 2] == 0 && chessboard[x][0] > 0 && Math.signum(chessboard[x][0]) == color && !hasMoved;

    }
}