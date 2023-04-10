package ChessBot;

import ChessNetwork.Chessboard;

import static ChessNetwork.BoardUtils.BLACK;
import static ChessNetwork.BoardUtils.WHITE;

public class Heuristics {



    final int color;
    final long whitePawns;
    final long whiteKnights;
    final long whiteBishops;
    final long whiteRooks;
    final long whiteQueens;
    final long whiteKing;
    final long blackPawns;
    final long blackKnights;
    final long blackBishops;
    final long blackRooks;
    final long blackQueens;
    final long blackKing;

    private static final int DEVELOPMENT_WEIGHT = 1;
    private static final int CASTLING_WEIGHT = 3;
    private static final int TWO_MIDDLE_PAWNS_WEIGHT = 4;
    private static final int ISOLATED_PAWN_WEIGHT = -1;
    private static final int DOUBLED_PAWN_WEIGHT = -1;
    private final long leftBoardEdge = 1L | 1L << 8 | 1L << 16 | 1L << 24 | 1L << 32 | 1L << 40 | 1L << 48 | 1L << 56;
    private final long rightBoardEdge = 1L << 7 | 1L << 15 | 1L << 23 | 1L << 31 | 1L << 39 | 1L << 47 | 1L << 55 | 1L << 63;
    private final long sixteenCenterSquares = 1L << 27 | 1L << 28 | 1L << 35 | 1L << 36 | 1L << 19 | 1L << 20 | 1L << 27 | 1L << 28 | 1L << 35 | 1L << 36 | 1L << 19 | 1L << 20 | 1L << 27 | 1L << 28 | 1L << 35 | 1L << 36;
    private final long fourCenterSquares = 1L << 27 | 1L << 28 | 1L << 35 | 1L << 36;

    private static final int KNIGHT_SEMI_OUTPOST_WEIGHT = 1;
    private static final int KNIGHT_OUTPOST_WEIGHT = 3;
    private static final int KNIGHT_EDGE_WEIGHT = 2;

    public Heuristics(Chessboard chessboard, int color) {
        this.color = color;

        whitePawns = chessboard.getWhitePawns();
        whiteKnights = chessboard.getWhiteKnights();
        whiteBishops = chessboard.getWhiteBishops();
        whiteRooks = chessboard.getWhiteRooks();
        whiteQueens = chessboard.getWhiteQueens();
        whiteKing = chessboard.getWhiteKings();

        blackPawns = chessboard.getBlackPawns();
        blackKnights = chessboard.getBlackKnights();
        blackBishops = chessboard.getBlackBishops();
        blackRooks = chessboard.getBlackRooks();
        blackQueens = chessboard.getBlackQueens();
        blackKing = chessboard.getBlackKings();

    }

    int twoMiddlePawns() {
        //3 points for having two pawns in the middle of the board
        //1 point for having one pawn in the middle of the board, 0 points for having none obviously
        int score = 0;
        //check for pawns in the four center squares
        if (color == WHITE) {
            score += Long.bitCount(whitePawns & fourCenterSquares) * TWO_MIDDLE_PAWNS_WEIGHT;
        } else {
            score += Long.bitCount(blackPawns & fourCenterSquares) * TWO_MIDDLE_PAWNS_WEIGHT;
        }
        return score;
    }
    int developementScore() {
        //add score for each piece not in starting position, score for own color 1, for opponent color -1,
        int score = 0;
        //white pieces score
        int wKs = Long.bitCount(whiteKnights & ~((1L << 57) | (1L << 62)));
        int wBs = Long.bitCount(whiteBishops & ~((1L << 58) | (1L << 61)));
        int wRs = Long.bitCount(whiteRooks & ~((1L << 56) | (1L << 63)));
        int wQs = Long.bitCount(whiteQueens & ~(1L << 59));
        //black pieces score
        int bKs = Long.bitCount(blackKnights & ~((1L << 1) | (1L << 6)));
        int bBs = Long.bitCount(blackBishops & ~((1L << 2) | (1L << 5)));
        int bRs = Long.bitCount(blackRooks & ~((1L << 0) | (1L << 7)));
        int bQs = Long.bitCount(blackQueens & ~(1L << 3));
        if (color == WHITE) {
            // add score for own pieces
            score += wKs * DEVELOPMENT_WEIGHT;
            score += wBs * DEVELOPMENT_WEIGHT;
            score += wRs * DEVELOPMENT_WEIGHT;
            score += wQs * DEVELOPMENT_WEIGHT;

            // subtract score for opponent pieces
            score -= bKs * DEVELOPMENT_WEIGHT;
            score -= bBs * DEVELOPMENT_WEIGHT;
            score -= bRs * DEVELOPMENT_WEIGHT;
            score -= bQs * DEVELOPMENT_WEIGHT;
        } else {
            // add score for own pieces
            score -= wKs * DEVELOPMENT_WEIGHT;
            score -= wBs * DEVELOPMENT_WEIGHT;
            score -= wRs * DEVELOPMENT_WEIGHT;
            score -= wQs * DEVELOPMENT_WEIGHT;

            // subtract score for opponent pieces
            score += bKs * DEVELOPMENT_WEIGHT;
            score += bBs * DEVELOPMENT_WEIGHT;
            score += bRs * DEVELOPMENT_WEIGHT;
            score += bQs * DEVELOPMENT_WEIGHT;
        }
        return score;
    }

