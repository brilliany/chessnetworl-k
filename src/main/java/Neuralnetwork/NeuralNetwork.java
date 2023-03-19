package Neuralnetwork;


import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;
import org.jetbrains.annotations.Nullable;

import java.io.Serializable;
import java.util.Random;

public class NeuralNetwork implements Serializable {
    private final int inputSize;
    private final int hiddenSize;
    private final int outputSize;
    private double[][] inputWeights;
    private double[][] hiddenWeights;
    private double[][] outputWeights;
    private double[] inputBiases;
    private double[] hiddenBiases;
    private double[] outputBiases;
    private int color;
    //score for if the next move is a capture
    private double captureScore;
    private double promotionScore;



    private final double mutationRate = 0.1;
    private double fitness = 0;
    private final int mutationStrength = 10;

    public NeuralNetwork(int inputSize, int hiddenSize) {
        this.inputSize = inputSize;
        this.hiddenSize = hiddenSize;
        this.outputSize = 4;

        // Initialize inputWeights, hiddenWeights, outputWeights, inputBiases, hiddenBiases, and outputBiases
        this.inputWeights = new double[inputSize][hiddenSize];
        this.hiddenWeights = new double[hiddenSize][outputSize];
        this.outputWeights = new double[hiddenSize][outputSize];
        this.inputBiases = new double[inputSize];
        this.hiddenBiases = new double[hiddenSize];
        this.outputBiases = new double[outputSize];

        // Initialize inputWeights and hiddenWeights with random values between -1 and 1
        Random rand = new Random();
        for (int i = 0; i < inputSize; i++) {
            for (int j = 0; j < hiddenSize; j++) {
                this.inputWeights[i][j] = rand.nextDouble() * 2 - 1;
            }
        }
        for (int i = 0; i < hiddenSize; i++) {
            for (int j = 0; j < outputSize; j++) {
                this.hiddenWeights[i][j] = rand.nextDouble() * 2 - 1;
            }
        }

        // Initialize inputBiases, hiddenBiases, and outputBiases with random values between -1 and 1
        for (int i = 0; i < inputSize; i++) {
            this.inputBiases[i] = rand.nextDouble() * 2 - 1;
        }
        for (int i = 0; i < hiddenSize; i++) {
            this.hiddenBiases[i] = rand.nextDouble() * 2 - 1;
        }
        for (int i = 0; i < outputSize; i++) {
            this.outputBiases[i] = rand.nextDouble() * 2 - 1;
        }
        //initialize scores and penalties
        captureScore = rand.nextDouble() * 2 - 1;
        promotionScore = rand.nextDouble() * 2 - 1;

    }

    public int getColor() {
        return this.color;
    }

    public void setColor(int c) {
        this.color = c;
    }

    public void setInputWeights(double[][] inputWeights) {
        this.inputWeights = inputWeights;
    }

    public double[][] getInputWeights() {
        return inputWeights;
    }

    public void setHiddenWeights(double[][] hiddenWeights) {
        this.hiddenWeights = hiddenWeights;
    }

    public double[][] getHiddenWeights() {
        return hiddenWeights;
    }

    public void setHiddenBiases(double[] hiddenBiases) {
        this.hiddenBiases = hiddenBiases;
    }

    public double[] getHiddenBiases() {
        return hiddenBiases;
    }

    public void setOutputBiases(double[] outputBiases) {
        this.outputBiases = outputBiases;
    }

    public double[] getOutputBiases() {
        return outputBiases;
    }

    public void setFitness(double fitness) {
        this.fitness = fitness;
    }

    public double getFitness() {
        return fitness;
    }

    public void resetFitness() {
        fitness = 0;
    }

    public void incrementFitness() {
        fitness++;
    }

    public void setOutputWeights(double[][] outputWeights) {
        this.outputWeights = outputWeights;
    }

    public double[][] getOutputWeights() {
        return outputWeights;
    }

    public void setInputBiases(double[] inputBiases) {
        this.inputBiases = inputBiases;
    }
    public double[] getInputBiases() {
        return inputBiases;
    }

    public void mutate() {
        Random rand = new Random();
        int mutationChance = rand.nextInt(100);
        if (mutationChance < mutationRate) {
            int type = rand.nextInt(3);
            switch (type) {
                case 0:
                    int i = rand.nextInt(inputSize);
                    int j = rand.nextInt(hiddenSize);
                    inputWeights[i][j] += mutationStrength * rand.nextDouble() - mutationStrength / 2D;
                    break;
                case 1:
                    i = rand.nextInt(hiddenSize);
                    j = rand.nextInt(outputSize);
                    hiddenWeights[i][j] += mutationStrength * rand.nextDouble() - mutationStrength / 2D;
                    break;
                case 2:
                    i = rand.nextInt(outputSize);
                    outputBiases[i] += mutationStrength * rand.nextDouble() - mutationStrength / 2D;
                    break;
            }
        }
    }

