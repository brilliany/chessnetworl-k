

import Pieces.Knight;
import Pieces.Pawn;
import javafx.application.Application;
import javafx.geometry.HPos;
import javafx.geometry.VPos;
import javafx.scene.Node;
import javafx.scene.Scene;
import javafx.scene.image.Image;
import javafx.scene.image.ImageView;
import javafx.scene.layout.GridPane;
import javafx.scene.paint.Color;
import javafx.scene.shape.Rectangle;
import javafx.scene.shape.Shape;
import javafx.stage.Stage;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class ChessBoard extends Application {

    private final int WHITE = 0;
    private final int BLACK = 1;
    private MoveGenerator moveGenerator;
    private GridPane board;

    public static void main(String[] args) {
        launch(args);
    }

    @Override
    public void start(Stage primaryStage) {
        moveGenerator = new MoveGenerator();
        System.out.println(Arrays.toString(moveGenerator.getBoardState()));

        // Create a GridPane for the chess board
        board = new GridPane();

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
                    System.out.println("Square clicked: " + row + ", " + col);
                    // add your logic here
                });
                board.add(square, i, j);
            }
        }


        // Add chess pieces to the board using the moveGenerator.getBoardState()
        double[] boardState = moveGenerator.getBoardState();
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                int piece = (int) boardState[i * 8 + j];
                if (piece != 0) {
                    // Create a chess piece object based on the value in boardState
                    // and add it to the appropriate square on the board
                    addPiecesToBoard();
                }
            }
        }

        // Create a Scene with the board as its root node
        Scene scene = new Scene(board);

        // Set the stage's title and scene
        primaryStage.setTitle("Chess");
        primaryStage.setScene(scene);

        // Show the stage
        primaryStage.show();
    }
    private void addPiecesToBoard() {
        int[][] chessboard = moveGenerator.getChessboard();
        double[] boardState = moveGenerator.getBoardState();
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                int piece = (int) boardState[i * 8 + j];
                if (piece != 0) {
                    int finalI = i;
                    int finalJ = j;
                    ImageView pieceImage = new ImageView();
                    pieceImage.setFitWidth(50);
                    pieceImage.setFitHeight(50);
                    int color = piece > 0 ? WHITE : BLACK;
                    if (color == WHITE) {
                        pieceImage.setImage(new Image("Pieces/white_" + getPieceName(piece) + ".png"));

                    } else {
                        pieceImage.setImage(new Image("Pieces/black_" + getPieceName(piece) + ".png"));
                    }

                    GridPane.setRowIndex(pieceImage, i);
                    GridPane.setColumnIndex(pieceImage, j);
                    board.getChildren().add(pieceImage);

                    pieceImage.setOnMouseClicked(event -> {
                        System.out.println("Color: " + color + " Piece: " + piece);
                        //highlight all valid moves
                        //get all the valid moves for the piece type and get the moves for the specific piece
                        int[][] validMoves = new int[0][];
                        switch (Math.abs(piece)) {
                            case 1 -> validMoves = moveGenerator.getPawnMoves(finalI, finalJ, color, chessboard);
                            case 3 -> validMoves = moveGenerator.getBishopMoves(finalI, finalJ, color, chessboard);
                            case 4 -> validMoves = moveGenerator.getKnightMoves(finalI, finalJ, color, chessboard);
                            case 5 -> validMoves = moveGenerator.getRookMoves(finalI, finalJ, color, chessboard);
                            case 9 -> validMoves = moveGenerator.getQueenMoves(finalI, finalJ, color, chessboard);
                            case 10 -> validMoves = moveGenerator.getKingMoves( finalI, finalJ, color, chessboard);
                        }
                        System.out.println(Arrays.deepToString(validMoves));
                        //highlight all the valid moves
                        for (int[] validMove : validMoves) {
                            int row = validMove[2];
                            int col = validMove[3];
                            Rectangle square = new Rectangle(50, 50);
                            square.setFill(Color.GREEN);
                            square.setOnMouseClicked(event1 -> {
                                //move the piece to the new square
                                //remove the piece from the old square
                                board.getChildren().remove(pieceImage);
                                //add the piece to the new square
                                GridPane.setRowIndex(pieceImage, row);
                                GridPane.setColumnIndex(pieceImage, col);
                                board.getChildren().add(pieceImage);
                                //update the board state
                                int[] move = {finalI, finalJ, row, col};
                                moveGenerator.makeMove(move,color);
                                //remove all the green squares
                                board.getChildren().removeIf(node -> node instanceof Rectangle && ((Rectangle) node).getFill() == Color.GREEN);
                                //sync the board state with the move generator
                                //remove all the pieces from the board
                                board.getChildren().removeIf(node -> node instanceof ImageView);
                                addPiecesToBoard();
                            });
                            board.add(square, col, row);
                        }

                    });
                }
            }
        }
    }

    private String getPieceName(int piece) {
        return switch (Math.abs(piece)) {
            case 1 -> "pawn";
            case 3 -> "bishop";
            case 4 -> "knight";
            case 5 -> "rook";
            case 9 -> "queen";
            case 10 -> "king";
            default -> "";
        };
    }


}