    public int castlingScore() {
        int score = 0;
        if (color == WHITE) {
            long wKposShort = whiteKing & (1L << 62);
            long wRposShort = whiteRooks & (1L << 61);
            long wKposLong = whiteKing & (1L << 58);
            long wRposLong = whiteRooks & (1L << 59);
            if (wKposShort != 0 && wRposShort != 0) {
                score += CASTLING_WEIGHT;
            }
            if (wKposLong != 0 && wRposLong != 0) {
                score += CASTLING_WEIGHT;
            }
        } else {
            long bKposShort = blackKing & (1L << 6);
            long bRposShort = blackRooks & (1L << 5);
            long bKposLong = blackKing & (1L << 2);
            long bRposLong = blackRooks & (1L << 3);
            if (bKposShort != 0 && bRposShort != 0) {
                score += CASTLING_WEIGHT;
            }
            if (bKposLong != 0 && bRposLong != 0) {
                score += CASTLING_WEIGHT;
            }
        }
        return score;
    }

    public int pawnStructureScore() {
        //check any doubled pawns, isolated pawns, passed pawns
        int score = 0;
        //check formations for each square
        if (color == WHITE) {
            score += isolatedPawnCheck(WHITE);
        } else {
            score += isolatedPawnCheck(BLACK);
        }
        return score;
}

    private int isolatedPawnCheck(int color) {
        int score = 0;
        if (color == WHITE) {
            //check if the pawn is isolated using the left and right rows
            //get left of each pawn that is not on the left edge
            long leftRow = whitePawns & ~leftBoardEdge;
            //get right of each pawn that is not on the right edge
            long rightRow = whitePawns & ~rightBoardEdge;
            //shift left and right one square
            leftRow = leftRow << 1;
            //all squares under and above the pawn
            leftRow = leftRow | (leftRow << 8) | (leftRow << 16) | (leftRow << 24) | (leftRow << 32) | (leftRow << 40) | (leftRow << 48) | (leftRow << 56);
            rightRow = rightRow >> 1;
            rightRow = rightRow | (rightRow << 8) | (rightRow << 16) | (rightRow << 24) | (rightRow << 32) | (rightRow << 40) | (rightRow << 48) | (rightRow << 56);

            //check if the pawn is isolated
            if ((whitePawns & leftRow) == 0 && (whitePawns & rightRow) == 0) {
                score += ISOLATED_PAWN_WEIGHT;
            }
        } else {
            //check if the pawn is isolated using the left and right rows
            //get left of each pawn that is not on the left edge
            long leftRow = blackPawns & ~leftBoardEdge;
            //get right of each pawn that is not on the right edge
            long rightRow = blackPawns & ~rightBoardEdge;
            //shift left and right one square
            leftRow = leftRow << 1;
            //all squares under and above the pawn
            leftRow = leftRow | (leftRow << 8) | (leftRow << 16) | (leftRow << 24) | (leftRow << 32) | (leftRow << 40) | (leftRow << 48) | (leftRow << 56);
            rightRow = rightRow >> 1;
            rightRow = rightRow | (rightRow << 8) | (rightRow << 16) | (rightRow << 24) | (rightRow << 32) | (rightRow << 40) | (rightRow << 48) | (rightRow << 56);

            //check if the pawn is isolated
            if ((blackPawns & leftRow) == 0 && (blackPawns & rightRow) == 0) {
                score += ISOLATED_PAWN_WEIGHT;
            }
        }

        return score;
    }

    private int doubledPawnCheck(int score, int squareNumber) {
        // this method checks all the doubled pawns and returns the score for each doubled pawn
        long squareUnder = (1L << (squareNumber - 8));

        if ((whitePawns & squareUnder) != 0) {
            score += DOUBLED_PAWN_WEIGHT;
        }
        return score;
    }

    public int knightPositioning() {
        // knight on the edge of the board is bad, knight in the middle is good, if knight protected by pawn its good
        int score = 0;

        score += knightEdgeCheck();
        score += knightOutpostCheck();


        return score;
    }

    private int knightOutpostCheck() {
        //give score based on given color knights position on the board
        int score = 0;
        long centerKnights = Long.bitCount(sixteenCenterSquares & (color == WHITE ? whiteKnights : blackKnights));
        //check if the knight is protected by a pawn, if not, give semi outpost score, if yes, give full outpost score

        long defendedSquares = getDefendedByPawnSquares();
        //give score per knight in center defended by pawn/not defended by pawn
        score += (centerKnights & defendedSquares) * KNIGHT_OUTPOST_WEIGHT;
        score += (centerKnights & ~defendedSquares) * KNIGHT_SEMI_OUTPOST_WEIGHT;
        return score;
    }

    private long getDefendedByPawnSquares() {
        long defendedSquares = 0L;
        if (color == WHITE) {
            defendedSquares = (whitePawns & ~leftBoardEdge) << 7;
            defendedSquares |= (whitePawns & ~rightBoardEdge) << 9;
        } else {
            defendedSquares = (blackPawns & ~leftBoardEdge) >> 9;
            defendedSquares |= (blackPawns & ~rightBoardEdge) >> 7;
        }
        return defendedSquares;
    }


    private int knightEdgeCheck() {
        return (Long.bitCount(rightBoardEdge & (color == WHITE? whiteKnights : blackKnights)) * -KNIGHT_EDGE_WEIGHT) + (Long.bitCount(leftBoardEdge & (color == WHITE? whiteKnights : blackKnights)) * -KNIGHT_EDGE_WEIGHT);
    }

}
