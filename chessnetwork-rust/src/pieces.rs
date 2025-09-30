// convert enum to string
#[macro_export] macro_rules! enum_str {
    (
        #[derive($($derive:ident),*)]   // Capture derive attributes
        $vis:vis enum $name:ident {
            $($variant:ident = $val:expr),*,
        }
    ) => {
        // Apply visibility and derive attributes to the enum
        #[derive($($derive),*)]
        $vis enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };

    // Case for when no derive attributes are provided
    (
        $vis:vis enum $name:ident {
            $($variant:ident = $val:expr),*,
        }
    ) => {
        // Apply visibility to the enum
        $vis enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

enum_str! {
    #[derive(Copy, Clone)]
    pub enum PieceType {
        Pawn = 1,
        Knight = 2,
        Bishop = 3,
        Rook = 4,
        Queen = 5,
        King = 6,
        Empty = 0,
    }
}

enum_str! {
    #[derive(Copy, Clone)]
    pub enum Color {
    White = 1,
    Black = -1,
    }
}


pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }
    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }
    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn to_string(&self) -> String {
        format!("{}_{:?}", self.color.name(), self.piece_type.name())
    }
}
