pub use crate::engine::square::{Position, PositionRow, PositionColumn};
use anyhow::{Result, anyhow};
use crate::error::{ChessError};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
impl PieceType {
    fn from_fen_char(ch: &char) -> anyhow::Result<Self> {
        Ok(match ch {
            'p' | 'P' => Self::Pawn,
            'n' | 'N' => Self::Knight,
            'b' | 'B' => Self::Bishop,
            'r' | 'R' => Self::Rook,
            'q' | 'Q' => Self::Queen,
            'k' | 'K' => Self::King,
            _ => return Err(anyhow!("wrong fen"))
        })
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
   White,
   Black
}

impl Color {
    pub fn next(self) -> Self {
        if self == Self::White {
            Self::Black
        } else {
            Self::White
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: Position
}








// impl Position {
//     pub fn from_coords(roe)
// }


impl Piece {
    pub fn name(&self) -> String {
        let first = if self.color == Color::White {
            "w"
        } else {
            "b"
        };

        let second = match self {
            Self{piece_type: PieceType::Pawn, ..} => "p",
            Self{piece_type: PieceType::Knight, ..} => "n",
            Self{piece_type: PieceType::King, ..} => "k",
            Self{piece_type: PieceType::Bishop, ..} => "b",
            Self{piece_type: PieceType::Rook, ..} => "r",
            _ => "q"
        };
        format!("{}{}", first, second)
    }
    pub fn board_position(&self) -> (f32, f32) {
        let offset_x = self.position.column as i32 as f32 * 12.5f32;
        let offset_y = (7 - self.position.row as i32) as f32 * 12.5f32;
        return (offset_x, offset_y)
    }
    pub fn from_fen_char<Y: Into<PositionRow>, X:Into<PositionColumn>>(ch: &char, row: Y, column: X) -> Result<Self> {
        Ok(Piece{
            position: Position{
                column: column.into(),
                row: row.into(),
            },
            color: if ch.is_lowercase() {Color::Black} else {Color::White},
            piece_type: PieceType::from_fen_char(ch)?,
        })
    }
}
