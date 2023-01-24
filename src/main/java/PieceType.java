public enum PieceType {
    PAWN(1, "Pawn"),
    ROOK(5, "Rook"),
    KNIGHT(3, "Knight"),
    BISHOP(3, "Bishop"),
    QUEEN(9, "Queen"),
    KING(100, "King");

    private final int value;
    private final String name;

    PieceType(int value, String name) {
        this.value = value;
        this.name = name;
    }

    public int getValue() {
        return value;
    }

    public String getName() {
        return name;
    }
}
