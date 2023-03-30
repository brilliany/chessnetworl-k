package Neuralnetwork;


import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;

import java.io.Serializable;
import java.util.Random;

import static ChessNetwork.ChessboardHelper.makeMoveSilent;

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

    public int[] chooseMove(int[][][] boardState, int color, int depth, MoveGenerator moveGenerator) {
        int[][][][] boards = getBoards(depth, boardState, color, moveGenerator);
        double[] inputLayer = createInputLayer(boards, depth, color);
        double[] outputLayer = forwardPropagate(inputLayer);
        double maxScore = Double.NEGATIVE_INFINITY;
        int[] bestMove = null;

        for (Move move : moveGenerator.getAllMoves(color, boardState)) {
            int[] moveCoords = new int[]{move.getFromX(), move.getFromY(), move.getToX(), move.getToY()};
            double score = getMoveScore(move, outputLayer);
            if (score > maxScore) {
                maxScore = score;
                bestMove = moveCoords;
            }
        }

        return bestMove;
    }

    private int[][][][] getBoards(int depth, int[][][] boardState, int color, MoveGenerator moveGenerator) {
        int[][][][] boards = new int[depth][8][8][2];
        boards[0] = boardState;
        for (int i = 1; i < depth; i++) {
            boards[i] = getBoardsHelper(boards[i - 1], color, moveGenerator);
        }
        return boards;
    }

    private int[][][] getBoardsHelper(int[][][] board, int color, MoveGenerator moveGenerator) {
        int[][][] newBoard = new int[8][8][2];
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                newBoard[i][j][0] = board[i][j][0];
                newBoard[i][j][1] = board[i][j][1];
            }
        }
        for (Move move : moveGenerator.getAllMoves(color, board)) {
            makeMoveSilent(move,newBoard);
        }
        return newBoard;
    }

    private double[] forwardPropagate(double[] inputLayer) {
        double[] hiddenLayer = new double[hiddenSize];
        double[] outputLayer = new double[outputSize];

        // Calculate hidden layer
        for (int i = 0; i < hiddenSize; i++) {
            double sum = 0;
            for (int j = 0; j < inputSize; j++) {
                sum += inputLayer[j] * inputWeights[j][i];
            }
            sum += hiddenBiases[i];
            hiddenLayer[i] = sigmoid(sum);
        }

        // Calculate output layer
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

    private double getMoveScore(Move move, double[] outputLayer) {
        int startRow = move.getFromY();
        int startCol = move.getFromX();
        int endRow = move.getToY();
        int endCol = move.getToX();

        int startIndex = startRow * 8 + startCol;
        int endIndex = endRow * 8 + endCol;
        int promotionIndex = 64 + (endCol * 4) + (endRow == 7 ? 0 : 1);


        double captureValue = move.isCapture() ? captureScore : 0;
        double promotionValue = isPromotion(move) ? promotionScore : 0;
        double startIndexValue = outputLayer[startIndex];
        double endIndexValue = outputLayer[endIndex];
        double promotionIndexValue = outputLayer[promotionIndex];

        double score = captureValue + promotionValue + startIndexValue + endIndexValue + promotionIndexValue;
        System.out.println("Move: " + move + " Score: " + score);
        return score;
    }

    private boolean isPromotion(Move move) {
        int endRow = move.getToY();
        int[] piece = move.getPiece();
        if (piece[0] == 1 && endRow == 7) {
            return true;
        } else return piece[0] == -1 && endRow == 0;
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
        int[] pieceValues = new int[]{0,1, 2, 3, 4, 5, 6};
        int[] pieceWeights = new int[]{0, 1, 3, 3, 5, 9};
        int[] pieceSquares = new int[7];

        for (int i = 0; i < depth; i++) {
            for (int j = 0; j < 8; j++) {
                for (int k = 0; k < 8; k++) {
                    int pieceType = Math.abs(boards[i][j][k][0]);
                    int pieceColor = boards[i][j][k][1];
                    if (pieceType != 0) {
                        pieceSquares[pieceType] += (pieceColor == color) ? 1 : -1;
                    }
                }
            }
        }

        for (int i = 0; i < pieceValues.length; i++) {
            inputLayer[i] = pieceValues[i] * pieceSquares[i];
        }

        return inputLayer;
    }

    private double sigmoid(double x) {
        return 1 / (1 + Math.exp(-x));
    }
}