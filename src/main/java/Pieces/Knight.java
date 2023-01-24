package Pieces;

import lombok.Getter;
import lombok.Setter;

import java.util.ArrayList;
import java.util.List;

public class Knight {
    @Getter
    @Setter
    private int x;
    @Getter@Setter
    private int y;
    private final int color;
    public final int WHITE = 0;
    public final int BLACK = 1;
    public Knight(int x, int y, int color) {
        this.x = x;
        this.y = y;
        this.color = color;
    }

    public int getColor() {
        return color;
    }

    public List<int[]> getMoves(int[][] chessboard) {
        List<int[]> moves = new ArrayList<>();
        int[][] possibleMoves = {{-2, -1}, {-2, 1}, {-1, -2}, {-1, 2}, {1, -2}, {1, 2}, {2, -1}, {2, 1}};
        for (int[] move : possibleMoves) {
            int[] newPos = {x + move[0], y + move[1]};
            if (isValidMove(newPos, chessboard)) {
                int[] m = {x, y, newPos[0], newPos[1]};
                moves.add(m);
            }
        }
        return moves;
    }

    private boolean isValidMove(int[] move, int[][] chessboard) {
        if (move[0] < 0 || move[0] >= 8 || move[1] < 0 || move[1] >= 8) {
            return false;
        }
        int piece = chessboard[move[0]][move[1]];
        return piece == 0 || Math.signum(piece) != color;
    }
}