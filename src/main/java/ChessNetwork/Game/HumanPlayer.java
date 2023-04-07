package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.Chessboard;

public class HumanPlayer extends Player {

    public HumanPlayer() {
        super();
    }

    @Override
    public void awaitMove(int color, Chessboard chessboard) {
        //do nothing, because the move will be made by the user
    }

    @Override
    public void init(ChessGame chessGame, int color) {
        this.chessGame = chessGame;
        this.color = color;
    }
}
