package ChessNetwork;

import ChessNetwork.Game.HumanPlayer;
import ChessNetwork.Game.Player;
import ChessNetwork.Pieces.Move;
import org.jetbrains.annotations.Nullable;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.atomic.AtomicReference;

import static ChessNetwork.ChessboardHelper.PAWN;
import static ChessNetwork.ChessboardHelper.getColor;

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
        AtomicReference<Move> lastMove = new AtomicReference<>(new Move(-1, -1, -1, -1, new int[]{-1, -1, -1}));
        moveGenerator.addMoveListener(move -> {
            int[] enPassantablePawn = enPassantCheck(lastMove);
            System.out.println("En passantable pawn: " + Arrays.toString(enPassantablePawn));
            if (gameEnded(moveGenerator.getChessboard(), getColor(move.getPiece()),enPassantablePawn)) {
                System.out.println("Game ended");
                return;
            }
            System.out.println("New move: " + move);

            if (getColor(move.getPiece()) == player1.getColor()) {
                player2.awaitMove(moveGenerator.getChessboard(), -1, moveGenerator, enPassantablePawn);
            } else {
                player1.awaitMove(moveGenerator.getChessboard(), 1, moveGenerator, enPassantablePawn);
            }
            lastMove.set(move);
        });
        System.out.println("Player 1: " + player1.getClass().getSimpleName());
        if (player1 instanceof HumanPlayer) {
            player1.init(this, 1);
            player2.init(this, -1);
            player1.awaitMove(moveGenerator.getChessboard(), 1, moveGenerator, null);
        } else {
            player1.init(this, 1);
            player2.init(this, -1);
            player2.awaitMove(moveGenerator.getChessboard(), -1, moveGenerator, null);
        }
    }

    @Nullable
    private static int[] enPassantCheck(AtomicReference<Move> lastMove) {
        int[] enPassantablePawn = null;
        if (lastMove.get().getPiece()[0] == PAWN){
            //if was double push, then create a new array containing the x and y of the pawn which is now en passantable
            enPassantablePawn = new int[]{lastMove.get().getToX(), lastMove.get().getToY()};
        }
        return enPassantablePawn;
    }


    private boolean gameEnded(int[][][] boardState, int color,@Nullable int[] pawnWhichIsEnPassantable) {
        if (moveGenerator.isCheckmate(color, boardState, pawnWhichIsEnPassantable)) {
            if (color == 1) {
                callGameEndListeners("checkmate", 1);
            } else {
                callGameEndListeners("checkmate", -1);
            }
            return true;
        }
        if (moveGenerator.isStalemate(color, boardState)) {
            callGameEndListeners("stalemate", 0);
            return true;
        }
        if (moveGenerator.isInsufficientMaterial(color, boardState)) {
            callGameEndListeners("draw", 0);
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
