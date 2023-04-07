package ChessBot;

import static ChessNetwork.BoardUtils.*;
import static ChessNetwork.BoardUtils.KING;

public class BotHelper {
    public static int getRawPieceValue(int piece, int forColor) {
        int value = 0;
        // A heuristic to evaluate the value of a piece for the given color of the player
        // The value is the same for both players, opponent colors are negative scores for the player
        if (piece == PAWN * forColor) {
            value = 10;
        } else if (piece == KNIGHT * forColor) {
            value = 25;
        } else if (piece == BISHOP * forColor) {
            value = 30;
        } else if (piece == ROOK * forColor) {
            value = 50;
        } else if (piece == QUEEN * forColor) {
            value = 90;
        } else if (piece == KING * forColor) {
            value = 2000;
        } else if (piece == PAWN * -forColor) {
            value = -10;
        } else if (piece == KNIGHT * -forColor) {
            value = -25;
        } else if (piece == BISHOP * -forColor) {
            value = -30;
        } else if (piece == ROOK * -forColor) {
            value = -50;
        } else if (piece == QUEEN * -forColor) {
            value = -90;
        } else if (piece == KING * -forColor) {
            value = -2000;
        } else {
            throw new IllegalStateException("Unexpected value: " + piece + " forColor: " + forColor + " piece: " + piece);
        }

        return value;
    }
}
