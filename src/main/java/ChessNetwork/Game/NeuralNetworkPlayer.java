package ChessNetwork.Game;

import ChessNetwork.ChessGame;
import ChessNetwork.Chessboard;
import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;


public class NeuralNetworkPlayer extends Player {
/*    private final NeuralNetwork neuralNetwork;*/
    private final int depth;
    public NeuralNetworkPlayer(Integer depth) {
        super();
        this.depth = depth;
/*        this.neuralNetwork = ChessNeuralNetwork.loadNetwork("bestnn2");*/
    }



    public Move getMove(int[][][] boardState, int color, MoveGenerator moveGenerator) {
/*        int[] moveAsInt = neuralNetwork.chooseMove(boardState, color,depth, moveGenerator);
        return new Move(moveAsInt[0], moveAsInt[1], moveAsInt[2], moveAsInt[3], boardState[moveAsInt[1]][moveAsInt[0]]);*/
        return null;
    }

    @Override
    public void awaitMove(int color, Chessboard chessboard) {
/*        //call listener with the move
        Move move = getMove(boardState, color, moveGenerator);

        moveGenerator.makeMove(move, boardState);*/
    }

    @Override
    public void init(ChessGame chessGame, int color) {
        this.chessGame = chessGame;
    }
}
