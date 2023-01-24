package Pieces;


import lombok.Getter;
import lombok.Setter;

import java.util.ArrayList;
import java.util.List;

public class Bishop {
    @Getter
    @Setter
    private int x;
    @Getter@Setter
    private int y;
    private final int color;
    public final int WHITE = 0;
    public final int BLACK = 1;
    public Bishop(int x, int y, int color) {
        this.x = x;
        this.y = y;
        this.color = color;
    }

    public int getColor() {
        return color;
    }

    public List<int[]> getMoves(int[][] chessboard) {
        List<int[]> moves = new ArrayList<>();
// Move diagonally up and to the left
        for (int i = 1; x - i >= 0 && y - i >= 0; i++) {
            if (chessboard[x - i][y - i] == 0) {
                int[] move = {x, y, x - i, y - i}; // <-- change here
                moves.add(move);
            } else if (Math.signum(chessboard[x - i][y - i]) != color) {
                int[] move = {x, y, x - i, y - i}; // <-- change here
                moves.add(move);
                break;
            } else {
                break;
            }
        }
// Move diagonally up and to the right
        for (int i = 1; x - i >= 0 && y + i < 8; i++) {
            if (chessboard[x - i][y + i] == 0) {
                int[] move = {x, y, x - i, y + i}; // <-- change here
                moves.add(move);
            } else if (Math.signum(chessboard[x - i][y + i]) != color) {
                int[] move = {x, y, x - i, y + i}; // <-- change here
                moves.add(move);
                break;
            } else {
                break;
            }
        }
// Move diagonally down and to the left
        for (int i = 1; x + i < 8 && y - i >= 0; i++) {
            if (chessboard[x + i][y - i] == 0) {
                int[] move = {x, y, x + i, y - i}; // <-- change here
                moves.add(move);
            } else if (Math.signum(chessboard[x + i][y - i]) != color) {
                int[] move = {x, y, x + i, y - i}; // <-- change here
                moves.add(move);
                break;
            } else {
                break;
            }
        }
// Move diagonally down and to the right
        for (int i = 1; x + i < 8 && y + i < 8; i++) {
            if (chessboard[x + i][y + i] == 0) {
                int[] move = {x, y, x + i, y + i}; // <-- change here
                moves.add(move);
            } else if (Math.signum(chessboard[x + i][y + i]) != color) {
                int[] move = {x, y, x + i, y + i}; // <-- change here
                moves.add(move);
                break;
            } else {
                break;
            }
        }
        return moves;
    }
}
