package ChessNetwork;

import ChessNetwork.Game.BotPlayer;
import ChessNetwork.Game.HumanPlayer;
import ChessNetwork.Game.NeuralNetworkPlayer;
import ChessNetwork.Game.Player;
import ChessNetwork.Pieces.Move;
import javafx.application.Application;
import javafx.application.Platform;
import javafx.scene.Node;
import javafx.scene.Scene;
import javafx.scene.control.Button;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.scene.layout.GridPane;
import javafx.scene.paint.Color;
import javafx.scene.shape.Rectangle;
import javafx.stage.Stage;
import lombok.Getter;

import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import static ChessNetwork.ChessboardHelper.*;

public class ChessBoard extends Application {
    private final int DEPTH = 4;
    private GridPane rootNode;
    @Getter
    private GridPane board;
    private GridPane menuButtons;

    public static void main(String[] args) {
        launch(args);
    }

    @Override
    public void start(Stage primaryStage) {


        // Create a GridPane for the chess board
        board = new GridPane();

        // Add a menu bar to the right of the board
        menuButtons = new GridPane();

        // Add rectangles to the GridPane to represent the chess board squares
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                Rectangle square = new Rectangle(50, 50);
                if ((i + j) % 2 == 0) {
                    square.setFill(Color.WHITE);
                } else {
                    square.setFill(Color.DARKGOLDENROD);
                }
                square.setOnMouseClicked(event -> {
                    int row = GridPane.getRowIndex(square);
                    int col = GridPane.getColumnIndex(square);
                    // add your logic here
                });
                board.add(square, i, j);
            }
        }


        MoveGenerator moveGenerator = new MoveGenerator();
        addPiecesToBoard(moveGenerator);

        // Add a button to either play against the computer or against another player
        addOptions(moveGenerator);

        // Create a Scene with the board and the menu bar on the right of the board

        rootNode = new GridPane();

        Scene scene = new Scene(rootNode, 600, 400);
        rootNode.add(board, 0, 0);
        rootNode.add(menuButtons, 1, 0);


        // Set the stage's title and scene
        primaryStage.setTitle("Chess");
        primaryStage.setScene(scene);


        // Show the stage
        primaryStage.show();
    }

    private void addOptions(MoveGenerator moveGenerator) {
        // Add a button to either play against the computer or against another player
        Button playAgainstComputerButton = new Button("Play against computer");
        Button playAgainstPlayerButton = new Button("Play against player");
        Button playAgainstEngineButton = new Button("Play against engine");
        Button viewGameButton = new Button("View game");
        // Add the buttons to the board so that they are on the right of the chess board
        menuButtons.add(playAgainstComputerButton, 8, 0);
        menuButtons.add(playAgainstEngineButton, 8, 1);
        menuButtons.add(playAgainstPlayerButton, 8, 2);
        menuButtons.add(viewGameButton, 8, 3);

        playAgainstComputer(playAgainstComputerButton, moveGenerator);
        playAgainstEngine(playAgainstEngineButton,moveGenerator);
        playAgainstPlayer(playAgainstPlayerButton);
        viewGame(viewGameButton, moveGenerator);

    }

    private void playAgainstEngine(Button playAgainstEngineButton, MoveGenerator moveGenerator) {
        // engine is ChessBot.class
        playAgainstEngineButton.setOnAction(event -> {
            Player engine = new BotPlayer(DEPTH);
            Player humanPlayer = new HumanPlayer();
            moveGenerator.resetChessBoard();
            updateChessBoard(moveGenerator);
                ChessGame game = new ChessGame(humanPlayer, engine, moveGenerator);
                moveGenerator.addMoveListener(move -> updateChessBoard(moveGenerator));
                game.addMoveListener((message, winner) -> handleGameEnd(message, winner, moveGenerator));
        });
    }

    private void handleGameEnd(String message, int winner, MoveGenerator moveGenerator) {
        //if not a draw
        if (winner != 0) {
            //find the winner king and color it green
            int kingColor = winner == WHITE ? WHITE : BLACK;
            for (int i = 0; i < 8; i++) {
                for (int j = 0; j < 8; j++) {
                    if (moveGenerator.getChessboard()[i][j] != null &&
                        getPieceType(moveGenerator.getChessboard()[i][j]) == KING &&
                        getColor(moveGenerator.getChessboard()[i][j]) == kingColor) {
                        Rectangle square = (Rectangle) getNodeByRowColumnIndex(i, j, board);
                        if (square != null) {
                            square.setFill(Color.GREEN);
                        }
                    }
                }
            }
        }
        //find the losers king and color it red
        int kingColor = winner == WHITE ? BLACK : WHITE;
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                if (moveGenerator.getChessboard()[i][j] != null &&
                    getPieceType(moveGenerator.getChessboard()[i][j]) == KING &&
                    getColor(moveGenerator.getChessboard()[i][j]) == kingColor) {
                    Rectangle square = (Rectangle) getNodeByRowColumnIndex(i, j, board);
                    if (square != null) {
                        square.setFill(Color.RED);
                    }
                }
            }
        }
    }

    private Object getNodeByRowColumnIndex(int i, int j, GridPane board) {
        for (Node node : board.getChildren()) {
            if (GridPane.getRowIndex(node) == i && GridPane.getColumnIndex(node) == j) {
                return node;
            }
        }
        return null;
    }


    private void viewGame(Button viewGameButton, MoveGenerator moveGenerator) {
        viewGameButton.setOnAction(event -> {
            //get list of games in the games folder
            moveGenerator.resetChessBoard();
        });
    }

    private void playAgainstPlayer(Button playAgainstPlayerButton) {
        playAgainstPlayerButton.setOnAction(event -> {
            // add your logic here
        });
    }

    private void playAgainstComputer(Button playAgainstComputerButton, MoveGenerator moveGenerator) {
        playAgainstComputerButton.setOnAction(event -> {
            NeuralNetworkPlayer networkPlayer = new NeuralNetworkPlayer(DEPTH);
            Player humanPlayer = new HumanPlayer();
            moveGenerator.resetChessBoard();
            updateChessBoard(moveGenerator);
                ChessGame game = new ChessGame(humanPlayer, networkPlayer, moveGenerator);
                moveGenerator.addMoveListener(move -> {
                        updateChessBoard(moveGenerator);
                });
                game.addMoveListener((message, winner) -> handleGameEnd(message, winner, moveGenerator));
        });
    }


    private void addPiecesToBoard(MoveGenerator moveGenerator) {
        int[][][] boardState = moveGenerator.getChessboard();
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                int[] piece = boardState[i][j];
                String pieceName = getPieceTypeAsStr(piece).toLowerCase();
                if (piece== EMPTY_SQUARE) {
                    continue;
                }
                String color = getColor(piece) == WHITE ? "white" : "black";
                String fileName = "src/main/resources/Pieces/" + color + "_" + pieceName + ".png";
                File file = new File(fileName);
                Image image = new Image(file.toURI().toString());
                ImageView imageView = new ImageView(image);
                imageView.setFitHeight(50);
                imageView.setFitWidth(50);
                board.add(imageView, j, i);
                handlePieceClick(imageView, j, i, moveGenerator);
            }
        }
    }

    void handlePieceClick(ImageView imageView, int x, int y, MoveGenerator moveGenerator) {
        imageView.setOnMouseClicked(event -> {
            final int[] piece = moveGenerator.getChessboard()[y][x];
            System.out.println("Clicked on " + getPieceTypeAsStr(piece) + " at " + x + ", " + y);
            Move[] moves = getPieceMoves(piece, x, y, moveGenerator.getChessboard(), moveGenerator, null);
            System.out.println("Moves: " + Arrays.toString(moves));
            for (Move move : moves) {
                int row = move.getToY();
                int col = move.getToX();
                //highlight the squares
                Rectangle square = new Rectangle(50, 50);
                square.setFill(Color.GREEN);
                board.add(square, col, row);
                square.setOnMouseClicked(event1 -> {
                    // try to move the piece
                    moveGenerator.makeMove(move, moveGenerator.getChessboard());
                    updateChessBoard(moveGenerator);
                });
            }
        });
    }

    private void unhighlight() {
        // unhighlight all the squares
        List<Node> nodes = new ArrayList<>(board.getChildren());
        for (Node node : nodes) {
            if (node instanceof Rectangle) {
                if (((Rectangle) node).getFill() == Color.GREEN) {
                    board.getChildren().remove(node);
                }
            }
        }
    }

    private void updateChessBoard(MoveGenerator moveGenerator) {
        //use the queue to update the board without producing concurrent modification exception
          Platform.runLater(() -> {
                unhighlight();
                board.getChildren().removeIf(node -> node instanceof ImageView);
                addPiecesToBoard(moveGenerator);
            });
    }
}