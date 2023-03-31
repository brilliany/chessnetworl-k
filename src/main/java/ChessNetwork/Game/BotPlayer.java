package ChessNetwork.Game;

import ChessBot.PieceTables;
import ChessNetwork.ChessGame;
import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;

public class BotPlayer extends Player {
    private final ChessBot.ChessBot bot;

    public BotPlayer(int depth, PieceTables pieceTables,int color, MoveGenerator moveGenerator) {
        super();
        this.bot = new ChessBot.ChessBot(depth, pieceTables, color, moveGenerator);
    }

    public Move getMove(int color, MoveGenerator moveGenerator) {
        return bot.getBestMove();
    }

    @Override
    public void awaitMove(int color, MoveGenerator moveGenerator) {
        //call listener with the move
        Move move = getMove(color, moveGenerator);
        moveGenerator.makeMove(move);
    }


    @Override
    public void init(ChessGame chessGame, int color) {
        this.chessGame = chessGame;
        this.color = color;
    }


}
