package ChessBot;

import ChessNetwork.Chessboard;

import static ChessBot.ChessBot.getPositionFromChessboard;
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
    private static final int TWO_MIDDLE_PAWNS_WEIGHT = 2;
    private static final int ISOLATED_PAWN_WEIGHT = -1;
    private static final int DOUBLED_PAWN_WEIGHT = -1;

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
        long middlePawns = (whitePawns & 0b0000000011100000L) | (blackPawns & 0b0000000011100000L);
        if (Long.bitCount(middlePawns) == 2) {
            score += TWO_MIDDLE_PAWNS_WEIGHT;
        } else if (Long.bitCount(middlePawns) == 1) {
            score += TWO_MIDDLE_PAWNS_WEIGHT / 2;
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
            long wKpos = whiteKing & (1L << 6);
            long wRpos = whiteRooks & (1L << 7);

            if (wKpos != 0 && wRpos != 0) {
                score += CASTLING_WEIGHT;
            }
            if ((whiteKing & (1L << 2)) != 0 && (whiteRooks & (1L << 0)) != 0) {
                score += CASTLING_WEIGHT;
            }
        } else {
            if ((blackKing & (1L << 62)) != 0 && (blackRooks & (1L << 63)) != 0) {
                score -= CASTLING_WEIGHT;
            }
            if ((blackKing & (1L << 58)) != 0 && (blackRooks & (1L << 56)) != 0) {
                score -= CASTLING_WEIGHT;
            }
        }
        return score;
    }

    public int pawnStructureScore() {
        //check any doubled pawns, isolated pawns, passed pawns
        int score = 0;
        //check formations for each square
        if (color == WHITE) {
            for (int i = 0; i < 64; i++) {
                if ((whitePawns & (1L << i)) != 0) {
                    score += doubledPawnCheck(score, i);
                    score += isolatedPawnCheck(score, i);
                }
            }
        }
        return score;
}

    private int isolatedPawnCheck(int score, int i) {
        //this method checks all the isolated pawns and returns the score for each isolated pawn
        //check the squares around the pawn for pawns, since the pawn isnt isolated if there is a pawn on the row beside that can move to defend it
        long leftRowMask = (1L << i-1) | (1L << i-8) | (1L << i-7) | (1L << i-9);
        long rightRowMask = (1L << i+1) | (1L << i-8) | (1L << i-7) | (1L << i-9);
        if ((whitePawns & leftRowMask) == 0 && (whitePawns & rightRowMask) == 0) {
            score += ISOLATED_PAWN_WEIGHT;
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
    public static int tables(Chessboard chessboard, int color) {
        long[] pieceBitboards = getPositionFromChessboard(chessboard);
        // tables are stored as an int[] with 64 entries, one for each square
        int score = 0;
        if (color == WHITE) {
            for (int i = 0; i < 64; i++) {
                //pawn table
                if ((pieceBitboards[0] & (1L << i)) != 0) {
                    score += PieceTables.pawnTable[i];
                }
                //knight table
                if ((pieceBitboards[1] & (1L << i)) != 0) {
                    score += PieceTables.knightTable[i];
                }
                //bishop table
                if ((pieceBitboards[2] & (1L << i)) != 0) {
                    score += PieceTables.bishopTable[i];
                }
                //rook table
                if ((pieceBitboards[3] & (1L << i)) != 0) {
                    score += PieceTables.rookTable[i];
                }
                //queen table
                if ((pieceBitboards[4] & (1L << i)) != 0) {
                    score += PieceTables.queenTable[i];
                }
                //king table
                if ((pieceBitboards[5] & (1L << i)) != 0) {
                    score += PieceTables.kingTable[i];
                }
            }
        } else {
            for (int i = 0; i < 64; i++) {
                //pawn table
                if ((pieceBitboards[6] & (1L << i)) != 0) {
                    score += PieceTables.pawnTableBlack[63-i];
                }
                //knight table
                if ((pieceBitboards[7] & (1L << i)) != 0) {
                    score += PieceTables.knightTableBlack[63-i];
                }
                //bishop table
                if ((pieceBitboards[8] & (1L << i)) != 0) {
                    score += PieceTables.bishopTableBlack[63-i];
                }
                //rook table
                if ((pieceBitboards[9] & (1L << i)) != 0) {
                    score += PieceTables.rookTableBlack[63-i];
                }
                //queen table
                if ((pieceBitboards[10] & (1L << i)) != 0) {
                    score += PieceTables.queenTableBlack[63-i];
                }
                //king table
                if ((pieceBitboards[11] & (1L << i)) != 0) {
                    score += PieceTables.kingTableBlack[63-i];
                }
            }
        }
        return score/10;
    }
}
