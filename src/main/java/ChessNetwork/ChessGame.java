package ChessNetwork;

import ChessNetwork.Game.HumanPlayer;
import ChessNetwork.Game.Player;

import java.util.ArrayList;
import java.util.List;

import static ChessNetwork.ChessboardHelper.*;

public class ChessGame {
    private final MoveGenerator moveGenerator;

    Player player1;
    Player player2;
    private final List<ChessGameListener> gameListeners = new ArrayList<>();

    //then add a method to add a listener


    //then call the method on all listeners


    public ChessGame(Player player1, Player player2, MoveGenerator moveGenerator) {
        this.player1 = player1;
        this.player2 = player2;
        this.moveGenerator = moveGenerator;
        startGame();
    }

    private void startGame() {
        System.out.println("Starting game");
    // use player.awaitMove() and then wait for moveGenerator.addMoveListener() to be called
        //there has to always be a human, so the human starts the game
        moveGenerator.addMoveListener(move -> {
            if (gameEnded()) {
                System.out.println("Game ended");
                return;
            }
            if (getColor(move.getPiece()) == player1.getColor()) {
                player2.awaitMove(BLACK, moveGenerator);
            } else {
                player1.awaitMove( WHITE, moveGenerator);
            }
        });
        System.out.println("Player 1: " + player1.getClass().getSimpleName());
        if (player1 instanceof HumanPlayer) {
            player1.init(this, WHITE);
            player2.init(this, BLACK);
            player1.awaitMove(WHITE, moveGenerator);
        } else {
            player1.init(this, WHITE);
            player2.init(this, BLACK);
            player2.awaitMove( BLACK, moveGenerator);
        }
    }



    private boolean gameEnded() {
        if (moveGenerator.isCheckmate(BLACK) || moveGenerator.isCheckmate(WHITE)) {
            callGameEndListeners("checkmate", 1);
            return true;
        }
        if (moveGenerator.isStalemate(BLACK) || moveGenerator.isStalemate(WHITE)) {
            callGameEndListeners("stalemate", 0);
            return true;
        }
        return false;
    }


    public interface ChessGameListener {
        void onGameEnd(String message, int winner);
    }
    public ChessGameListener addMoveListener(ChessGameListener listener) {
        gameListeners.add(listener);
        return listener;
    }

    /**
     * @param message either "checkmate", "stalemate", "draw", "resign"
     * @param winner 1,-1,0
     */
    private void callGameEndListeners(String message, int winner) {
        for (ChessGameListener listener : gameListeners) {
            listener.onGameEnd(message, winner);
        }
    }


}
