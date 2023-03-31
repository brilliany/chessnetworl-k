/*
package Neuralnetwork;

import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;

import java.io.*;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import static ChessNetwork.ChessboardHelper.BLACK;
import static ChessNetwork.ChessboardHelper.WHITE;

public class ChessNeuralNetwork {
    private static final int NUM_GENERATIONS = 1;

    private final int DEPTH = 10;

    public static void main(String[] args) {
        ChessNeuralNetwork chessNeuralNetwork = new ChessNeuralNetwork();
        chessNeuralNetwork.trainNetwork(2, NUM_GENERATIONS);
    }

    NeuralNetwork bestNetwork;

    public void trainNetwork(int populationSize, int numGenerations) {
        NeuralNetwork[] bestNetworks = new NeuralNetwork[numGenerations];
        // Create an array of networks
        NeuralNetwork[] population = new NeuralNetwork[populationSize];
        for (int i = 0; i < populationSize; i++) {
            // The input layer, hidden layer, and output layer should have a number of nodes that increases with the depth of the search, and corresponds to the number of squares on the chess board
            population[i] = new NeuralNetwork(64 * DEPTH, 64 * DEPTH);
        }

        // Train each network in the population for multiple generations
        for (int generation = 0; generation < numGenerations; generation++) {
            long generationStartTime = System.currentTimeMillis();
            long generationEndTime = System.currentTimeMillis();
            long startTime = System.currentTimeMillis();
            long endTime = System.currentTimeMillis();

            System.out.println("Starting generation " + (generation + 1));
            // Play each network against all other networks in the population
            for (int i = 0; i < populationSize; i++) {
                for (int j = i + 1; j < populationSize; j++) {
                    // Play a game of chess between the two networks
                    //store the time it takes to play the game
                    startTime = System.currentTimeMillis();
                    MoveGenerator moveGenerator = new MoveGenerator();
                    int winner = playGame(moveGenerator, population[i], population[j]);
// Update the fitness of the networks based on the result of the game
                    if (winner == WHITE) {
                        System.out.println("White wins");
                        population[i].incrementFitness();
                        System.out.println("White fitness: " + population[i].getFitness());
                        moveGenerator.print();
                    } else if (winner == BLACK) {
                        System.out.println("Black wins");
                        population[j].incrementFitness();
                        System.out.println("Black fitness: " + population[j].getFitness());
                        moveGenerator.print();
                    } else {
                        //game ended in a draw
                        moveGenerator.print();
                        System.out.println("The game ended in a draw.");
                    }
                    endTime = System.currentTimeMillis();
                    // Print out the time taken to play the first game
                    if (i == 0 && j == 1) {
                        System.out.println("Time taken to play first game: " + (endTime - startTime) + "ms " + "or " + (endTime - startTime) / 1000 + "s");
                    }

                }
            }
            generationEndTime = System.currentTimeMillis();

            System.out.println("Time taken for generation: " + (generationEndTime - generationStartTime) + "ms " + "or " + (generationEndTime - generationStartTime) / 1000 + "s");
            // Select the best networks for breeding
            //decrease the population size the closer we get to the end of the generations
            int newPopulationSize = populationSize - (int) (populationSize * (generation / (double) numGenerations));
            if (newPopulationSize < 2) {
                newPopulationSize = 2;
            }
            populationSize = newPopulationSize;
            NeuralNetwork[] newPopulation = new NeuralNetwork[populationSize];
            System.out.println("Population size is now: " + newPopulationSize);
            for (int i = 0; i < newPopulationSize; i++) {
                // Select the two best networks for breeding
                NeuralNetwork[] parents = selectBestParents(population);
                NeuralNetwork parent1 = parents[0];
                NeuralNetwork parent2 = parents[1];
                // Breed the two networks to create a new network
                newPopulation[i] = breed(parent1, parent2);
                // Add some mutation to the new network
                newPopulation[i].mutate();
            }

            // Replace the old population with the new population
            //set the population to the new population and adjust for the new population size
            population = newPopulation;


            // Print out the average fitness of the networks in the population
            double totalFitness = 0;
            for (NeuralNetwork network : population) {
                if (network != null) {
                    totalFitness += network.getFitness();
                }
            }
            double averageFitness = totalFitness / populationSize;
            System.out.println("Average fitness of population: " + averageFitness);

            // Find the best network in the population
            bestNetwork = selectBestParents(population)[0];

            // Add the best network to the array of best networks
            bestNetworks[generation] = bestNetwork;


            // Reset the fitness of all networks in the population for the next generation
            for (NeuralNetwork network : population) {
                network.resetFitness();
            }
        }
        // Save the best network from the population
        NeuralNetwork bestnn = selectBestParents(bestNetworks)[0];
        saveNetwork(bestnn, "bestnn" +populationSize);
    }


    private int playGame(MoveGenerator moveGenerator, NeuralNetwork network1, NeuralNetwork network2) {
        int moveNumber = 0;
        List<int[]> prevMoves = new ArrayList<>();
        // Save the game in a file, so that it can be viewed later
        //variable to store the game

        //randomize who gets white and who gets black
        network1.setColor(Math.random() < 0.5 ? WHITE : BLACK);
        network2.setColor(network1.getColor() == WHITE ? BLACK : WHITE);
        Move[] lastTwoMoves = new Move[2];
        Move[] lastTwoMoves1 = new Move[2];
        while (moveNumber < 120) {
            // Alternate between the two networks, check their color to determine which network is playing
            NeuralNetwork currentNetwork;
            if (moveNumber % 2 == 0) {
                currentNetwork = */
