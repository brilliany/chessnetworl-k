package ChessNetwork;

import ChessNetwork.Game.BotPlayer;
import ChessNetwork.Game.HumanPlayer;
import ChessNetwork.Game.NeuralNetworkPlayer;
import ChessNetwork.Game.Player;
import ChessNetwork.Pieces.Move;
import javafx.application.Application;
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

import java.util.ArrayList;
import java.util.List;

import static ChessNetwork.BoardUtils.*;

public class ChessUI extends Application {

    private final int DEPTH = 5;
    private GridPane rootNode;
    @Getter
    private GridPane board;
    private GridPane menuButtons;
    private final Color DARK_SQUARE = Color.DARKGOLDENROD;
    private final Color WHITE_SQUARE = Color.WHITE;
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
                    square.setFill(WHITE_SQUARE);
                } else {
                    square.setFill(DARK_SQUARE);
                }
                board.add(square, i, j);
            }
        }


        Chessboard chessboard = new Chessboard();
        addPiecesToBoard(chessboard);

        // Add a button to either play against the computer or against another player
        addOptions(chessboard);

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

    private void addOptions(Chessboard chessboard) {
        initNormalGame(chessboard);
        // Add a button to either play against the computer or against another player
        Button playAgainstComputerButton = new Button("Play against computer");
        Button playAgainstPlayerButton = new Button("Play against player");
        Button playAgainstEngineButton = new Button("Play against engine");
        Button engineVsEngineButton = new Button("Engine vs engine");
        Button viewGameButton = new Button("View game");
        // Add the buttons to the board so that they are on the right of the chess board
        menuButtons.add(playAgainstComputerButton, 8, 0);
        menuButtons.add(playAgainstEngineButton, 8, 1);
        menuButtons.add(playAgainstPlayerButton, 8, 2);
        menuButtons.add(viewGameButton, 8, 3);
        menuButtons.add(engineVsEngineButton, 8, 4);

        playAgainstComputer(playAgainstComputerButton, chessboard);
        playAgainstEngine(playAgainstEngineButton,chessboard);
        playAgainstPlayer(playAgainstPlayerButton);
        viewGame(viewGameButton, chessboard);
        engineVsEngine(engineVsEngineButton, chessboard);
    }

    private void engineVsEngine(Button engineVsEngineButton, Chessboard chessboard) {
        engineVsEngineButton.setOnAction(event -> {
            initEngineGame(chessboard);
        });
    }

    private void initNormalGame(Chessboard chessboard) {
        //human vs human
        Player whitePlayer = new HumanPlayer(WHITE, chessboard);
        Player blackPlayer = new HumanPlayer(BLACK, chessboard);
        chessboard.resetChessBoard();
        updateChessBoard(chessboard);
        chessboard.addMoveListener(move -> updateChessBoard(chessboard));
        ChessGame game = new ChessGame(whitePlayer, blackPlayer, chessboard);
        game.addMoveListener((message, winner) -> handleGameEnd(message, winner, chessboard));
    }
    private void initEngineGame(Chessboard chessboard) {
        //human vs human
        Player whitePlayer = new BotPlayer(DEPTH, WHITE, chessboard);
        Player blackPlayer = new BotPlayer(DEPTH, BLACK, chessboard);
        chessboard.resetChessBoard();
        updateChessBoard(chessboard);
        ChessGame game = new ChessGame(whitePlayer, blackPlayer, chessboard);
        chessboard.addMoveListener(move -> updateChessBoard(chessboard));
        game.addMoveListener((message, winner) -> handleGameEnd(message, winner, chessboard));
    }

    private void playAgainstEngine(Button playAgainstEngineButton, Chessboard chessboard) {
        // engine is ChessBot.class
        playAgainstEngineButton.setOnAction(event -> {
            chessboard.resetChessBoard();
            Player engine = new BotPlayer(DEPTH, BLACK, chessboard);
            Player humanPlayer = new HumanPlayer(WHITE, chessboard);

            updateChessBoard(chessboard);
                ChessGame game = new ChessGame(humanPlayer, engine, chessboard);
                chessboard.addMoveListener(move -> updateChessBoard(chessboard));
                game.addMoveListener((message, winner) -> handleGameEnd(message, winner, chessboard));
        });
    }

    private void handleGameEnd(String message, int winner, Chessboard moveGenerator) {
        //if not a draw
        if (winner != 0) {
            //find the winner king and color it green
            int kingColor = winner == WHITE ? WHITE : BLACK;
            if (kingColor == WHITE) {
                long whiteKings = moveGenerator.getWhiteKings();
                int kingIndex = Long.numberOfTrailingZeros(whiteKings);
                int kingRow = kingIndex / 8;
                int kingCol = kingIndex % 8;
                Rectangle square = (Rectangle) getNodeByRowColumnIndex(kingRow, kingCol, board);
                square.setFill(Color.GREEN);
            } else {
                long blackKings = moveGenerator.getBlackKings();
                int kingIndex = Long.numberOfTrailingZeros(blackKings);
                int kingRow = kingIndex / 8;
                int kingCol = kingIndex % 8;
                Rectangle square = (Rectangle) getNodeByRowColumnIndex(kingRow, kingCol, board);
                square.setFill(Color.GREEN);
            }
            System.out.println(message);
        }

    }

    private Object getNodeByRowColumnIndex(int kingRow, int kingCol, GridPane board) {
        for (Node node : board.getChildren()) {
            if (GridPane.getRowIndex(node) == kingRow && GridPane.getColumnIndex(node) == kingCol) {
                return node;
            }
        }
        return null;
    }

    private void viewGame(Button viewGameButton, Chessboard moveGenerator) {
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

    private void playAgainstComputer(Button playAgainstComputerButton, Chessboard chessboard) {
        playAgainstComputerButton.setOnAction(event -> {
            NeuralNetworkPlayer networkPlayer = new NeuralNetworkPlayer(DEPTH);
            Player humanPlayer = new HumanPlayer(WHITE, chessboard);
            chessboard.resetChessBoard();
            updateChessBoard(chessboard);
            chessboard.addMoveListener(move -> {
                updateChessBoard(chessboard);
            });
            ChessGame game = new ChessGame(humanPlayer, networkPlayer, chessboard);

            game.addMoveListener((message, winner) -> handleGameEnd(message, winner, chessboard));
        });
    }


    private void addPiecesToBoard(Chessboard chessboard) {
        // Add the pieces to the board
        for (int row = 0; row < 8; row++) {
            for (int col = 0; col < 8; col++) {
                // Add the pieces to the board
                int pieceType = chessboard.getPieceType(col, row);
                String piece = getPieceTypeAsStr(pieceType).toLowerCase();
                if (piece.equals("empty")) {
                    continue;
                }
                ImageView imageView = new ImageView();
                imageView.setFitHeight(50);
                imageView.setFitWidth(50);
                imageView.setImage(new Image("pieces/" + piece + ".png"));
                board.add(imageView, col, row);
                int finalCol = col;
                int finalRow = row;
                imageView.setOnMouseClicked(event -> {
                    ArrayList<Move> movesToHighlight = getMovesToHighlight(finalCol, finalRow, pieceType, chessboard);
                    unhighlight();
                    for (Move move : movesToHighlight) {
                        Rectangle square = (Rectangle) getNodeByRowColumnIndex(move.getToY(), move.getToX() , board);
                        if (square != null) {
                            square.setFill(Color.GREEN);
                            //if click on green square
                            square.setOnMouseClicked(event1 -> {
                                //move piece
                                chessboard.makeMove(move);
                                //update board
                                updateChessBoard(chessboard);
                                //unhighlight
                                unhighlight();
                            });
                        }
                    }
                });
            }
        }
    }
    private ArrayList<Move> getMovesToHighlight(int x, int y, int pieceType, Chessboard chessboard) {
        List<Move> allmoves = MoveGenerator.getAllMoves((pieceType > 0 ? WHITE : BLACK),chessboard);
        allmoves.removeIf(move -> move.getFromX() != x || move.getFromY() != y);
        return new ArrayList<>(allmoves);
    }

    private void unhighlight() {
        // unhighlight all the squares
        List<Node> nodes = new ArrayList<>(board.getChildren());
        for (Node node : nodes) {
            if (node instanceof Rectangle) {
                if (((Rectangle) node).getFill() == Color.GREEN) {
                    //make it the original color
                    int row = GridPane.getRowIndex(node);
                    int col = GridPane.getColumnIndex(node);
                    if ((row + col) % 2 == 0) {
                        ((Rectangle) node).setFill(WHITE_SQUARE);
                    } else {
                        ((Rectangle) node).setFill(DARK_SQUARE);
                    }
                }
            }
        }
    }

    private void updateChessBoard(Chessboard moveGenerator) {
        //use the queue to update the board without producing concurrent modification exception
                unhighlight();
                board.getChildren().removeIf(node -> node instanceof ImageView);
                //remove all listeners
                List<Node> nodes = new ArrayList<>(board.getChildren());
                for (Node node : nodes) {
                    if (node instanceof Rectangle) {
                        node.setOnMouseClicked(null);
                    }
                }
                addPiecesToBoard(moveGenerator);
    }
}