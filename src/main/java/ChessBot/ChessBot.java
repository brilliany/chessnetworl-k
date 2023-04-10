package ChessBot;

import ChessNetwork.Chessboard;
import ChessNetwork.Pieces.Move;

import java.util.*;

import static ChessNetwork.BoardUtils.*;
import static ChessNetwork.MoveGenerator.getAllMoves;

public class ChessBot {
    private final Chessboard chessboard;
    private Move bestMove;
    private int confidence = 0;
    private final int depth;
    private final int MAX_SCORE = Integer.MAX_VALUE - 30000;
    private final int MIN_SCORE = Integer.MIN_VALUE + 30000;
    private final int color;
    private HashMap<Move, Integer> killerMoves;
    private int cutOffBranches;
    private int amountOfBranches;
    private Search search;
    public ChessBot(Chessboard chessboard, int depth, int color) {
        this.chessboard = chessboard;
        this.depth = depth;
        this.color = color;
        search = new Search();
        search.start();
        System.out.println("Bot created with depth " + depth + " and color " + color);
        startSearch();
    }

    private void startSearch() {
        chessboard.addMoveListener(move -> {
            if (!(move.getPiece() > 0 && color == WHITE || move.getPiece() < 0 && color == BLACK)) {
                search.callStopSearchListeners();
                //clear everything
                bestMove = null;
                killerMoves = new HashMap<>();
                cutOffBranches = 0;
                amountOfBranches = 0;
                confidence = 0;
                search.start();
            }
        });
    }