/*which network is white*//*
 network1.getColor() == WHITE ? network1 : network2;
            } else {
                currentNetwork = */
/*which network is black*//*
 network1.getColor() == BLACK ? network1 : network2;
            }
            int[] move = getMove(moveGenerator, currentNetwork);

            System.out.println("Move: " + move[0] + " " + move[1] + " " + move[2] + " " + move[3]);

            // End game if getMove returns an illegal move
            if (Arrays.equals(move, new int[]{-1, -1, -1, -1})) {
                System.out.println("Illegal move");
                if (moveGenerator.isCheckmate(currentNetwork.getColor(), moveGenerator.getChessboard())) {
                    System.out.println("Checkmate because of " + currentNetwork.getColor() + " player ");
                    return (currentNetwork.getColor() == WHITE) ? BLACK : WHITE;
                } else if (moveGenerator.isStalemate(currentNetwork.getColor(), moveGenerator.getChessboard())) {
                    return -1;
                } else if (moveGenerator.isInsufficientMaterial(currentNetwork.getColor(), moveGenerator.getChessboard())) {
                    System.out.println("Insufficient material because of " + currentNetwork.getColor() + " player ");
                    return -1;
                    //if the last two moves are the same, for both players, then it is a threefold repetition
                } else if (lastTwoMoves[0] != null && lastTwoMoves[1] != null && lastTwoMoves[0].equals(lastTwoMoves[1]) && lastTwoMoves1[0] != null && lastTwoMoves1[1] != null && lastTwoMoves1[0].equals(lastTwoMoves1[1])) {
                    System.out.println("Threefold repetition because of " + currentNetwork.getColor() + " player ");
                    return -1;
                }
                return (currentNetwork.getColor() == WHITE) ? BLACK : WHITE;
            }


            // Penalize for illegal move
            Move moveAsMove = new Move(move[0], move[1], move[2], move[3], moveGenerator.getChessboard()[move[1]][move[0]]);

            moveGenerator.makeMove(moveAsMove, moveGenerator.getChessboard());
            moveNumber++;

            lastTwoMoves[0] = lastTwoMoves[1];
            lastTwoMoves[1] = moveAsMove;
            lastTwoMoves1[0] = lastTwoMoves1[1];
            lastTwoMoves1[1] = moveAsMove;
        }

        // End game if 120 moves have been made

        return -1;
    }


    private int[] getMove(MoveGenerator moveGenerator, NeuralNetwork currentNetwork) {
        return currentNetwork.chooseMove(moveGenerator.getChessboard(), currentNetwork.getColor(), DEPTH, moveGenerator);
    }


    private NeuralNetwork[] selectBestParents(NeuralNetwork[] population) {
        NeuralNetwork parent1 = null;
        NeuralNetwork parent2 = null;
        for (NeuralNetwork network : population) {
            if (parent1 == null || network.getFitness() > parent1.getFitness()) {
                parent2 = parent1;
                parent1 = network;
            } else if (parent2 == null || network.getFitness() > parent2.getFitness()) {
                parent2 = network;
            }
        }
        return new NeuralNetwork[]{parent1, parent2};
    }

    private NeuralNetwork breed(NeuralNetwork parent1, NeuralNetwork parent2) {
        // Create a new network with the weights and biases of the parents
        NeuralNetwork child = new NeuralNetwork(64 * DEPTH, 64 * DEPTH);
        child.setInputWeights(breedWeights(parent1.getInputWeights(), parent2.getInputWeights()));
        child.setHiddenBiases(breedBiases(parent1.getHiddenBiases(), parent2.getHiddenBiases()));
        child.setHiddenWeights(breedWeights(parent1.getHiddenWeights(), parent2.getHiddenWeights()));
        child.setOutputBiases(breedBiases(parent1.getOutputBiases(), parent2.getOutputBiases()));
        return child;
    }


    private double[][] breedWeights(double[][] parent1Weights, double[][] parent2Weights) {
// Create a new 2D array of weights with the values of the parents
        double[][] childWeights = new double[parent1Weights.length][parent1Weights[0].length];
        for (int i = 0; i < parent1Weights.length; i++) {
            for (int j = 0; j < parent1Weights[0].length; j++) {
// Randomly choose a weight from one of the parents
                if (Math.random() < 0.5) {
                    childWeights[i][j] = parent1Weights[i][j];
                } else {
                    childWeights[i][j] = parent2Weights[i][j];
                }
// Introduce random mutation to the weight
                if (Math.random() < 0.1) {
                    childWeights[i][j] += (Math.random() * 2 - 1) * 0.1;
                }
            }
        }
        return childWeights;
    }

    private double[] breedBiases(double[] parent1Biases, double[] parent2Biases) {
// Create a new array of biases with the values of the parents
        double[] childBiases = new double[parent1Biases.length];
        for (int i = 0; i < parent1Biases.length; i++) {
// Randomly choose a bias from one of the parents
            if (Math.random() < 0.5) {
                childBiases[i] = parent1Biases[i];
            } else {
                childBiases[i] = parent2Biases[i];
            }
// Introduce random mutation to the bias
            if (Math.random() < 0.1) {
                childBiases[i] += (Math.random() * 2 - 1) * 0.1;
            }
        }
        return childBiases;
    }

    void saveNetwork(NeuralNetwork nn, String name) {
// Saving the object
        try (FileOutputStream fos = new FileOutputStream(name + ".ser");
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {
            oos.writeObject(nn);
        } catch (IOException e) {
            e.printStackTrace();
        }
        System.out.println("Network saved as " + name + ".ser");

    }

    public static NeuralNetwork loadNetwork(String name) {
        // Loading the object
        try (FileInputStream fis = new FileInputStream(name + ".ser");
             ObjectInputStream ois = new ObjectInputStream(fis)) {
            return (NeuralNetwork) ois.readObject();
        } catch (IOException | ClassNotFoundException e) {
            e.printStackTrace();
        }
        return null;
    }



    private NeuralNetwork[] selectBestNetworks(NeuralNetwork[] population, int numToSelect) {
        NeuralNetwork[] bestNetworks = new NeuralNetwork[numToSelect];
        int currentIndex = 0;
        for (int i = 0; i < numToSelect; i++) {
            int bestIndex = 0;
            for (int j = 0; j < population.length; j++) {
                if (population[j].getFitness() > population[bestIndex].getFitness()) {
                    bestIndex = j;
                }
            }
            bestNetworks[currentIndex] = population[bestIndex];
            currentIndex++;
            population[bestIndex].setFitness(0);
        }
        return bestNetworks;
    }
}*/
