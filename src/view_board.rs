use crate::pieces::{Piece, Color, PieceType};
pub use crate::engine::square::{Position, PositionRow, PositionColumn};
use crate::error::{ChessError};
use anyhow::{Context, Result};
use thiserror::Error;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ViewBoard {
    pub pieces: Vec<Piece>,
}


impl ViewBoard {
    fn new() -> Self {
        Self {
            pieces: vec![Piece{
                color: Color::White,
                piece_type: PieceType::Pawn,
                position: Position{row: PositionRow::Seven, column: PositionColumn::C}
            },
            Piece{
                color: Color::White,
                piece_type: PieceType::Rook,
                position: Position{row: PositionRow::One, column: PositionColumn::H}
            }]
        }
    }
}



impl Default for ViewBoard {
    #[inline]
    fn default() -> Self {
        Self::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Valid Position")
    }
}

impl FromStr for ViewBoard {
    type Err = ChessError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut board = Self{
            pieces: vec![]
        };

        let tokens: Vec<&str> = value.split(' ').collect();
        if tokens.len() < 4 {
            return Err(ChessError::WrongFen);
        }

        let pieces = tokens[0];
        let mut row = 7;
        let mut column = 0;

        for x in pieces.chars() {
            match x {
                '/' => {
                    row = row-1;
                    column = -1;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    column = column + (x as i32) - 1;
                }
                _ => {
                    let pieace = match Piece::from_fen_char(&x, row, column) {
                        Ok(p) => p,
                        Err(_)=> return Err(ChessError::WrongFen)
                    };
                    board.pieces.push(pieace)
                }

            }
            column = column + 1;
        }

        Ok(board)
    }
}