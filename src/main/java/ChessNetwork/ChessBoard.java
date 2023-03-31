package ChessNetwork;

import ChessBot.PieceTables;
import ChessNetwork.Game.BotPlayer;
import ChessNetwork.Game.HumanPlayer;
import ChessNetwork.Game.NeuralNetworkPlayer;
import ChessNetwork.Game.Player;
import ChessNetwork.Pieces.*;
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

import static ChessNetwork.ChessboardHelper.*;
import static java.lang.Math.abs;

public class ChessBoard extends Application {
    private final int DEPTH = 7;
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
        initNormalGame(moveGenerator);
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

    private void initNormalGame(MoveGenerator moveGenerator) {
        //human vs human
        Player whitePlayer = new HumanPlayer();
        Player blackPlayer = new HumanPlayer();
        moveGenerator.resetChessBoard();
        updateChessBoard(moveGenerator);
        ChessGame game = new ChessGame(whitePlayer, blackPlayer, moveGenerator);
        moveGenerator.addMoveListener(move -> updateChessBoard(moveGenerator));
        game.addMoveListener((message, winner) -> handleGameEnd(message, winner, moveGenerator));
    }

    private void playAgainstEngine(Button playAgainstEngineButton, MoveGenerator moveGenerator) {
        // engine is ChessBot.class
        playAgainstEngineButton.setOnAction(event -> {
            moveGenerator.resetChessBoard();
            Player engine = new BotPlayer(DEPTH, new PieceTables(), BLACK, moveGenerator);
            Player humanPlayer = new HumanPlayer();

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
        // Add the pieces to the board
        for (int row = 0; row < 8; row++) {
            for (int col = 0; col < 8; col++) {
                // Add the pieces to the board
                int pieceType = moveGenerator.getPieceType(col, row);
                String piece = getPieceTypeAsStr(pieceType).toLowerCase();
                System.out.println(piece);
                if (piece.equals("empty")) {
                    continue;
                }
                ImageView imageView = new ImageView();
                imageView.setFitHeight(50);
                imageView.setFitWidth(50);
                imageView.setImage(new Image("pieces/" + piece + ".png"));
                board.add(imageView, col, row);
                handlePieceClick(imageView, col, row, moveGenerator, pieceType);
            }
        }
    }

    void handlePieceClick(ImageView imageView, int x, int y, MoveGenerator moveGenerator, int pieceType) {
        imageView.setOnMouseClicked(event -> {
            System.out.println("Clicked on " + x + " " + y);
            Move[] movesToHighlight = getMovesToHighlight(moveGenerator, x, y, pieceType);
            for (Move move : movesToHighlight) {
                int row = move.getToY();
                int col = move.getToX();
                Rectangle square = (Rectangle) getNodeByRowColumnIndex(row, col, board);
                if (square != null) {
                    square.setFill(Color.GREEN);
                }
            }
        });
    }

    private Move[] getMovesToHighlight(MoveGenerator moveGenerator, int x, int y, int pieceType) {
        int color = pieceType > 0 ? WHITE : BLACK;
        int piece = abs(pieceType);
        return switch (piece) {
            case PAWN -> Pawn.getMoves(x, y, color, moveGenerator);
            case KNIGHT -> Knight.getMoves(x, y, color, moveGenerator);
            case BISHOP -> Bishop.getMoves(x, y, color, moveGenerator);
            case ROOK -> Rook.getMoves(x, y, color, moveGenerator);
            case QUEEN -> Queen.getMoves(x, y, color, moveGenerator);
            case KING -> King.getMoves(x, y, color, moveGenerator);
            default -> new Move[0];
        };
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
                unhighlight();
                board.getChildren().removeIf(node -> node instanceof ImageView);
                addPiecesToBoard(moveGenerator);

    }
}