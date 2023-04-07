package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.Chessboard;

public abstract class Player {
    //no constructor, because we won't be creating instances of this class, only of its subclasses
    protected int color;
    protected ChessGame chessGame;
    public int getColor() {
        return color;
    }

    public abstract void awaitMove(int color, Chessboard chessboard);

    public abstract void init(ChessGame chessGame, int color);
}
