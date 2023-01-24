package Pieces;

import lombok.Getter;
import lombok.Setter;

import java.util.ArrayList;
import java.util.List;

public class Pawn {
    @Getter@Setter
    private int x;
    @Getter@Setter
    private int y;
    private final int color;
    public final int WHITE = 0;
    public final int BLACK = 1;
    public Pawn(int x, int y, int color) {
        this.x = x;
        this.y = y;
        this.color = color;
    }

    public int getColor() {
        return color;
    }

    public List<int[]> getMoves(int[][] chessboard) {
        List<int[]> moves = new ArrayList<>();
        int direction = color == WHITE ? -1 : 1;
        // check if the pawn can move one step forward
        if(x + direction >= 0 && x + direction < 8 && chessboard[x + direction][y] == 0) {
            //add the move to the list of possible moves
            moves.add(new int[] {x, y, x+direction, y});
        }
        // check if the pawn can move two steps forward
        if(x == (color == WHITE ? 6 : 1) && chessboard[x + direction*2][y] == 0) {
            //add the move to the list of possible moves
            moves.add(new int[] {x, y, x+direction*2, y});
        }
        // check if the pawn can capture to the left
        int[] captureLeft = {x + direction, y - 1};
        if(y-1 >= 0 && x + direction >= 0 && x + direction < 8 && chessboard[x + direction][y-1] != 0 && (chessboard[x + direction][y-1] < 0) != (color == WHITE)) {
            //add the move to the list of possible moves
            moves.add(new int[] {x, y, captureLeft[0], captureLeft[1]});
        }
        // check if the pawn can capture to the right
        int[] captureRight = {x + direction, y + 1};
        if(y+1 < 8 && x + direction >= 0 && x + direction < 8 && chessboard[x + direction][y+1] != 0 && (chessboard[x + direction][y+1] < 0) != (color == WHITE)) {
            //add the move to the list of possible moves
            moves.add(new int[] {x, y, captureRight[0], captureRight[1]});
        }
        return moves;
    }
}
