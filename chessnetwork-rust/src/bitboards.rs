#[macro_export]
macro_rules! define_bitboard_consts {
    () => {
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
        pub const FILE_A: u64 = 0x0101010101010101u64;
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
        
    };
}