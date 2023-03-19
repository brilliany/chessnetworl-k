package ChessBot;

import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;
import org.jetbrains.annotations.Nullable;

import java.util.Arrays;
import java.util.concurrent.atomic.AtomicReference;

import static ChessNetwork.ChessboardHelper.KING;
import static ChessNetwork.ChessboardHelper.PAWN;

public class ChessBot {
    // Bot that plays chess
    // Will use the MoveGenerator, ChessboardHelper and all the piece classes

    /*
    MOVEGENERATOR/CHESSBOARD GUIDE:
    The first two arrays are the y and x coordinates of the piece
     * The third array contains the piece type and color at the index 0, 2 for a white knight, -2 for a black knight etc.
     * Index 1 of the third array contains if the piece has moved or not,
     * this will not be used for most pieces but will be used for pawns, rooks and kings and is crucial for castling and en passant
     * */
    /*Negative values for black pieces, positive for white pieces
    chessboard is y, x
    * 1 = pawn
    * 2 = knight
    * 3 = bishop
    * 4 = rook
    * 5 = queen
    * 6 = king
    * 0 = empty
    * */
    /*The first two arrays are the y and x coordinates of the piece
     * The third array contains the piece type and color at the index 0, 2 for a white knight, -2 for a black knight etc.
     * Index 1 of the third array contains if the piece has moved or not,
     * this will not be used for most pieces but will be used for pawns, rooks and kings and is crucial for castling and en passant
     * */


    //When given a chessboard, it will evaluate the position and return the best move

    private final int depth;
    private final int MAX_SCORE = Integer.MAX_VALUE - 10000;
    private final int MIN_SCORE = Integer.MIN_VALUE + 10000;
    int[] pawnWhichIsEnPassantable = new int[2];

    public ChessBot(int depth) {
        //Constructor
        this.depth = depth;
    }

    public static class Result {
        public int score;
        public Move move;

        public Result(int score, Move move) {
            this.score = score;
            this.move = move;
        }
    }

    public Result minimax(int[][][] chessboard, int depth, int alpha, int beta, int color, boolean maximizingPlayer, MoveGenerator moveGenerator) {
        if (depth == 0) {
            return new Result(evaluate(chessboard), null);
        }

        Move bestMove = null;
        Move[] moves = moveGenerator.getAllMoves(color, chessboard, pawnWhichIsEnPassantable);

        // Sort the moves using a heuristic to prioritize more promising moves
        Arrays.sort(moves, (m1, m2) -> {
            int value1 = getMoveValue(m1, chessboard);
            int value2 = getMoveValue(m2, chessboard);
            return Integer.compare(value2, value1);
        });

        if (maximizingPlayer) {
            int maxEval = MIN_SCORE;
            for (Move move : moves) {
                int[][][] newChessboard = makeMove(move, chessboard);
                //if is checkmate, return the move
                if (moveGenerator.isCheckmate(color,newChessboard,pawnWhichIsEnPassantable)) {
                    return new Result(MAX_SCORE, move);
                }
                //if is stalemate, return the move
                if (moveGenerator.isStalemate(color,newChessboard)) {
                    return new Result(0, move);
                }
                int eval = minimax(newChessboard, depth - 1, alpha, beta, -color, false, moveGenerator).score;

                if (eval > maxEval) {
                    maxEval = eval;
                    bestMove = move;
                }
                alpha = Math.max(alpha, eval);
                if (beta <= alpha) {
                    break;
                }
            }
            return new Result(maxEval, bestMove);
        } else {
            int minEval = MAX_SCORE;
            for (Move move : moves) {
                int[][][] newChessboard = makeMove(move, chessboard);
                int eval = minimax(newChessboard, depth - 1, alpha, beta, -color, true, moveGenerator).score;
                if (eval < minEval) {
                    minEval = eval;
                    bestMove = move;
                }
                beta = Math.min(beta, eval);
                if (beta <= alpha) {
                    break;
                }
            }
            return new Result(minEval, bestMove);
        }
    }

