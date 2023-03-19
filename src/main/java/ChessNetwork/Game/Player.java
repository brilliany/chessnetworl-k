package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.MoveGenerator;
import org.jetbrains.annotations.Nullable;

public abstract class Player {
    //no constructor, because we won't be creating instances of this class, only of its subclasses
    protected int color;
    protected ChessGame chessGame;
    public int getColor() {
        return color;
    }

    public abstract void awaitMove(int[][][] boardState, int color, MoveGenerator moveGenerator, @Nullable int[] pawnWhichIsEnPassantable);

    public abstract void init(ChessGame chessGame, int color);
}
