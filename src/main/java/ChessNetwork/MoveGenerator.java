package ChessNetwork;


import ChessNetwork.Pieces.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import static ChessNetwork.ChessboardHelper.*;

public class MoveGenerator {
    /*The first two arrays are the y and x coordinates of the piece
    * The third array contains the piece type and color at the index 0, 2 for a white knight, -2 for a black knight etc.
    * Index 1 of the third array contains if the piece has moved or not,
    * this will not be used for most pieces but will be used for pawns, rooks and kings and is crucial for castling and en passant
    * */
    private int[][][] chessboard;

    public MoveGenerator() {
        initChessboard();
    }



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

    private void initChessboard() {
        chessboard = new int[8][8][2];


        /*Like this:
        * -4 -2 -3 -5 -6 -3 -2 -4
        * -1 -1 -1 -1 -1 -1 -1 -1
        *  0  0  0  0  0  0  0  0
        * 0  0  0  0  0  0  0  0
        *  0  0  0  0  0  0  0  0
        * 0  0  0  0  0  0  0  0
        *  1  1  1  1  1  1  1  1
        * 4  2  3  5  6  3  2  4
        * */

        chessboard[0][0] = new int[]{BLACK*ROOK, HAS_NOT_MOVED};
        chessboard[0][1] = new int[]{BLACK*KNIGHT, HAS_NOT_MOVED};
        chessboard[0][2] = new int[]{BLACK*BISHOP, HAS_NOT_MOVED};
        chessboard[0][3] = new int[]{BLACK*QUEEN, HAS_NOT_MOVED};
        chessboard[0][4] = new int[]{BLACK*KING, HAS_NOT_MOVED};
        chessboard[0][5] = new int[]{BLACK*BISHOP, HAS_NOT_MOVED};
        chessboard[0][6] = new int[]{BLACK*KNIGHT, HAS_NOT_MOVED};
        chessboard[0][7] = new int[]{BLACK*ROOK, HAS_NOT_MOVED};
        for (int i = 0; i < 8; i++) {
            chessboard[1][i] = new int[]{BLACK*PAWN, HAS_NOT_MOVED};
        }
        chessboard[7][0] = new int[]{WHITE*ROOK, HAS_NOT_MOVED};
        chessboard[7][1] = new int[]{WHITE*KNIGHT, HAS_NOT_MOVED};
        chessboard[7][2] = new int[]{WHITE*BISHOP, HAS_NOT_MOVED};
        chessboard[7][3] = new int[]{WHITE*QUEEN, HAS_NOT_MOVED};
        chessboard[7][4] = new int[]{WHITE*KING, HAS_NOT_MOVED};
        chessboard[7][5] = new int[]{WHITE*BISHOP, HAS_NOT_MOVED};
        chessboard[7][6] = new int[]{WHITE*KNIGHT, HAS_NOT_MOVED};
        chessboard[7][7] = new int[]{WHITE*ROOK, HAS_NOT_MOVED};
        for (int i = 0; i < 8; i++) {
            chessboard[6][i] = new int[]{WHITE*PAWN, HAS_NOT_MOVED};
        }

        //empty squares
        for (int i = 2; i < 6; i++) {
            for (int j = 0; j < 8; j++) {
                chessboard[i][j] = EMPTY_SQUARE;
            }
        }
    }
    // All of the piece classes have a getMoves method that returns a list of moves for a position
    public int[][][] getChessboard() {
        return chessboard;
    }


    public void makeMove(Move move, int[][][] boardState) {
        int color = ChessboardHelper.getColor(boardState[move.getFromY()][move.getFromX()]);
        // The pieces check themselves if the move is valid
        // This is the method that updates the chessboard
        if (move.isCastle()) {
            // coordinates are stored like chessboard[y][x]
            // so the rook is at the same y as the king
            // and the y is either 0 or 7

            int y = move.getToY();

            // short castling
            if (move.getToX() == 6) {
                // the move is valid since the piece checked it
                // so we can just update the chessboard
                setSquare(4, y, EMPTY_SQUARE, HAS_NOT_MOVED, boardState);
                setSquare(6, y, new int[]{KING*color}, HAS_MOVED, boardState);
                setSquare(7, y, EMPTY_SQUARE, HAS_NOT_MOVED, boardState);
                setSquare(5, y, new int[]{ROOK*color}, HAS_MOVED, boardState);
            } else {
                // the move is valid since the piece checked it
                // so we can just update the chessboard
                setSquare(4, y, EMPTY_SQUARE, HAS_NOT_MOVED, boardState);
                setSquare(2, y, new int[]{KING*color}, HAS_MOVED, boardState);
                setSquare(0, y, EMPTY_SQUARE, HAS_NOT_MOVED, boardState);
                setSquare(3, y, new int[]{ROOK*color}, HAS_MOVED, boardState);
            }
            // Trigger event
            callMoveListeners(move);
            return;
        }
        int pawnDestination = color == WHITE ? 7 : 0;
        //promotion
        if (move.getPiece()[0] == PAWN && move.getToY() == pawnDestination) {
            // the move is valid since the piece checked it
            // so we can just update the chessboard
            setSquare(move.getFromX(), move.getFromY(), EMPTY_SQUARE, HAS_NOT_MOVED, boardState);
            setSquare(move.getToX(), move.getToY(), new int[]{QUEEN}, HAS_MOVED, boardState);
            callMoveListeners(move);
        }
        // the move is valid since the piece checked it
        // so we can just update the chessboard
        setSquare(move.getFromX(), move.getFromY(), EMPTY_SQUARE, HAS_NOT_MOVED, boardState);
        setSquare(move.getToX(), move.getToY(), move.getPiece(), HAS_MOVED, boardState);
        callMoveListeners(move);
    }
    public void setSquare(int x, int y, int[] piece, int hasMoved, int[][][] boardState) {
        boardState[y][x] = new int[]{piece[0], hasMoved};
    }

