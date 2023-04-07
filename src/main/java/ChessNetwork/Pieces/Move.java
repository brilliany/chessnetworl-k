package ChessNetwork.Pieces;

public class Move {
    private final int x;
    private final int y;
    private final int newX;
    private final int newY;

    private int piece;
    private boolean castle;
    private boolean capture;

    public Move(int x, int y, int newX, int newY, int piece) {
        this.piece = piece;
        this.x = x;
        this.y = y;
        this.newX = newX;
        this.newY = newY;
    }

    public int getFromX() {
        return x;
    }

    public int getFromY() {
        return y;
    }

    public int getToX() {
        return newX;
    }

    public int getToY() {
        return newY;
    }

    public void setCastle(boolean castle) {
        this.castle = castle;
    }

    public boolean isCastle() {
        return castle;
    }
    public int getPiece() {
        return piece;
    }
    public void setPiece(int piece) {
        this.piece = piece;
    }

    public boolean isCapture() {
        return capture;
    }
    public void setCapture(boolean capture) {
        this.capture = capture;
    }
}
