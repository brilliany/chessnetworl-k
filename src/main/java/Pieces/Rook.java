package Pieces;

import lombok.Getter;
import lombok.Setter;

import java.util.ArrayList;
import java.util.List;

public class Rook {
    @Getter
    @Setter
    private int x;
    @Getter@Setter

    private int y;
    private final int color;
    public final int WHITE = 0;
    public final int BLACK = 1;
    public Rook(int x, int y, int color) {
        this.x = x;
        this.y = y;
        this.color = color;
    }

    public int getColor() {
        return color;
    }

    public List<int[]> getMoves(int[][] chessboard) {
        List<int[]> moves = new ArrayList<>();

        // Move up
        for (int i = x - 1; i >= 0; i--) {
            int[] move = {x, y, i, y};
            if (isValidMove(move, chessboard)) {
                moves.add(move);
            } else {
                break;
            }
        }

        // Move down
        for (int i = x + 1; i < 8; i++) {
            int[] move = {x, y, i, y};
            if (isValidMove(move, chessboard)) {
                moves.add(move);
            } else {
                break;
            }
        }

        // Move left
        for (int j = y - 1; j >= 0; j--) {
            int[] move = {x, y, x, j};
            if (isValidMove(move, chessboard)) {
                moves.add(move);
            } else {
                break;
            }
        }

        // Move right
        for (int j = y + 1; j < 8; j++) {
            int[] move = {x, y, x, j};
            if (isValidMove(move, chessboard)) {
                moves.add(move);
            } else {
                break;
            }
        }

        return moves;
    }

    private boolean isValidMove(int[] move, int[][]
            chessboard) {
        return move[0] >= 0 && move[0] < 8 && move[1] >= 0 && move[1] < 8 && chessboard[move[0]][move[1]] == 0;
    }
}
