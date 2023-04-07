package ChessBot;

import ChessNetwork.Chessboard;
import ChessNetwork.Pieces.Move;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static ChessBot.Heuristics.tables;
import static ChessNetwork.BoardUtils.EMPTY;
import static ChessNetwork.BoardUtils.WHITE;
import static ChessNetwork.MoveGenerator.getAllMoves;

public class ChessBot {

    private final int depth;
    private final int MAX_SCORE = Integer.MAX_VALUE - 10000;
    private final int MIN_SCORE = Integer.MIN_VALUE + 10000;
    private int cutOffBranches;
    private int amountOfBranches;
    private final int color;



    public ChessBot(int depth, int color) {
        this.depth = depth;
        this.color = color;
        System.out.println("Bot created with depth " + depth + " and color " + color);
    }


    public Move getBestMove(Chessboard chessboard) {
        // Get the best move
        cutOffBranches = 0;
        amountOfBranches = 0;
        long startTime = System.currentTimeMillis();

        List<Move> moves = getAllMoves(color, chessboard);
        Move bestMove = null;
        int bestScore = MIN_SCORE;
        for (Move move : moves) {
            chessboard.silentMove(move);
            int score = alphaBeta(depth - 1, MIN_SCORE, MAX_SCORE, color, true, new HashMap<>(), chessboard).score;
            if (score > bestScore) {
                bestScore = score;
                bestMove = move;
            }
            chessboard.restorePrevious();
        }
        if (bestMove == null) {
            System.out.println("No moves found");
            return new Move(-1, -1, -1,-1, EMPTY);
        }

        long endTime = System.currentTimeMillis();
        System.out.println("Time taken: " + (endTime - startTime) + "ms");
        System.out.println("Cut off branches: " + cutOffBranches);
        System.out.println("Amount of branches: " + amountOfBranches);
        System.out.println("Percentage of branches cut off: " + Math.round((double) cutOffBranches / amountOfBranches * 100) + "%");
        System.out.println("Best score: " + bestScore);
        return bestMove;
    }
    private Result alphaBeta(int depth, int alpha, int beta, int color, boolean maximizingPlayer, Map<String, TranspositionEntry> transpositionTable, Chessboard currentPosition) {
        amountOfBranches++;
        String boardString = boardToString(currentPosition);
        TranspositionEntry transpositionEntry = transpositionTable.get(boardString);
        int score = evaluate(currentPosition, color);
        List<Move> moves = getAllMoves(color, currentPosition);
        if (moves.size() == 0){
            //checkmate or stalemate, return minimum score
//            System.out.println("No moves found at end of branch");
            return new Result(MIN_SCORE, null);
        }
        //order moves based on evaluation
//        Arrays.sort(new List[]{moves}, (o1, o2) -> {
//            currentPosition.silentMove((Move) o1);
//            int score1 = evaluate(currentPosition, color);
//            currentPosition.restorePrevious();
//            currentPosition.silentMove((Move) o2);
//            int score2 = evaluate(currentPosition, color);
//            currentPosition.restorePrevious();
//            return score2 - score1;
//        });

        // handle transposition table check here
        if (transpositionEntry != null) {
            if (transpositionEntry.depth >= depth) {
                if (transpositionEntry.type == TranspositionEntry.Type.EXACT) {
                    return new Result(transpositionEntry.score, transpositionEntry.bestMove);
                } else if (transpositionEntry.type == TranspositionEntry.Type.LOWER_BOUND && transpositionEntry.score >= beta) {
                    return new Result(transpositionEntry.score, transpositionEntry.bestMove);
                } else if (transpositionEntry.type == TranspositionEntry.Type.UPPER_BOUND && transpositionEntry.score <= alpha) {
                    return new Result(transpositionEntry.score, transpositionEntry.bestMove);
                }
            }
        }

        if (score >= MAX_SCORE) {
            transpositionTable.put(boardString, new TranspositionEntry(score, depth, TranspositionEntry.Type.EXACT, null));
            return new Result(score, null);
        } else if (score <= MIN_SCORE) {
            transpositionTable.put(boardString, new TranspositionEntry(score, depth, TranspositionEntry.Type.EXACT, null));
            return new Result(score, null);
        } else if (depth == 0) {
            transpositionTable.put(boardString, new TranspositionEntry(score, depth, TranspositionEntry.Type.EXACT, null));
//            System.out.println("Final score at end of branch: " + score);
//            printBitboardAsChessboard(currentPosition[0]|currentPosition[1]|currentPosition[2]|currentPosition[3]|currentPosition[4]|currentPosition[5]|currentPosition[6]|currentPosition[7]|currentPosition[8]|currentPosition[9]|currentPosition[10]|currentPosition[11]);
              return new Result(score, moves.get(0));
        }
        int bestScore;
        Move best;

        if (maximizingPlayer) {
            bestScore = MIN_SCORE;
            best = new Move(-1, -1, -1, -1, EMPTY);
            for (Move move : moves) {
                currentPosition.silentMove(move);
                Result result = alphaBeta(depth - 1, alpha, beta, -color, false, transpositionTable, currentPosition);
                currentPosition.restorePrevious();
                if (result.score > bestScore) {
                    best = move;
                    bestScore = result.score;
                    alpha = Math.max(alpha, result.score);
// handle cutoff
                    if (beta <= alpha) {
                        cutOffBranches++;
                        transpositionTable.put(boardString, new TranspositionEntry(bestScore, depth, TranspositionEntry.Type.LOWER_BOUND, best));
                        break;
                    }
                }
            }
// handle transposition table
            if (bestScore <= alpha) {
                transpositionTable.put(boardString, new TranspositionEntry(bestScore, depth, TranspositionEntry.Type.UPPER_BOUND, best));
            } else {
                transpositionTable.put(boardString, new TranspositionEntry(bestScore, depth, TranspositionEntry.Type.EXACT, best));
            }
        } else {
            bestScore = MAX_SCORE;
            best = new Move(-1, -1, -1, -1, EMPTY);
            for (Move move : moves) {
            // make the move
                currentPosition.silentMove(move);
                // call alpha beta recursively
                Result result = alphaBeta(depth - 1, alpha, beta, -color, true, transpositionTable, currentPosition);
                currentPosition.restorePrevious();
                // check if we found a better move
                if (result.score < bestScore) {
                    best = move;
                    bestScore = result.score;
                    beta = Math.min(beta, result.score);
                // handle cutoff
                    if (beta <= alpha) {
                        cutOffBranches++;
                        transpositionTable.put(boardString, new TranspositionEntry(bestScore, depth, TranspositionEntry.Type.UPPER_BOUND, best));
                        break;
                    }
                }
            }
            // handle transposition table
            if (bestScore >= beta) {
                transpositionTable.put(boardString, new TranspositionEntry(bestScore, depth, TranspositionEntry.Type.LOWER_BOUND, best));
            } else {
                transpositionTable.put(boardString, new TranspositionEntry(bestScore, depth, TranspositionEntry.Type.EXACT, best));
            }
        }
        return new Result(bestScore, best);
    }



