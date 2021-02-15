use crate::pieces::{Piece, Color, PieceType, PiecePosition};
use anyhow::{Context, Result};
use thiserror::Error;


pub struct Board {
    pub pieces: Vec<Piece>,
}
#[derive(Error, Debug)]
pub enum ChessError {


    #[error("Fen contains incorrect data")]
    WrongFen,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl Board {
    /// Construct a new `Board` that is completely empty.
    /// Note: This does NOT give you the initial position.  Just a blank slate.
    fn new() -> Board {
        Board {
            pieces: vec![Piece{
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: PiecePosition{row: PositionRow::Seven, column: PositionColumn::C}
            },
            Piece{
                color: Color::White,
                piece_type: PieceType::Rook,
                position: PiecePosition{row: PositionRow::One, column: PositionColumn::H}
            }]
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PositionColumn{
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

impl From<i32> for PositionColumn {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PositionRow{
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7
}
impl From<i32> for PositionRow {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            4 => Self::Five,
            5 => Self::Six,
            6 => Self::Seven,
            7 => Self::Eight,
            _ => unreachable!(),
        }
    }
}


impl Default for Board {
    #[inline]
    fn default() -> Board {
        Board::new()
    }
}

// impl FromStr for BoardBuilder {
//     type Err = ChessError;

//     fn from_str(value: &str) -> Result<Self, Self::Err> {
//         let mut cur_rank = Rank::Eighth;
//         let mut cur_file = File::A;
//         let mut fen = &mut BoardBuilder::new();

//         let tokens: Vec<&str> = value.split(' ').collect();
//         if tokens.len() < 4 {
//             return Err(Error::InvalidFen {
//                 fen: value.to_string(),
//             });
//         }

//         let pieces = tokens[0];
//         let side = tokens[1];
//         let castles = tokens[2];
//         let ep = tokens[3];

//         for x in pieces.chars() {
//             match x {
//                 '/' => {
//                     cur_rank = cur_rank.down();
//                     cur_file = File::A;
//                 }
//                 '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
//                     cur_file =
//                         File::from_index(cur_file.to_index() + (x as usize) - ('0' as usize));
//                 }
//                 'r' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Rook, Color::Black));
//                     cur_file = cur_file.right();
//                 }
//                 'R' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Rook, Color::White));
//                     cur_file = cur_file.right();
//                 }
//                 'n' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Knight, Color::Black));
//                     cur_file = cur_file.right();
//                 }
//                 'N' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Knight, Color::White));
//                     cur_file = cur_file.right();
//                 }
//                 'b' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Bishop, Color::Black));
//                     cur_file = cur_file.right();
//                 }
//                 'B' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Bishop, Color::White));
//                     cur_file = cur_file.right();
//                 }
//                 'p' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Pawn, Color::Black));
//                     cur_file = cur_file.right();
//                 }
//                 'P' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Pawn, Color::White));
//                     cur_file = cur_file.right();
//                 }
//                 'q' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Queen, Color::Black));
//                     cur_file = cur_file.right();
//                 }
//                 'Q' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::Queen, Color::White));
//                     cur_file = cur_file.right();
//                 }
//                 'k' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::King, Color::Black));
//                     cur_file = cur_file.right();
//                 }
//                 'K' => {
//                     fen[Square::make_square(cur_rank, cur_file)] =
//                         Some((Piece::King, Color::White));
//                     cur_file = cur_file.right();
//                 }
//                 _ => {
//                     return Err(Error::InvalidFen {
//                         fen: value.to_string(),
//                     });
//                 }
//             }
//         }
//         match side {
//             "w" | "W" => fen = fen.side_to_move(Color::White),
//             "b" | "B" => fen = fen.side_to_move(Color::Black),
//             _ => {
//                 return Err(Error::InvalidFen {
//                     fen: value.to_string(),
//                 })
//             }
//         }

//         if castles.contains("K") && castles.contains("Q") {
//             fen.castle_rights[Color::White.to_index()] = CastleRights::Both;
//         } else if castles.contains("K") {
//             fen.castle_rights[Color::White.to_index()] = CastleRights::KingSide;
//         } else if castles.contains("Q") {
//             fen.castle_rights[Color::White.to_index()] = CastleRights::QueenSide;
//         } else {
//             fen.castle_rights[Color::White.to_index()] = CastleRights::NoRights;
//         }

//         if castles.contains("k") && castles.contains("q") {
//             fen.castle_rights[Color::Black.to_index()] = CastleRights::Both;
//         } else if castles.contains("k") {
//             fen.castle_rights[Color::Black.to_index()] = CastleRights::KingSide;
//         } else if castles.contains("q") {
//             fen.castle_rights[Color::Black.to_index()] = CastleRights::QueenSide;
//         } else {
//             fen.castle_rights[Color::Black.to_index()] = CastleRights::NoRights;
//         }

//         if let Ok(sq) = Square::from_str(&ep) {
//             fen = fen.en_passant(Some(sq.get_file()));
//         }

//         Ok(*fen)
//     }
// }