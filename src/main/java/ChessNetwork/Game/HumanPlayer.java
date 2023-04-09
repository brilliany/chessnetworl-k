package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.Chessboard;

public class HumanPlayer extends Player {

    public HumanPlayer(int color, Chessboard chessboard) {
        super();
        this.chessBoard = chessboard;
        this.color = color;
    }

    @Override
    public void awaitMove(int color, Chessboard chessboard) {
        //do nothing, because the move will be made by the user
    }

    @Override
    public void init(ChessGame chessGame) {
        this.chessGame = chessGame;
    }
}