    public int[] chooseMove(int[][][] boardState, int color, int depth, MoveGenerator moveGenerator, @Nullable int[] pawnWhichIsEnPassantable) {
        int[][][][] boards = getBoards(depth, boardState, color, moveGenerator, pawnWhichIsEnPassantable);
        double[] inputLayer = createInputLayer(boards, depth, color);

        Move[] moves = moveGenerator.getAllMoves(color, boardState, pawnWhichIsEnPassantable);
        if (moves.length == 0) {
            return new int[] {-1, -1, -1, -1};
        }

        double[] hiddenLayer = calculateHiddenLayer(inputLayer);
        double[] outputLayer = calculateOutputLayer(hiddenLayer);
        int bestTimeline = 0;
        double bestValue = 0;


        for (int i = 0; i < outputLayer.length; i++) {
            if (outputLayer[i] > bestValue) {
                bestValue = outputLayer[i];
                bestTimeline = i / moves.length;
            }
        }
        // check for out of bounds for all of these
        //get the best move from the output layer
        return getBestMove(outputLayer, bestTimeline, moves);
    }

    private int[] getBestMove(double[] outputLayer, int bestTimeline, Move[] moves) {
        int bestMoveIndex = 0;
        double bestValue = 0;
        for (int i = 0; i < moves.length; i++) {
            if (outputLayer[i + (bestTimeline * moves.length)] > bestValue) {
                bestValue = outputLayer[i + (bestTimeline * moves.length)];
                bestMoveIndex = i;
            }
        }
        return getMoveFromIndex(bestMoveIndex, moves);
    }



    private int[] getMoveFromIndex(int bestMoveIndex, Move[] moves) {
        int[] move = new int[4];
        move[0] = moves[bestMoveIndex].getFromY();
        move[1] = moves[bestMoveIndex].getFromX();
        move[2] = moves[bestMoveIndex].getToY();
        move[3] = moves[bestMoveIndex].getToX();
        return move;
    }

    private double[] calculateOutputLayer(double[] hiddenLayer) {
        double[] outputLayer = new double[outputSize];
        for (int i = 0; i < outputSize; i++) {
            double sum = 0;
            for (int j = 0; j < hiddenSize; j++) {
                sum += hiddenLayer[j] * hiddenWeights[j][i];
            }
            sum += outputBiases[i];
            outputLayer[i] = sigmoid(sum);
        }
        return outputLayer;
    }

    private double[] calculateHiddenLayer(double[] inputLayer) {
        double[] hiddenLayer = new double[hiddenSize];
        for (int i = 0; i < hiddenSize; i++) {
            double sum = 0;
            for (int j = 0; j < inputSize; j++) {
                sum += inputLayer[j] * inputWeights[j][i];
            }
            sum += inputBiases[i];
            hiddenLayer[i] = sigmoid(sum);
        }
        return hiddenLayer;
    }

    private double[] createInputLayer(int[][][][] boards, int depth, int color) {
        double[] inputLayer = new double[inputSize];
        int index = 0;
        for (int i = 0; i < depth; i++) {
            for (int j = 0; j < 8; j++) {
                for (int k = 0; k < 8; k++) {
                    if (boards[i][j][k][0] == color) {
                        inputLayer[index] = 1;
                    } else if (boards[i][j][k][0] == -color) {
                        inputLayer[index] = -1;
                    } else {
                        inputLayer[index] = 0;
                    }
                    index++;
                }
            }
        }
        return inputLayer;
    }

    private int[][][][] getBoards(int depth, int[][][] boardState, int color, MoveGenerator moveGenerator, @Nullable int[] pawnWhichIsEnPassantable) {
        int[][][][] futureBoards = new int[depth][8][8][2];
        // Calculate all future boards until the given depth
        // Color in this case is the color of the player whose turn it is
        // board[y][x] is the coordinate system
        // store the pieces as integers instead of Pieces to save memory with the piecetype enum
        //this method should just return the boards, the best sequences are calculated later
        // loop through all the moves that can be made

        //copy the current board into the first board in the array
        for (int i = 0; i < 8; i++) {
            System.arraycopy(boardState[i], 0, futureBoards[0][i], 0, 8);
        }

        for (int i = 1; i < depth; i++) {
            //copy the previous board into the current board
            for (int j = 0; j < 8; j++) {
                System.arraycopy(futureBoards[i - 1][j], 0, futureBoards[i][j], 0, 8);
            }
            //loop through all the moves that can be made
            Move[] moves = moveGenerator.getAllMoves(color, futureBoards[i], pawnWhichIsEnPassantable);
            for (Move move : moves) {
                //move the piece
                futureBoards[i][move.getToY()][move.getToX()][0] = futureBoards[i][move.getFromY()][move.getFromX()][0];
                futureBoards[i][move.getToY()][move.getToX()][1] = futureBoards[i][move.getFromY()][move.getFromX()][1];
                //remove the piece from the old position
                futureBoards[i][move.getFromY()][move.getFromX()][0] = 0;
                futureBoards[i][move.getFromY()][move.getFromX()][1] = 0;
            }
        }

        return futureBoards;
    }

    public static double sigmoid(double x) {
        return 1 / (1 + Math.exp(-x));
    }

}
