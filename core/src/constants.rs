#[macro_export]
macro_rules! define_consts {
    () => {

        pub const PAWN: u8 = 1;
        pub const KNIGHT: u8 = 2;
        pub const BISHOP: u8 = 3;
        pub const ROOK: u8 = 4;
        pub const QUEEN: u8 = 5;
        pub const KING: u8 = 6;
        pub const EMPTY: u8 = 0;


        pub const WHITE: u8 = 1,
        pub const  BLACK: u8 = 0,
        
        pub const NONE: i8 = 0;

        // Ranks
        const RANK_0: u64 = 0x00000000000000FFu64;

        pub const RANKS: &[u64; 8] = &[
            0x00000000000000FFu64,
            RANK_0 << 8,
            RANK_0 << 16,
            RANK_0 << 24,
            RANK_0 << 32,
            RANK_0 << 40,
            RANK_0 << 48,
            RANK_0 << 56
        ];

        // Files
        const FILE_A: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;

        pub const FILES: &[u64; 8] = &[
            FILE_A,
            FILE_A << 1,
            FILE_A << 2,
            FILE_A << 3,
            FILE_A << 4,
            FILE_A << 5,
            FILE_A << 6,
            FILE_A << 7
        ];


        // Square colors and a common center mask
        pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AAu64;
        pub const DARK_SQUARES: u64 = !LIGHT_SQUARES;
        pub const CENTER_4: u64 = 0x0000001818000000u64; // d4,e4,d5,e5


        //starting position bitboards for each piece type
        pub const STARTING_POS : &[(u8, i8, u64)] = &[
            (PAWN, WHITE, RANKS[1]),
            (PAWN, BLACK, RANKS[6]),
            (KNIGHT, BLACK, (1u64 << 57) | (1u64 << 62)),
            (KNIGHT, WHITE, (1u64 << 1)  | (1u64 << 6)),
            (BISHOP, BLACK, (1u64 << 58) | (1u64 << 61)),
            (BISHOP, WHITE, (1u64 << 2)  | (1u64 << 5)),
            (ROOK, BLACK, (1u64 << 56) | (1u64 << 63)),
            (ROOK, WHITE, (1u64 << 0)  | (1u64 << 7)),
            (QUEEN, BLACK, (1u64 << 60)),
            (QUEEN, WHITE, (1u64 << 4)),
            (KING, BLACK, (1u64 << 59)),
            (KING, WHITE, (1u64 << 3)),
        ];

        //castling
        pub const WHITE_KINGSIDE_CASTLE: u8 = 0b0001;
        pub const WHITE_QUEENSIDE_CASTLE: u8 = 0b0010;
        pub const BLACK_KINGSIDE_CASTLE: u8 = 0b0100;
        pub const BLACK_QUEENSIDE_CASTLE: u8 = 0b1000;
    };
}

