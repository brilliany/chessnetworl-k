package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.Chessboard;
import ChessNetwork.Pieces.Move;

public class BotPlayer extends Player {
    private final ChessBot.ChessBot bot;

    public BotPlayer(int depth, int color, Chessboard chessboard) {
        super();
        this.chessBoard = chessboard;
        this.bot = new ChessBot.ChessBot(chessboard, depth, color);
    }

    public Move getMove(int color, Chessboard chessboard) {
        return bot.getBestMove(chessboard);
    }

    @Override
    public void awaitMove(int color, Chessboard chessboard) {
        //call listener with the move
        Move move = getMove(color, chessboard);
        chessboard.makeMove(move);
    }


    @Override
    public void init(ChessGame chessGame) {
        this.chessGame = chessGame;
    }


}