    public int evaluate(Chessboard chessboard, int color) {
        // Evaluate the position of the chessboard
        int score = 0;
        score += material(chessboard, color);
        score += heuristics(chessboard, color);
        score += tables(chessboard, color);
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
            score -= Long.bitCount(chessboard.getBlackPawns()) * -100;
            score -= Long.bitCount(chessboard.getBlackKnights()) * -29;
            score -= Long.bitCount(chessboard.getBlackBishops()) * -30;
            score -= Long.bitCount(chessboard.getBlackRooks()) * -50;
            score -= Long.bitCount(chessboard.getBlackQueens()) * -90;
            score -= Long.bitCount(chessboard.getBlackKings()) * -2000;
        }
        else {
            score += Long.bitCount(chessboard.getWhitePawns()) * -100;
            score += Long.bitCount(chessboard.getWhiteKnights()) * -29;
            score += Long.bitCount(chessboard.getWhiteBishops()) * -30;
            score += Long.bitCount(chessboard.getWhiteRooks()) * -50;
            score += Long.bitCount(chessboard.getWhiteQueens()) * -90;
            score += Long.bitCount(chessboard.getWhiteKings()) * -2000;
            score -= Long.bitCount(chessboard.getBlackPawns()) * 10;
            score -= Long.bitCount(chessboard.getBlackKnights()) * 29;
            score -= Long.bitCount(chessboard.getBlackBishops()) * 30;
            score -= Long.bitCount(chessboard.getBlackRooks()) * 50;
            score -= Long.bitCount(chessboard.getBlackQueens()) * 90;
            score -= Long.bitCount(chessboard.getBlackKings()) * 2000;
        }
        return score * color;
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
    public static class Result {
        public int score;
        public Move move;

        public Result(int score, Move move) {
            this.score = score;
            this.move = move;
        }
    }
    public static String boardToString(Chessboard board) {
        long[] chessboard = getPositionFromChessboard(board);
        // Returns a string representation of the board
        // This is used for saving positions
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 12; i++) {
            long piece = chessboard[i];
            while (piece != 0) {
                int square = Long.numberOfTrailingZeros(piece);
                int x = square % 8;
                int y = square / 8;
                sb.append(x);
                sb.append(y);
                sb.append(i);
                piece &= piece - 1;
            }
        }
        return sb.toString();
    }
    static long[] getPositionFromChessboard(Chessboard board){
        long[] position = new long[12];
        position[0] = board.getWhitePawns();
        position[1] = board.getWhiteKnights();
        position[2] = board.getWhiteBishops();
        position[3] = board.getWhiteRooks();
        position[4] = board.getWhiteQueens();
        position[5] = board.getWhiteKings();
        position[6] = board.getBlackPawns();
        position[7] = board.getBlackKnights();
        position[8] = board.getBlackBishops();
        position[9] = board.getBlackRooks();
        position[10] = board.getBlackQueens();
        position[11] = board.getBlackKings();

        return position;

    }
}