    private int getMoveValue(Move move, int[][][] chessboard) {
        // A heuristic to prioritize more promising moves
        int value = 0;
        int startY = move.getFromY();
        int startX = move.getFromX();
        int endY = move.getToY();
        int endX = move.getToX();
        int piece = chessboard[startY][startX][0];
        int color = chessboard[startY][startX][1];
        int targetPiece = chessboard[endY][endX][0];
        int targetColor = chessboard[endY][endX][1];

        // Prioritize capturing moves
        if (targetPiece != 0) {
            value += 100;
        }

        // Prioritize promoting pawns
        if (piece == PAWN && (endY == 0 || endY == 7)) {
            value += 50;


        }
// Prioritize moving pieces towards the center of the board
        if (piece != PAWN) {
            double distanceFromCenter = Math.abs(endX - 3.5) + Math.abs(endY - 3.5);
            value += (int) (10 / (distanceFromCenter + 1));
        }

// Prioritize castling moves
        if (piece == KING && Math.abs(startX - endX) == 2) {
            value += 30;
        }

// Prioritize protecting the king
        if (targetPiece == KING) {
            value += 20;
        }

// Prioritize attacking the opponent's king
        if (targetPiece == KING && targetColor != color) {
            value += 25;
        }

        return value;
    }

    private int[][][] makeMove(Move move, int[][][] chessboard) {
        //handle castling, promotion
        int[][][] newChessboard = new int[8][8][2];
        for (int y = 0; y < 8; y++) {
            for (int x = 0; x < 8; x++) {
                System.arraycopy(chessboard[y][x], 0, newChessboard[y][x], 0, 2);
            }
        }
        int startY = move.getFromY();
        int startX = move.getFromX();
        int endY = move.getToY();
        int endX = move.getToX();
        int piece = newChessboard[startY][startX][0];
        int color = newChessboard[startY][startX][1];
        newChessboard[endY][endX][0] = piece;
        newChessboard[endY][endX][1] = color;
        newChessboard[startY][startX][0] = 0;
        newChessboard[startY][startX][1] = 0;
        pawnWhichIsEnPassantable = enPassantCheck(new AtomicReference<>(move));
        return newChessboard;
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
    public Move getBestMove(int color, MoveGenerator moveGenerator, @Nullable int[] enPassantablePawn) {
        System.out.println("Getting best move");
        //store time
        long startTime = System.currentTimeMillis();
        int[][][] chessboard = moveGenerator.getChessboard();
        int maxEval = MIN_SCORE;
        Move bestMove = null;
        Move[] allMoves = moveGenerator.getAllMoves(color, chessboard, enPassantablePawn);
        //evaluate all moves and sort them by score
//        Arrays.sort(allMoves, Comparator.comparingInt(move -> -evaluate(makeMove(move, chessboard))));

        for (Move move : allMoves) {
            int[][][] newChessboard = makeMove(move, chessboard);
            Result eval = minimax(newChessboard, depth, MIN_SCORE, MAX_SCORE, -color, false, moveGenerator);
            if (eval.score > maxEval) {
                maxEval = eval.score;
                bestMove = move;
            }
        }
        if (bestMove == null) {
            if (allMoves.length == 0) {
                System.out.println("No moves");
            } else {
                bestMove = allMoves[0];
            }
        }
        long endTime = System.currentTimeMillis();
        System.out.println("Time taken: " + (endTime - startTime) + "ms");
        System.out.println("Best move: " + bestMove + " with score: " + maxEval);
        return bestMove;
    }

    private int evaluate(int[][][] chessboard) {
        //evaluation function to determine score of current position
        int score = 0;
        for (int y = 0; y < 8; y++) {
            for (int x = 0; x < 8; x++) {
                int piece = chessboard[y][x][0];
                int color = chessboard[y][x][1];
                if (piece == 0) {
                    continue;
                }
                int pieceValue = getPieceValue(piece);
                score += pieceValue * color;
            }
        }
        return score;
    }

    private int getPieceValue(int piece) {
        return switch (piece) {
            case 1 -> 1; //pawn
            case 2, -2 -> 3; //knight
            case 3, -3 -> 3; //bishop
            case 4, -4 -> 5; //rook
            case 5, -5 -> 9; //queen
            case 6, -6 -> 100; //king
            default -> 0;
        };
    }
}
