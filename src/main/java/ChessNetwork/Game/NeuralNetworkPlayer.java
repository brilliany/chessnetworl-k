package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;
import Neuralnetwork.ChessNeuralNetwork;
import Neuralnetwork.NeuralNetwork;
import org.jetbrains.annotations.Nullable;

public class NeuralNetworkPlayer extends Player {
    private final NeuralNetwork neuralNetwork;
    private final int depth;
    public NeuralNetworkPlayer(Integer depth) {
        super();
        this.depth = depth;
        this.neuralNetwork = ChessNeuralNetwork.loadNetwork("bestnn2");
    }



    public Move getMove(int[][][] boardState, int color, MoveGenerator moveGenerator, @Nullable int[] pawnWhichIsEnPassantable) {
        int[] moveAsInt = neuralNetwork.chooseMove(boardState, color,depth, moveGenerator, pawnWhichIsEnPassantable);
        return new Move(moveAsInt[0], moveAsInt[1], moveAsInt[2], moveAsInt[3], boardState[moveAsInt[1]][moveAsInt[0]]);
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
    }
}
