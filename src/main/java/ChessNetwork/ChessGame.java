package ChessNetwork;

import ChessNetwork.Game.Player;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

import static ChessNetwork.BoardUtils.*;

public class ChessGame {
    private final Chessboard chessboard;

    Player player1;
    Player player2;
    MoveListener listener;
    private final List<ChessGameListener> gameListeners = new ArrayList<>();

    //then add a method to add a listener


    //then call the method on all listeners


    public ChessGame(Player player1, Player player2, Chessboard chessboard) {
        this.player1 = player1;
        this.player2 = player2;
        this.chessboard = chessboard;
        startGame();
    }

    private void startGame() {
        System.out.println("Starting game");
    // use player.awaitMove() and then wait for moveGenerator.addMoveListener() to be called
        //there has to always be a human, so the human starts the game
        AtomicInteger moveCount = new AtomicInteger();
        listener = chessboard.addMoveListener(move -> {
            if (moveCount.get() >= 175) {
                endGame("draw", 0);
                callGameEndListeners("draw", 0);
                return;
            }
            if (gameEnded()) {
                System.out.println("Game ended");
                return;
            }
            if (getColor(move.getPiece()) == player1.getColor()) {
                System.out.println("Awaiting move from player 2");
                player2.awaitMove(-getColor(move.getPiece()), chessboard);
            } else {
                System.out.println("Awaiting move from player 1");
                player1.awaitMove(-getColor(move.getPiece()), chessboard);
            }
            moveCount.getAndIncrement();
        });
        System.out.println("Player 1: " + player1.getClass().getSimpleName());
        if (player1.getColor() == WHITE) {
            player1.init(this);
            player2.init(this);
            player1.awaitMove(WHITE, chessboard);
        } else {
            player1.init(this);
            player2.init(this);
            player2.awaitMove(WHITE, chessboard);
        }
    }

    private void endGame(String reason, int winner) {
        //forcefully end the game
        callGameEndListeners(reason, winner);
        chessboard.removeMoveListener(listener);
    }


    private boolean gameEnded() {
        if (MoveGenerator.isCheckmate(BLACK,chessboard) || MoveGenerator.isCheckmate(WHITE,chessboard)) {
            callGameEndListeners("checkmate", 1);
            chessboard.removeMoveListener(listener);
            return true;
        }
        if (MoveGenerator.isStalemate(BLACK, chessboard) || MoveGenerator.isStalemate(WHITE, chessboard)) {
            callGameEndListeners("stalemate", 0);
            chessboard.removeMoveListener(listener);
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
