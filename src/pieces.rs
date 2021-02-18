use crate::board::{PositionRow, PositionColumn};

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
impl PieceType {
    fn from_fen_char(ch: &char) -> Self {
        match ch {
            'p' | 'P' => Self::Pawn,
            'n' | 'N' => Self::Knight,
            'b' | 'B' => Self::Bishop,
            'r' | 'R' => Self::Rook,
            'q' | 'Q' => Self::Queen,
            'k' | 'K' => Self::King,
            _ => unreachable!()
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
   White,
   Black
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: PiecePosition
}

#[derive(Copy, Clone, PartialEq)]
pub struct PiecePosition {
    pub row: PositionRow,
    pub column: PositionColumn
}



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
            _ => "q"
        };
        format!("{}{}", first, second)
    }
    pub fn board_position(&self) -> (f32, f32) {

        let offset_x = self.position.column as i32 as f32 * 12.5f32;
        let offset_y = (7 - self.position.row as i32) as f32 * 12.5f32;
        return (offset_x, offset_y)
    }
    pub fn from_fen_char(ch: &char, row: i32, column:i32) -> Self {
        Piece{
            position: PiecePosition{
                column: column.into(),
                row: row.into(),
            },
            color: if ch.is_lowercase() {Color::Black} else {Color::White},
            piece_type: PieceType::from_fen_char(ch),
        }
    }
}
