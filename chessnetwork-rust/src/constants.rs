use crate::{BISHOP, BLACK, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE};

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

        pub const WHITE: i8 = 1;
        pub const BLACK: i8 = -1;
        pub const NONE: i8 = 0;

        // Ranks
        pub const RANK_0: u64 = 0x00000000000000FFu64;
        pub const RANK_1: u64 = RANK_0 << 8;
        pub const RANK_2: u64 = RANK_0 << 16;
        pub const RANK_3: u64 = RANK_0 << 24;
        pub const RANK_4: u64 = RANK_0 << 32;
        pub const RANK_5: u64 = RANK_0 << 40;
        pub const RANK_6: u64 = RANK_0 << 48;
        pub const RANK_7: u64 = RANK_0 << 56;

        // Files
        pub const FILE_A: u64 = /*0x1010101010101010u64;*/ 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
        pub const FILE_B: u64 = FILE_A << 1;
        pub const FILE_C: u64 = FILE_A << 2;
        pub const FILE_D: u64 = FILE_A << 3;
        pub const FILE_E: u64 = FILE_A << 4;
        pub const FILE_F: u64 = FILE_A << 5;
        pub const FILE_G: u64 = FILE_A << 6;
        pub const FILE_H: u64 = FILE_A << 7;


        // Square colors and a common center mask
        pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AAu64;
        pub const DARK_SQUARES: u64 = !LIGHT_SQUARES;
        pub const CENTER_4: u64 = 0x0000001818000000u64; // d4,e4,d5,e5


        //starting position bitboards for each piece type
        pub const STARTING_POS : &[(u8, i8, u64)] = &[
            (PAWN, WHITE, RANK_1),
            (PAWN, BLACK, RANK_6),
            (KNIGHT, BLACK, (1u64 << 57) | (1u64 << 62)),
            (KNIGHT, WHITE, (1u64 << 1)  | (1u64 << 6)),
            (BISHOP, BLACK, (1u64 << 58) | (1u64 << 61)),
            (BISHOP, WHITE, (1u64 << 2)  | (1u64 << 5)),
            (ROOK, BLACK, (1u64 << 56) | (1u64 << 63)),
            (ROOK, WHITE, (1u64 << 0)  | (1u64 << 7)),
            (QUEEN, BLACK, (1u64 << 59)),
            (QUEEN, WHITE, (1u64 << 3)),
            (KING, BLACK, (1u64 << 60)),
            (KING, WHITE, (1u64 << 4)),
        ];
    };
}

