



import Pieces.*;
import lombok.Getter;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class MoveGenerator {
    @Getter
    private final int[][] chessboard;


    public final int WHITE = 0;
    public final int BLACK = 1;
    private Pawn[][] pawns;
    private Knight[][] knights;
    private Rook[][] rooks;
    private Bishop[][] bishops;
    private Queen[][] queens;
    private King[][] kings;
    public MoveGenerator() {
        // Initialize the chessboard
        chessboard = new int[8][8];
        pawns = new Pawn[8][8];
        knights = new Knight[8][8];
        rooks = new Rook[8][8];
        bishops = new Bishop[8][8];
        queens = new Queen[8][8];
        kings = new King[8][8];
        initializePieces();
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                int piece = chessboard[i][j];
                switch (Math.abs(piece)) {
                    case 1 -> pawns[i][j] = new Pawn(i, j, piece > 0 ? 0 : 1);
                    case 3 -> bishops[i][j] = new Bishop(i, j, piece > 0 ? 0 : 1);
                    case 4 -> knights[i][j] = new Knight(i, j, piece > 0 ? 0 : 1);
                    case 5 -> rooks[i][j] = new Rook(i, j, piece > 0 ? 0 : 1);
                    case 9 -> queens[i][j] = new Queen(i, j, piece > 0 ? 0 : 1);
                    case 10 -> kings[i][j] = new King(i, j, piece > 0 ? 0 : 1);
                }
            }
        }
    }
    private void initializePieces() {
        // Place white pieces on the board
        chessboard[0][0] = -5; // Black rook
        chessboard[0][1] = -4; // Black knight
        chessboard[0][2] = -3; // Black bishop
        chessboard[0][3] = -9; // Black queen
        chessboard[0][4] = -10; // Black king
        chessboard[0][5] = -3; // Black bishop
        chessboard[0][6] = -4; // Black knight
        chessboard[0][7] = -5; // Black rook
        for (int i = 0; i < 8; i++) {
            chessboard[1][i] = -1; // Black pawn
        }

// Place white pieces on the board
        chessboard[7][0] = 5; // White rook
        chessboard[7][1] = 4; // White knight
        chessboard[7][2] = 3; // White bishop
        chessboard[7][3] = 9; // White queen
        chessboard[7][4] = 10; // White king
        chessboard[7][5] = 3; // White bishop
        chessboard[7][6] = 4; // White knight
        chessboard[7][7] = 5; // White rook
        for (int i = 0; i < 8; i++) {
            chessboard[6][i] = 1; // White pawn
        }
    }
    public boolean makeMove(int[] move, int color) {
        int[][] legalMoves = getAllMoves(color, chessboard);
        boolean validMove = false;
        for(int[] legalMove : legalMoves) {
            if(Arrays.equals(legalMove, move)) {
                validMove = true;
                break;
            }
        }
        //check if the king is in check
        if(validMove) {
            if(isCheck(color, chessboard)) {
                validMove = false;
                //check if the move gets the king out of check with a temporary board
                int[][] tempBoard = new int[8][8];
                for (int i = 0; i < 8; i++) {
                    System.arraycopy(chessboard[i], 0, tempBoard[i], 0, 8);
                }
                tempBoard[move[2]][move[3]] = tempBoard[move[0]][move[1]];
                tempBoard[move[0]][move[1]] = 0;
                if(!isCheck(color, tempBoard)) {
                    validMove = true;
                    System.out.println("King is in check, but the move gets the king out of check");
                }
            }
        }
        //check if the move put the king in check
        if(validMove) {
            int[][] tempBoard = new int[8][8];
            for (int i = 0; i < 8; i++) {
                System.arraycopy(chessboard[i], 0, tempBoard[i], 0, 8);
            }
            tempBoard[move[2]][move[3]] = tempBoard[move[0]][move[1]];
            tempBoard[move[0]][move[1]] = 0;
            if(isCheck(color, tempBoard)) {
                validMove = false;
                System.out.println("The move put the king in check");
            }
        }
        if(validMove) {
            // Update chessboard
            chessboard[move[2]][move[3]] = chessboard[move[0]][move[1]];
            chessboard[move[0]][move[1]] = 0;
            // Update pieces
            switch (Math.abs(chessboard[move[2]][move[3]])) {
                case 1 -> {
                    pawns[move[2]][move[3]] = new Pawn(move[2], move[3], chessboard[move[2]][move[3]] > 0 ? 0 : 1);
                    pawns[move[0]][move[1]] = null;
                }
                case 3 -> {
                    bishops[move[2]][move[3]] = new Bishop(move[2], move[3], chessboard[move[2]][move[3]] > 0 ? 0 : 1);
                    bishops[move[0]][move[1]] = null;
                }
                case 4 -> {
                    knights[move[2]][move[3]] = new Knight(move[2], move[3], chessboard[move[2]][move[3]] > 0 ? 0 : 1);
                    knights[move[0]][move[1]] = null;
                }
                case 5 -> {
                    rooks[move[2]][move[3]] = new Rook(move[2], move[3], chessboard[move[2]][move[3]] > 0 ? 0 : 1);
                    rooks[move[0]][move[1]] = null;
                }
                case 9 -> {
                    queens[move[2]][move[3]] = new Queen(move[2], move[3], chessboard[move[2]][move[3]] > 0 ? 0 : 1);
                    queens[move[0]][move[1]] = null;
                }
                case 10 -> {
                    kings[move[2]][move[3]] = new King(move[2], move[3], chessboard[move[2]][move[3]] > 0 ? 0 : 1);
                    kings[move[0]][move[1]] = null;
                }
            }
            System.out.println("Move successful");
            return true;
        } else {
            System.out.println("Invalid move.");
            return false;
        }
    }
    public int[][] getAllMoves(int color, int[][] board) {
        int[][] moves = new int[0][0];
        // Get all moves for each piece
        for (int i = 0; i < 8; i++) {

            for (int j = 0; j < 8; j++) {
                // Get moves from each piece
                int[][] pawnMoves = getPawnMoves(i, j, color, board);
                int[][] bishopMoves = getBishopMoves(i, j, color, board);
                int[][] knightMoves = getKnightMoves(i, j, color, board);
                int[][] rookMoves = getRookMoves(i, j, color, board);
                int[][] queenMoves = getQueenMoves(i, j, color, board);
                int[][] kingMoves = getKingMoves(i, j, color, board);
                // Combine all moves into one array
                int[][] allMoves = new int[pawnMoves.length + bishopMoves.length + knightMoves.length + rookMoves.length + queenMoves.length + kingMoves.length][4];

                int index = 0;
                for (int[] move : pawnMoves) {
                    allMoves[index] = move;
                    index++;
                }
                for (int[] move : bishopMoves) {
                    allMoves[index] = move;
                    index++;
                }
                for (int[] move : knightMoves) {
                    allMoves[index] = move;
                    index++;
                }
                for (int[] move : rookMoves) {
                    allMoves[index] = move;
                    index++;
                }
                for (int[] move : queenMoves) {
                    allMoves[index] = move;
                    index++;
                }
                for (int[] move : kingMoves) {
                    allMoves[index] = move;
                    index++;
                }
                //add the moves to the array
                moves = addMoves(moves, allMoves);

            }
        }
        return moves;

    }

    private int[][] addMoves(int[][] moves, int[][] allMoves) {
        int[][] newMoves = new int[moves.length + allMoves.length][4];
        int index = 0;
        for (int[] move : moves) {
            newMoves[index] = move;
            index++;
        }
        for (int[] move : allMoves) {
            newMoves[index] = move;
            index++;
        }
        return newMoves;
    }


    public int[][] getPawnMoves(int currentposx, int currentposy, int color, int[][] board) {
        //get the moves using the pawn class
        List<int[]> moves = new ArrayList<>();
        if (pawns[currentposx][currentposy] != null && pawns[currentposx][currentposy].getColor() == color) {
            moves.addAll(pawns[currentposx][currentposy].getMoves(board));
        }
        // Initialize legalMoves array with the correct size
        int[][] legalMoves = new int[moves.size()][4];
        // Copy elements from moves list to legalMoves array
        for (int i = 0; i < moves.size(); i++) {
            legalMoves[i] = moves.get(i);  // <-- change here
        }
        return legalMoves;
    }

    public int[][] getKnightMoves(int currentposy, int currentposx, int color, int[][] board) {
        //get the moves using the pawn class
        List<int[]> moves = new ArrayList<>();
        if (knights[currentposy][currentposx] != null && knights[currentposy][currentposx].getColor() == color) {
            moves.addAll(knights[currentposy][currentposx].getMoves(board));
        }
        // Initialize legalMoves array with the correct size
        int[][] legalMoves = new int[moves.size()][4];
        // Copy elements from moves list to legalMoves array
        for (int i = 0; i < moves.size(); i++) {
            legalMoves[i] = moves.get(i);  // <-- change here
        }
        return legalMoves;
    }

    public int[][] getRookMoves(int currentposy, int currentposx, int color, int[][] board) {
        //get the moves using the pawn class
        List<int[]> moves = new ArrayList<>();
        if (rooks[currentposy][currentposx] != null && rooks[currentposy][currentposx].getColor() == color) {
            moves.addAll(rooks[currentposy][currentposx].getMoves(board));
        }
        // Initialize legalMoves array with the correct size
        int[][] legalMoves = new int[moves.size()][4];
        // Copy elements from moves list to legalMoves array
        for (int i = 0; i < moves.size(); i++) {
            legalMoves[i] = moves.get(i);  // <-- change here
        }
        return legalMoves;
    }

    public int[][] getBishopMoves(int currentposy, int currentposx, int color, int[][] board) {
        //get the moves using the pawn class
        List<int[]> moves = new ArrayList<>();
        if (bishops[currentposy][currentposx] != null && bishops[currentposy][currentposx].getColor() == color) {
            moves.addAll(bishops[currentposy][currentposx].getMoves(board));
        }
        // Initialize legalMoves array with the correct size
        int[][] legalMoves = new int[moves.size()][4];
        // Copy elements from moves list to legalMoves array
        for (int i = 0; i < moves.size(); i++) {
            legalMoves[i] = moves.get(i);  // <-- change here
        }
        return legalMoves;
    }

    public int[][] getQueenMoves(int currentposy, int currentposx, int color, int[][] board) {
        //get the moves using the pawn class
        List<int[]> moves = new ArrayList<>();
        if (queens[currentposy][currentposx] != null && queens[currentposy][currentposx].getColor() == color) {
            moves.addAll(queens[currentposy][currentposx].getMoves(board));
        }
        // Initialize legalMoves array with the correct size
        int[][] legalMoves = new int[moves.size()][4];
        // Copy elements from moves list to legalMoves array
        for (int i = 0; i < moves.size(); i++) {
            legalMoves[i] = moves.get(i);  // <-- change here
        }
        return legalMoves;
    }

    public int[][] getKingMoves(int currentposy, int currentposx, int color, int[][] board) {
        //get the moves using the pawn class
        List<int[]> moves = new ArrayList<>();
        if (kings[currentposy][currentposx] != null && kings[currentposy][currentposx].getColor() == color) {
            moves.addAll(kings[currentposy][currentposx].getMoves(board));
        }
        // Initialize legalMoves array with the correct size
        int[][] legalMoves = new int[moves.size()][4];
        // Copy elements from moves list to legalMoves array
        for (int i = 0; i < moves.size(); i++) {
            legalMoves[i] = moves.get(i);  // <-- change here
        }
        return legalMoves;
    }






    public double[] getBoardState() {
        double[] boardState = new double[64];
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                boardState[i * 8 + j] = chessboard[i][j];
            }
        }
        return boardState;
    }

    public void print() {
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                System.out.print(chessboard[i][j] + " ");
            }
            System.out.println();
        }
    }

    public boolean isCheckmate(int color) {
        // Check if the given color has no moves left
        int[] kingPos = findKing(color);
        if (!isCheck(color, chessboard)) {
            int[][] allMoves = getAllMoves(color, chessboard);
            if (allMoves.length == 0) {
                return true;
            }
        }
        // Check if the king can move out of check
        List<int[]> kingMoves = kings[kingPos[0]][kingPos[1]].getMoves(chessboard);
        for (int[] move : kingMoves) {
            int temp = chessboard[move[0]][move[1]];
            chessboard[move[0]][move[1]] = chessboard[kingPos[0]][kingPos[1]];
            chessboard[kingPos[0]][kingPos[1]] = 0;
            if (!isCheck(color, chessboard)) {
                chessboard[kingPos[0]][kingPos[1]] = chessboard[move[0]][move[1]];
                chessboard[move[0]][move[1]] = temp;
                return true;
            }
            chessboard[kingPos[0]][kingPos[1]] = chessboard[move[0]][move[1]];
            chessboard[move[0]][move[1]] = temp;
        }
        return false;
    }

    public boolean isStalemate(int color) {
        // Check if the given color has no moves left
        int[] kingPos = findKing(color);
        if (!isCheck(color, chessboard)) {
            int[][] allMoves = getAllMoves(color, chessboard);
            if (allMoves.length == 0) {
                return true;
            }
        }
        // Check if the king can move out of check
        List<int[]> kingMoves = kings[kingPos[0]][kingPos[1]].getMoves(chessboard);
        for (int[] move : kingMoves) {
            int temp = chessboard[move[0]][move[1]];
            chessboard[move[0]][move[1]] = chessboard[kingPos[0]][kingPos[1]];
            chessboard[kingPos[0]][kingPos[1]] = 0;
            if (!isCheck(color, chessboard)) {
                chessboard[kingPos[0]][kingPos[1]] = chessboard[move[0]][move[1]];
                chessboard[move[0]][move[1]] = temp;
                return false;
            }
            chessboard[kingPos[0]][kingPos[1]] = chessboard[move[0]][move[1]];
            chessboard[move[0]][move[1]] = temp;
        }
        return true;

    }

    private int[] findKing(int color) {
        // Find the position of the king of the given color
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                if (chessboard[i][j] == color * 6) {
                    return new int[]{i, j};
                }
            }
        }
        return null;
    }
    public double calculateAggressionScore(int startRow, int startCol, int endRow, int endCol, int color) {
        // initialize aggression score to 0
        double aggressionScore = 0;

        // calculate the difference in x and y coordinates
        int xDiff = endRow - startRow;
        int yDiff = endCol - startCol;

        // if the move is towards the opponent's side of the board (for white, towards the bottom; for black, towards the top)
        if (color == WHITE && yDiff > 0 || color == BLACK && yDiff < 0) {
            // increase aggression score
            aggressionScore += 0.5;
        } else {
            // decrease aggression score
            aggressionScore -= 0.5;
        }
        // if the move is capturing an opponent's piece
        if (chessboard[endRow][endCol] != 0) {
            // increase aggression score
            aggressionScore += 1;
        }
        return aggressionScore;
    }
    public boolean isCheck(int color, int[][] board) {
        // Find the position of the king of the given color
        int[] kingPos = findKing(color);
        // Check if the king is attacked by any opponent's piece
        if (kingPos != null) {
            return isAttacked(kingPos[0], kingPos[1], -color, board);
        }
        return false;
    }

    private boolean isAttacked(int kingRow, int kingCol, int opponentColor, int[][] board) {
        if (opponentColor == WHITE) {
            opponentColor = 1;

        } else if (opponentColor == BLACK) {
            opponentColor = -1;
        }
        int[][] allmoves = getAllMoves(opponentColor, board);
        for (int[] move : allmoves) {
            if (move[2] == kingRow && move[3] == kingCol) {
                System.out.println("King is attacked by " + move[0] + " " + move[1] + " " + move[2] + " " + move[3] + " by piece " + board[move[0]][move[1]]);
                return true;
            }
        }
        return false;
    }
}


