package ChessBot;

import ChessNetwork.ChessboardHelper;
import ChessNetwork.MoveGenerator;
import ChessNetwork.Pieces.Move;

import java.util.*;

import static ChessNetwork.ChessboardHelper.*;

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
    private final PieceTables pieceTables;
    private final int depth;
    private final int MAX_SCORE = Integer.MAX_VALUE - 10000;
    private final int MIN_SCORE = Integer.MIN_VALUE + 10000;
    private final MoveGenerator moveGenerator;
    private int cutOffBranches;
    private int amountOfBranches;
    private int ASPIRATION_WINDOW = 50;

    private int[][][] currentPosition;
    private final int color;
    private int confidence;
    private Move bestMove;

    private Map<String, TranspositionEntry> transpositionTable;

    public ChessBot(int depth, PieceTables pieceTables, int color, MoveGenerator moveGenerator) {
        this.depth = depth;
        this.pieceTables = pieceTables;
        this.transpositionTable = new HashMap<>();
        this.color = color;
        this.moveGenerator = moveGenerator;
        this.currentPosition = ChessboardHelper.copyChessboard(moveGenerator.getChessboard());

        // add a move listener to move generator
        moveGenerator.addMoveListener(move -> {
            // ignore bot moves
            currentPosition = ChessboardHelper.copyChessboard(ChessboardHelper.makeMoveSilent(move,currentPosition));
            startSearch();
        });

        // start the initial search
        startSearch();
    }

    private void startSearch() {
        //for each of the opponent's moves, start alpha beta search, then when the opponent's move is made, choose the best move from the list of moves that were searched

        int howDeepWeRn = 0;

        // start iterative deepening if depth hasn't been reached
        if (howDeepWeRn < depth) {
            // clear transposition table
            transpositionTable.clear();
            // reset the best move
            bestMove = null;
            // start iterative deepening
            for (int i = 1; i <= depth; i++) {
                Result result = alphaBeta(currentPosition, i, MIN_SCORE, MAX_SCORE, color, true, transpositionTable);
                if (result.move != null) {
                    bestMove = result.move;
                }
                System.out.println("Depth: " + i + " Score: " + result.score + " Best move: " + result.move);
                confidence = /*percentage of depth completed*/ (int) (((double) i / depth) * 100);
                howDeepWeRn++;
            }
        }
    }

    public Move getBestMove() {
        //wait until confidence is high enough
        while (confidence < 80) {
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
        System.out.println("Confidence: " + confidence + "%");
        return bestMove;
    }

    private Result alphaBeta(int[][][] chessboard, int depth, int alpha, int beta, int color, boolean maximizingPlayer, Map<String, TranspositionEntry> transpositionTable) {
        amountOfBranches++;
        String boardString = ChessboardHelper.boardToString(chessboard);
        TranspositionEntry transpositionEntry = transpositionTable.get(boardString);
        int score = evaluate(chessboard, color);

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
            return new Result(score, null);
        }
        int bestScore;
        Move best;
        Move[] moves = moveGenerator.getAllMoves(color, chessboard);
        //order moves based on evaluation
//        Arrays.sort(moves, (o1, o2) -> {
//            int[][][] newBoard = ChessboardHelper.copyChessboard(chessboard);
//            ChessboardHelper.makeMoveSilent(o1, newBoard);
//            int score1 = evaluate(newBoard, color);
//            newBoard = ChessboardHelper.copyChessboard(chessboard);
//            ChessboardHelper.makeMoveSilent(o2, newBoard);
//            int score2 = evaluate(newBoard, color);
//            return score2 - score1;
//        });
        if (maximizingPlayer) {
            bestScore = MIN_SCORE;
            best = new Move(-1, -1, -1, -1, EMPTY_SQUARE);
            for (Move move : moves) {
// make the move
                int[][][] newBoard = ChessboardHelper.copyChessboard(chessboard);
                ChessboardHelper.makeMoveSilent(move, newBoard);

                Result result = alphaBeta(newBoard, depth - 1, alpha, beta, -color, false, transpositionTable);
// check if we found a better move
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
            best = new Move(-1, -1, -1, -1, EMPTY_SQUARE);
            for (Move move : moves) {
// make the move
                int[][][] newBoard = ChessboardHelper.copyChessboard(chessboard);
                ChessboardHelper.makeMoveSilent(move, newBoard);
// call alpha beta recursively
                Result result = alphaBeta(newBoard, depth - 1, alpha, beta, -color , true, transpositionTable);
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




    public int evaluate(int[][][] chessboard, int color) {
        // Evaluate the position of the chessboard
        int score = 0;
        for (int y = 0; y < 8; y++) {
            for (int x = 0; x < 8; x++) {
                int piece = chessboard[y][x][0];
                if (piece != 0) {
                    score += getRawPieceValue(piece, color);
                }
            }
        }
        score += heuristics(chessboard, color);

        return score;
    }

    private int heuristics(int[][][] chessboard, int color) {
        int[][] pawnPositions = PieceTables.pawnTable;
        int[][] knightPositions = PieceTables.knightTable;
        int[][] bishopPositions = PieceTables.bishopTable;
        int[][] rookPositions = PieceTables.rookTable;
        int[][] queenPositions = PieceTables.queenTable;
        int[][] kingPositions = PieceTables.kingTable;
        // the tables must be flipped for black
        if (color == -1) {
            pawnPositions = PieceTables.pawnTableBlack;
            knightPositions = PieceTables.knightTableBlack;
            bishopPositions = PieceTables.bishopTableBlack;
            rookPositions = PieceTables.rookTableBlack;
            queenPositions = PieceTables.queenTableBlack;
            kingPositions = PieceTables.kingTableBlack;
        }
        int score = 0;
        for (int y = 0; y < 8; y++) {
            for (int x = 0; x < 8; x++) {
                int piece = chessboard[y][x][0];
                if (piece == PAWN * color) {
                    score += pawnPositions[y][x];
                } else if (piece == KNIGHT * color) {
                    score += knightPositions[y][x];
                } else if (piece == BISHOP * color) {
                    score += bishopPositions[y][x];
                } else if (piece == ROOK * color) {
                    score += rookPositions[y][x];
                } else if (piece == QUEEN * color) {
                    score += queenPositions[y][x];
                } else if (piece == KING * color) {
                    score += kingPositions[y][x];
                }
            }
        }
        return score/10;
    }

    private int getRawPieceValue(int piece, int forColor) {
        int value = 0;
        // A heuristic to evaluate the value of a piece for the given color of the player
        // The value is the same for both players, opponent colors are negative scores for the player
        if (piece == PAWN * forColor) {
            value = 100;
        } else if (piece == KNIGHT * forColor) {
            value = 320;
        } else if (piece == BISHOP * forColor) {
            value = 330;
        } else if (piece == ROOK * forColor) {
            value = 500;
        } else if (piece == QUEEN * forColor) {
            value = 900;
        } else if (piece == KING * forColor) {
            value = 20000;
        } else if (piece == PAWN * -forColor) {
            value = -100;
        } else if (piece == KNIGHT * -forColor) {
            value = -320;
        } else if (piece == BISHOP * -forColor) {
            value = -330;
        } else if (piece == ROOK * -forColor) {
            value = -500;
        } else if (piece == QUEEN * -forColor) {
            value = -900;
        } else if (piece == KING * -forColor) {
            value = -20000;
        } else {
            throw new IllegalStateException("Unexpected value: " + piece + " forColor: " + forColor + " piece: " + piece);
        }

        return value;
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
}
