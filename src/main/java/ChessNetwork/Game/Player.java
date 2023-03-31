package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.MoveGenerator;

public abstract class Player {
    //no constructor, because we won't be creating instances of this class, only of its subclasses
    protected int color;
    protected ChessGame chessGame;
    public int getColor() {
        return color;
    }

    public abstract void awaitMove(int color, MoveGenerator moveGenerator);

    public abstract void init(ChessGame chessGame, int color);
}