    public void resetChessBoard() {
        initChessboard();
        //remove all listeners
        moveListeners.clear();
    }

    //then create a list of listeners
    private final List<MoveListener> moveListeners = new ArrayList<>();

    //then add a method to add a listener
    public MoveListener addMoveListener(MoveListener listener) {
        moveListeners.add(listener);
        return listener;
    }

    //then call the method on all listeners
    private void callMoveListeners(Move move) {
        for (MoveListener listener : moveListeners) {
            listener.onMove(move);
        }
    }
    private List<MoveListener> getMoveListeners() {
        return moveListeners;
    }

    //remove a listener
    public void removeMoveListener(MoveListener listener) {
        moveListeners.remove(listener);
    }

    public Move[] getAllMoves(int color, int[][][] boardState) {
        Move[] allMoves = new Move[0];
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                if (boardState[i][j][0] != 0 && getColor(boardState[i][j]) == color) {
                    //loop and get the corresponding class for the piece
                    Move[] moves = ChessboardHelper.getPieceMoves(boardState[i][j], j, i, boardState, this);
                    //add the moves to the list
                    for (Move move : moves) {
                        allMoves = Arrays.copyOf(allMoves, allMoves.length + 1);
                        allMoves[allMoves.length - 1] = move;
                    }
                }
            }
        }
        return allMoves;
    }

    public boolean putsKingInCheck(Move move, int[][][] boardState) {
        // Check if the move puts the king in check
        int[] temp = boardState[move.getToY()][move.getToX()];

        //make the move
        setSquare(move.getFromX(), move.getFromY(), EMPTY_SQUARE, HAS_NOT_MOVED, boardState);
        setSquare(move.getToX(), move.getToY(), move.getPiece(), HAS_MOVED, boardState);
        boolean check = isCheck(getColor(move.getPiece()), boardState);
        //undo the move
        setSquare(move.getFromX(), move.getFromY(), move.getPiece(), move.getPiece()[1], boardState);
        setSquare(move.getToX(), move.getToY(), temp, temp[1], boardState);
        //check if the king is in check
        return check;
    }

    public void print() {

        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                if (chessboard[i][j] == EMPTY_SQUARE) {
                    System.out.print("0 ");
                } else {
                    System.out.print(ChessboardHelper.getPieceTypeAsSymbol(chessboard[i][j]));
                }
            }
            System.out.println();
        }
    }

    public boolean isCheckmate(int color, int[][][] boardState) {
        // Check if the given color is in check
        if (isCheck(color, boardState)) {
            Move[] allMoves = getAllMoves(color, boardState);
            for (Move move : allMoves) {
                if (!putsKingInCheck(move, boardState)) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }

    public boolean isStalemate(int color, int[][][] boardState) {
        // Check if the given color is not in check
        if (!isCheck(color, boardState)) {
            Move[] allMoves = getAllMoves(color, boardState);
            for (Move move : allMoves) {
                if (!putsKingInCheck(move, boardState)) {
                    return false;
                }
            }
            return true;
        }
        return false;
    }


    public boolean isCheck(int color, int[][][] boardState) {

        //get the king of the given color
        int king = color == WHITE ? 6 : -6;
        int opponentcolor = color == WHITE ? BLACK : WHITE;
        //find the position of the king
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                if (boardState[i][j][0] == king) {
                    return isAttacked(j, i, opponentcolor, boardState);
                }
            }
        }
        return false;
    }


    public boolean isAttacked(int x, int y, int opponentColor, int[][][] boardState) {
        //get all the pieces, then call their corresponding isValidMove methods and pass their position and the position of the king
        //if any of them return true, then the king is in check

        //for each square with a piece of the opponent color
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                if (boardState[i][j][0] == PAWN * opponentColor) {
                    //check if the pawn can attack the king
                    Move move = new Move(j, i, x, y, boardState[i][j]);
                    if (Pawn.isValidCapture(move, boardState, j, i, opponentColor)) {
                        return true;
                    }
                } else if (boardState[i][j][0] == KNIGHT*opponentColor){
                    Move move = new Move(j, i, x, y, boardState[i][j]);
                    if (Knight.isValidMove(move, boardState, j, i, opponentColor)) {
                        return true;
                    }
                } else if (boardState[i][j][0] == BISHOP*opponentColor){
                    Move move = new Move(j, i, x, y, boardState[i][j]);
                    if (Bishop.isValidMove(move, boardState, j, i, opponentColor)) {
                        return true;
                    }
                } else if (boardState[i][j][0] == ROOK*opponentColor){
                    Move move = new Move(j, i, x, y, boardState[i][j]);
                    if (Rook.isValidMove(move, boardState, j, i, opponentColor)) {
                        return true;
                    }
                } else if (boardState[i][j][0] == QUEEN*opponentColor){
                    Move move = new Move(j, i, x, y, boardState[i][j]);
                    if (Queen.isValidMove(move, boardState, j, i, opponentColor)) {
                        return true;
                    }
                } else if (boardState[i][j][0] == KING*opponentColor){
                    Move move = new Move(j, i, x, y, boardState[i][j]);
                    if (King.isValidMove(move, boardState, j, i, opponentColor, this)) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    public boolean isInsufficientMaterial(int color, int[][][] boardState) {
        //todo
        return false;
    }
}



