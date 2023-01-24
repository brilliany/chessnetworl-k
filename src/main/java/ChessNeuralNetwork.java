import java.io.*;
import java.util.*;

public class ChessNeuralNetwork {
    private static final int NUM_GENERATIONS = 1; // or any number you choose
    public final int WHITE = 0;
    public final int BLACK = 1;

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
            population[i] = new NeuralNetwork(64, 32, 64);
        }

        // Train each network in the population for multiple generations
        for (int generation = 0; generation < numGenerations; generation++) {
            System.out.println("Starting generation " + (generation + 1));
            // Play each network against all other networks in the population
            for (int i = 0; i < populationSize; i++) {
                for (int j = i + 1; j < populationSize; j++) {
                    // Play a game of chess between the two networks
                    MoveGenerator moveGenerator = new MoveGenerator();
                    int winner = playGame(moveGenerator, population[i], population[j]);
// Update the fitness of the networks based on the result of the game
                    if (winner == WHITE) {
                        System.out.println("White wins");
                        population[i].incrementFitness();
                        moveGenerator.print();
                    } else if (winner == BLACK) {
                        System.out.println("Black wins");
                        population[j].incrementFitness();
                        moveGenerator.print();
                    } else {
                        moveGenerator.print();
                        System.out.println("The game ended in a draw.");
                    }
                }
            }

            // Select the best networks for breeding
            NeuralNetwork[] newPopulation = new NeuralNetwork[populationSize];
            for (int i = 0; i < populationSize; i++) {

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
            population = newPopulation;

            // Print out the average fitness of the networks in the population
            double totalFitness = 0;
            for (NeuralNetwork network : population) {
                totalFitness += network.getFitness();
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
        System.out.println("Starting a new game of chess.");
        network1.setColor(WHITE);
        network2.setColor(BLACK);
        int movenumber = 0;
        List<int[]> prevMoves = new ArrayList<>();
        int lastPlayer = BLACK;
        System.out.println(moveGenerator.isCheckmate(network1.getColor()));
        System.out.println(moveGenerator.isStalemate(network1.getColor()));

        while (!moveGenerator.isStalemate(network1.getColor()) && !moveGenerator.isCheckmate(network1.getColor()) && movenumber < 175) {
            NeuralNetwork currentNetwork = lastPlayer == BLACK ? network1 : network2;
            lastPlayer = currentNetwork.getColor();
            int[] move = currentNetwork.chooseMove(moveGenerator.getBoardState(), moveGenerator, currentNetwork.getColor());
            System.out.println("Move: " + move[0] + " " + move[1] + " " + move[2] + " " + move[3]);
            if (prevMoves.contains(move)) {
                System.out.println(currentNetwork.getColor() == WHITE ? "White" : "Black" + " player wins due to repeating moves");
                return (currentNetwork.getColor() == WHITE) ? BLACK : WHITE;
            } else {
                prevMoves.add(move);
            }
            boolean successfulmove = moveGenerator.makeMove(move, currentNetwork.getColor());
            if (!successfulmove) {
                System.out.println(currentNetwork.getColor() == WHITE ? "White" : "Black" + " player wins due to illegal move");
                return (currentNetwork.getColor() == WHITE) ? BLACK : WHITE;
            }
            movenumber++;
            if (movenumber % 50 == 0) {
                prevMoves.clear();
            }
        }
        System.out.println("Game over.");
        if (moveGenerator.isCheckmate(network1.getColor())) {
            return (network1.getColor() == WHITE) ? BLACK : WHITE;
        } else {
            return -1;
        }
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
        NeuralNetwork child = new NeuralNetwork(64, 32, 64);
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


    }

    NeuralNetwork loadNetwork(String name) {
        // Loading the object
        try (FileInputStream fis = new FileInputStream(name + ".ser");
             ObjectInputStream ois = new ObjectInputStream(fis)) {
            return (NeuralNetwork) ois.readObject();
        } catch (IOException | ClassNotFoundException e) {
            e.printStackTrace();
        }
        return null;
    }

    private void breedNewPopulation(NeuralNetwork[] population, int populationSize) {
        int numToSelect = (int) (populationSize * 0.2);
        NeuralNetwork[] bestNetworks = selectBestNetworks(population, numToSelect);
        NeuralNetwork[] newPopulation = new NeuralNetwork[populationSize];
        int currentIndex = 0;
        for (int i = 0; i < bestNetworks.length; i++) {
            newPopulation[currentIndex] = bestNetworks[i];
            currentIndex++;
        }
        while (currentIndex < populationSize) {
            NeuralNetwork parent1 = bestNetworks[(int) (Math.random() * numToSelect)];
            NeuralNetwork parent2 = bestNetworks[(int) (Math.random() * numToSelect)];
            NeuralNetwork child = new NeuralNetwork(64, 32, 64);
            child.setInputWeights(breedWeights(parent1.getInputWeights(), parent2.getInputWeights()));
            child.setHiddenWeights(breedWeights(parent1.getHiddenWeights(), parent2.getHiddenWeights()));
            child.setOutputWeights(breedWeights(parent1.getOutputWeights(), parent2.getOutputWeights()));
            child.setInputBiases(breedBiases(parent1.getInputBiases(), parent2.getInputBiases()));
            child.setHiddenBiases(breedBiases(parent1.getHiddenBiases(), parent2.getHiddenBiases()));
            child.setOutputBiases(breedBiases(parent1.getOutputBiases(), parent2.getOutputBiases()));
            child.mutate();
            newPopulation[currentIndex] = child;
            currentIndex++;
        }
        for (int i = 0; i < populationSize; i++) {
            population[i] = newPopulation[i];
        }
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
}