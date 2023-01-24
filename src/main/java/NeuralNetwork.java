


import java.io.Serializable;
import java.util.Arrays;
import java.util.Random;

public class NeuralNetwork implements Serializable {
    private double[][] inputWeights;
    private double[] hiddenBiases;
    private double[][] hiddenWeights;
    private double[] outputBiases;
    private double fitness;
    private double[][] outputWeights;
    private double[] inputBiases;

    int color;
    private static final double mutationRate = 10; // 1% chance of mutation
    private static final double mutationStrength = 0.2; // mutation strength of 10%

    private final int inputSize;
    private final int hiddenSize;
    private final int outputSize;

    public NeuralNetwork(int inputSize, int hiddenSize, int outputSize) {
        this.inputSize = inputSize;
        this.hiddenSize = hiddenSize;
        this.outputSize = outputSize;

        // Initialize inputWeights, hiddenWeights, outputWeights, inputBiases, hiddenBiases, and outputBiases
        this.inputWeights = new double[inputSize][hiddenSize];
        this.hiddenWeights = new double[hiddenSize][outputSize];
        this.outputWeights = new double[hiddenSize][outputSize];
        this.inputBiases = new double[inputSize];
        this.hiddenBiases = new double[hiddenSize];
        this.outputBiases = new double[outputSize];
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
        Random r = new Random();
        for (int i = 0; i < inputWeights.length; i++) {
            for (int j = 0; j < inputWeights[i].length; j++) {
                if (r.nextDouble() < mutationRate) {
                    inputWeights[i][j] += r.nextGaussian() * mutationStrength;
                }
            }
        }
        for (int i = 0; i < hiddenWeights.length; i++) {
            for (int j = 0; j < hiddenWeights[i].length; j++) {
                if (r.nextDouble() < mutationRate) {
                    hiddenWeights[i][j] += r.nextGaussian() * mutationStrength;
                }
            }
        }
        for (int i = 0; i < hiddenBiases.length; i++) {
            if (r.nextDouble() < mutationRate) {
                hiddenBiases[i] += r.nextGaussian() * mutationStrength;
            }
        }
        for (int i = 0; i < outputBiases.length; i++) {
            if (r.nextDouble() < mutationRate) {
                outputBiases[i] += r.nextGaussian() * mutationStrength;
            }
        }
    }

    public int[] chooseMove(double[] boardState, MoveGenerator moveGenerator, int color) {
        // Feed forward through the network
        double[] hidden = new double[hiddenSize];
        for (int i = 0; i < hiddenSize; i++) {
            for (int j = 0; j < inputSize; j++) {
                hidden[i] += boardState[j] * inputWeights[j][i];
            }
            hidden[i] += hiddenBiases[i];
            hidden[i] = sigmoid(hidden[i]);
        }
        double[] output = new double[outputSize];
        for (int i = 0; i < outputSize; i++) {
            for (int j = 0; j < hiddenSize; j++) {
                output[i] += hidden[j] * hiddenWeights[j][i];
            }
            output[i] += outputBiases[i];
            output[i] = sigmoid(output[i]);
        }

        // Get all legal moves
        int[][] legalMoves = moveGenerator.getAllMoves(color, moveGenerator.getChessboard());
        double bestScore = Double.NEGATIVE_INFINITY;
        int[] bestMove = new int[4];
        // Choose the move with the highest output value and agression score
        for(int i = 0; i < legalMoves.length; i++) {
            int[] move = legalMoves[i];
            double moveScore = output[move[2] * 8 + move[3]] + moveGenerator.calculateAggressionScore(move[0], move[1], move[2], move[3], color);
            if(moveScore > bestScore) {
                bestScore = moveScore;
                bestMove = move;
            }
        }
//return the best move
        System.out.println("Best move: " + Arrays.toString(bestMove));
        return bestMove;
    }
    public static double sigmoid(double x) {
        return 1 / (1 + Math.exp(-x));
    }

}
