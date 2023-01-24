package Pieces;

import lombok.Getter;
import lombok.Setter;

import java.util.ArrayList;
import java.util.List;

public class Queen {
    @Getter
    @Setter
    private int x;
    @Getter@Setter
    private int y;
    private final int color;
    public final int WHITE = 0;
    public final int BLACK = 1;

    public Queen(int x, int y, int color) {
        this.x = x;
        this.y = y;
        this.color = color;
    }

    public int getColor() {
        return color;
    }

    public List<int[]> getMoves(int[][] chessboard) {
        List<int[]> moves = new ArrayList<>();

        // Add moves to the left
        int[] left = {x, y - 1};
        while (isValidMove(left, chessboard)) {
            int[] move = {x, y, left[0], left[1]};
            moves.add(move);
            left[1]--;
        }
        // Add moves to the right
        int[] right = {x, y + 1};
        while (isValidMove(right, chessboard)) {
            int[] move = {x, y, right[0], right[1]};
            moves.add(move);
            right[1]++;
        }
        // Add moves to the up
        int[] up = {x - 1, y};
        while (isValidMove(up, chessboard)) {
            int[] move = {x, y, up[0], up[1]};
            moves.add(move);
            up[0]--;
        }
        // Add moves to the down
        int[] down = {x + 1, y};
        while (isValidMove(down, chessboard)) {
            int[] move = {x, y, down[0], down[1]};
            moves.add(move);
            down[0]++;
        }
// Add moves to the top-left
        int[] topLeft = {x - 1, y - 1};
        while (isValidMove(topLeft, chessboard)) {
            int[] move = {x, y, topLeft[0], topLeft[1]};
            moves.add(move);
            topLeft[0]--;
            topLeft[1]--;
        }
// Add moves to the top-right
        int[] topRight = {x - 1, y + 1};
        while (isValidMove(topRight, chessboard)) {
            int[] move = {x, y, topRight[0], topRight[1]};
            moves.add(move);
            topRight[0]--;
            topRight[1]++;
        }
// Add moves to the bottom-left
        int[] bottomLeft = {x + 1, y - 1};
        while (isValidMove(bottomLeft, chessboard)) {
            int[] move = {x, y, bottomLeft[0], bottomLeft[1]};
            moves.add(move);
            bottomLeft[0]++;
            bottomLeft[1]--;
        }
// Add moves to the bottom-right
        int[] bottomRight = {x + 1, y + 1};
        while (isValidMove(bottomRight, chessboard)) {
            int[] move = {x, y, bottomRight[0], bottomRight[1]};
            moves.add(move);
            bottomRight[0]++;
            bottomRight[1]++;
        }
        return moves;
    }

    private boolean isValidMove(int[] move, int[][] chessboard) {
        int xDiff = Math.abs(move[0] - x);
        int yDiff = Math.abs(move[1] - y);
        if (xDiff == 0 || yDiff == 0 || xDiff == yDiff) {
            // move is either horizontal, vertical, or diagonal
            // check if the path to the move is clear
            int xStep = Integer.compare(move[0], x);
            int yStep = Integer.compare(move[1], y);
            int[] nextMove = {x + xStep, y + yStep};
            while (!(nextMove[0] == move[0] && nextMove[1] == move[1])) {
                if (chessboard[nextMove[0]][nextMove[1]] != 0) {
                    // path is not clear
                    return false;
                }
                nextMove[0] += xStep;
                nextMove[1] += yStep;
            }
            // check if the move is within the chessboard
            return move[0] >= 0 && move[0] < 8 && move[1] >= 0 && move[1] < 8;
        } else {
            // move is not valid
            return false;
        }
    }
}