    public Move getBestMove(Chessboard chessboard) {
        // freeze until confidence is above 90%
        while (confidence < 90) {
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
        return bestMove;
    }
    private Result alphaBeta(int depth, int alpha, int beta, int color, boolean maximizingPlayer, TranspositionTable transpositionTable, Chessboard currentPosition, boolean useMoveOrdering) {
        amountOfBranches++;
        long boardKey = boardToKey(currentPosition);
        TranspositionEntry transpositionEntry = transpositionTable.get(boardKey);
        if (transpositionEntry != null) {
            return new Result(transpositionEntry.score, transpositionEntry.bestMove);
        }

        List<Move> moves = getAllMoves(color, currentPosition);
        if (moves.size() == 0){
            return new Result(MIN_SCORE, null);
        }
        if (useMoveOrdering) {
//            moves.sort(Comparator.comparingInt(move -> {
//                currentPosition.silentMove(move);
//                int score = evaluate(currentPosition, color);
//                currentPosition.restorePrevious();
//                return maximizingPlayer ? -score : score;
//            }));
        }
        //put killer moves at the front of the list
        moves.sort(Comparator.comparingInt(move -> {
            if (killerMoves.containsKey(move)) {
                return -killerMoves.get(move);
            }
            return 0;
        }));

        if (depth == 0) {
            return new Result(evaluate(currentPosition, color), moves.get(0));
        }

        int bestScore = maximizingPlayer ? MIN_SCORE : MAX_SCORE;
        Move best = new Move(-1, -1, -1, -1, EMPTY);

        for (Move move : moves) {
            currentPosition.silentMove(move);
            Result result = alphaBeta(depth - 1, alpha, beta, -color, !maximizingPlayer, transpositionTable, currentPosition, useMoveOrdering);
            currentPosition.restorePrevious();

            if (maximizingPlayer && result.score > bestScore) {
                best = move;
                bestScore = result.score;
                alpha = Math.max(alpha, result.score);
                if (beta <= alpha) {
                    cutOffBranches++;
                    transpositionTable.put(boardKey, new TranspositionEntry(bestScore, depth, TranspositionEntry.Type.LOWER_BOUND, best));
                    killerMoves.put(best, bestScore);
                    break;
                }
            } else if (!maximizingPlayer && result.score < bestScore) {
                best = move;
                bestScore = result.score;
                beta = Math.min(beta, result.score);
                if (beta <= alpha) {
                    cutOffBranches++;
                    transpositionTable.put(boardKey, new TranspositionEntry(bestScore, depth, TranspositionEntry.Type.UPPER_BOUND, best));
                    killerMoves.put(best, bestScore);
                    break;
                }
            }
        }

        TranspositionEntry.Type type;
        if (bestScore <= alpha) {
            type = TranspositionEntry.Type.UPPER_BOUND;
        } else if (bestScore >= beta) {
            type = TranspositionEntry.Type.LOWER_BOUND;
        } else {
            type = TranspositionEntry.Type.EXACT;
        }
        transpositionTable.put(boardKey, new TranspositionEntry(bestScore, depth, type, best));

        return new Result(bestScore, best);
    }


    public int evaluate(Chessboard chessboard, int color) {
        // Evaluate the position of the chessboard
        int score = 0;
        score += material(chessboard, color);
        score += heuristics(chessboard, color);
        return score;
    }

    private int material(Chessboard chessboard, int color) {
        // Loop through bitboards and count the number of pieces
        int score = 0;
        if (color == WHITE) {
            score += Long.bitCount(chessboard.getWhitePawns()) * 10;
            score += Long.bitCount(chessboard.getWhiteKnights()) * 29;
            score += Long.bitCount(chessboard.getWhiteBishops()) * 30;
            score += Long.bitCount(chessboard.getWhiteRooks()) * 50;
            score += Long.bitCount(chessboard.getWhiteQueens()) * 90;
            score += Long.bitCount(chessboard.getWhiteKings()) * 2000;
            score += Long.bitCount(chessboard.getBlackPawns()) * -10;
            score += Long.bitCount(chessboard.getBlackKnights()) * -29;
            score += Long.bitCount(chessboard.getBlackBishops()) * -30;
            score += Long.bitCount(chessboard.getBlackRooks()) * -50;
            score += Long.bitCount(chessboard.getBlackQueens()) * -90;
            score += Long.bitCount(chessboard.getBlackKings()) * -2000;
        }
        else {
            score += Long.bitCount(chessboard.getWhitePawns()) * -10;
            score += Long.bitCount(chessboard.getWhiteKnights()) * -29;
            score += Long.bitCount(chessboard.getWhiteBishops()) * -30;
            score += Long.bitCount(chessboard.getWhiteRooks()) * -50;
            score += Long.bitCount(chessboard.getWhiteQueens()) * -90;
            score += Long.bitCount(chessboard.getWhiteKings()) * -2000;
            score += Long.bitCount(chessboard.getBlackPawns()) * 10;
            score += Long.bitCount(chessboard.getBlackKnights()) * 29;
            score += Long.bitCount(chessboard.getBlackBishops()) * 30;
            score += Long.bitCount(chessboard.getBlackRooks()) * 50;
            score += Long.bitCount(chessboard.getBlackQueens()) * 90;
            score += Long.bitCount(chessboard.getBlackKings()) * 2000;
        }
        return score;
    }


    private int heuristics(Chessboard chessboard, int color) {
        // A heuristic to evaluate the position of the chessboard
        //bitboards (color dependent, -1 for black, 1 for white)
        Heuristics heuristics = new Heuristics(chessboard, color);
        int score = 0;
        //material score is already calculated in the evaluate function

        //development score
        score += heuristics.developementScore();
        //castling score
        score += heuristics.castlingScore();
        //pawn structure score
        score += heuristics.pawnStructureScore();
        // two middle pawn bonus
        score += heuristics.twoMiddlePawns();
//        // knight outposts and positioning away from sides
        score += heuristics.knightPositioning();

        return score;
    }


    private static class TranspositionEntry {

        //type of entry
        enum Type {
            EXACT, LOWER_BOUND, UPPER_BOUND
        }
        Type type;
        int depth;
        int score;

        Move bestMove;

        public TranspositionEntry(int score, int depth, Type type, Move bestMove) {
            this.type = type;
            this.depth = depth;
            this.score = score;
            this.bestMove = bestMove;
        }
    }
    public static class TranspositionTable {
        private final Map<Long, TranspositionEntry> table;

        public TranspositionTable(int size) {
            this.table = new HashMap<>(size);
        }

        public void put(long key, TranspositionEntry entry) {
            table.put(key, entry);
        }

        public TranspositionEntry get(long key) {
            return table.get(key);
        }
    }
    public static class Result {
        public int score;
        public Move move;

        public Result(int score, Move move) {
            this.score = score;
            this.move = move;
        }
    }
    public static long boardToKey(Chessboard board) {
        long key = 0;
        key += board.getWhitePawns();
        key += board.getWhiteKnights();
        key += board.getWhiteBishops();
        key += board.getWhiteRooks();
        key += board.getWhiteQueens();
        key += board.getWhiteKings();
        key += board.getBlackPawns();
        key += board.getBlackKnights();
        key += board.getBlackBishops();
        key += board.getBlackRooks();
        key += board.getBlackQueens();
        key += board.getBlackKings();
        key += board.getWhitePieces();
        key += board.getBlackPieces();
        key += board.getAllCastleRights();
        return key;
    }

    public interface StopSearchListener {
        void onFinish();
    }

    public class Search {
        private boolean isStopped = false;
        StopSearchListener[] stopSearchListeners = new StopSearchListener[0];
        StopSearchListener listener;

        public Search() {
            listener = () -> isStopped = true;
        }

        public void start() {
            System.out.println("Starting search for color " + color + " with depth " + depth);
            // Get the best move
            cutOffBranches = 0;
            amountOfBranches = 0;
            long startTime = System.currentTimeMillis();

            List<Move> moves = getAllMoves(color, chessboard);
            int bestScore = MIN_SCORE;
            //use same transposition table for all moves
            TranspositionTable transpositionTable = new TranspositionTable(1_000_000_0); // maximum number of positions stored in the table
            killerMoves = new HashMap<>();
            boolean useMoveOrdering = evaluate(chessboard, color) != 0;
            System.out.println("Starting evaluation: " + evaluate(chessboard, color));
            for (Move move : moves) {

                chessboard.silentMove(move);
                //use move ordering if the position is not equal, since an equal position will (often) lose performance on move ordering
                int score = alphaBeta(depth - 1, MIN_SCORE, MAX_SCORE, color, true, transpositionTable , chessboard, useMoveOrdering).score;
                if (score > bestScore) {
                    bestScore = score;
                    bestMove = move;
                }
                chessboard.restorePrevious();
                //change confidence percentage based on how far the search has progressed
                confidence = (int) Math.round((double) moves.indexOf(move) / moves.size() * 100);
                //modify line to show progress
                System.out.print("\rCalculating best move... " + (useMoveOrdering ? "(with move ordering) " : "") + Math.round((double) moves.indexOf(move) / moves.size() * 100) + "% complete");
                if (isStopped) {
                    System.out.println("\nSearch stopped forcefully");
                    break;
                }
            }

            long endTime = System.currentTimeMillis();
            System.out.println("\nTime taken: " + (endTime - startTime) + "ms");
            System.out.println("Cut off branches: " + cutOffBranches);
            System.out.println("Amount of branches: " + amountOfBranches);
            System.out.println("Percentage of branches cut off: " + Math.round((double) cutOffBranches / amountOfBranches * 100) + "%");
            System.out.println("Best score: " + bestScore);

        }


        public StopSearchListener addStopSearchListener(StopSearchListener listener) {
            stopSearchListeners = Arrays.copyOf(stopSearchListeners, stopSearchListeners.length + 1);
            stopSearchListeners[stopSearchListeners.length - 1] = listener;
            return listener;
        }
        void callStopSearchListeners() {
            for (StopSearchListener listener : stopSearchListeners) {
                listener.onFinish();
            }
        }
        void resetStopSearchListeners() {
            stopSearchListeners = new StopSearchListener[0];
        }
    }
}
