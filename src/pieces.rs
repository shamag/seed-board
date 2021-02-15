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
            _ => "q"
        };
        format!("{}{}", first, second)
    }
    pub fn board_position(&self) -> (f32, f32) {

        let offset_x = self.position.column as i32 as f32 * 12.5f32;
        let offset_y = (7 - self.position.row as i32) as f32 * 12.5f32;
        return (offset_x, offset_y)
    }
}
