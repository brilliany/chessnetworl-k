package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;
import org.jetbrains.annotations.Nullable;

public class BotPlayer extends Player {
    private final ChessBot.ChessBot bot;

    public BotPlayer(Integer depth) {
        super();
        this.bot = new ChessBot.ChessBot(depth.intValue());
    }

    public Move getMove(int[][][] boardState, int color, MoveGenerator moveGenerator, @Nullable int[] pawnWhichIsEnPassantable) {
        return bot.getBestMove(color, moveGenerator, pawnWhichIsEnPassantable);
    }

    @Override
    public void awaitMove(int[][][] boardState, int color, MoveGenerator moveGenerator, @Nullable int[] pawnWhichIsEnPassantable) {
        //call listener with the move
        Move move = getMove(boardState, color, moveGenerator, pawnWhichIsEnPassantable);
        moveGenerator.makeMove(move, boardState);
    }


    @Override
    public void init(ChessGame chessGame, int color) {
        this.chessGame = chessGame;
        this.color = color;
    }


}
