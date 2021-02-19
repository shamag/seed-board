use crate::pieces::{Piece, Color, PieceType, PiecePosition};
use anyhow::{Context, Result};
use thiserror::Error;
use std::str::FromStr;


pub struct Board {
    pub pieces: Vec<Piece>,
}
#[derive(Error, Debug)]
pub enum ChessError {


    #[error("Fen contains incorrect data")]
    WrongFen,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl Board {
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
        Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Valid Position")
    }
}

impl FromStr for Board {
    type Err = ChessError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut cur_row = PositionRow::Eight;
        let mut cur_column = PositionColumn::A;
        let mut board = Board{
            pieces: vec![]
        };

        let tokens: Vec<&str> = value.split(' ').collect();
        if tokens.len() < 4 {
            return Err(ChessError::WrongFen);
        }

        let pieces = tokens[0];
        // let side = tokens[1];
        // let castles = tokens[2];
        // let ep = tokens[3];
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
        // match side {
        //     "w" | "W" => fen = fen.side_to_move(Color::White),
        //     "b" | "B" => fen = fen.side_to_move(Color::Black),
        //     _ => {
        //         return Err(Error::InvalidFen {
        //             fen: value.to_string(),
        //         })
        //     }
        // }

        // if castles.contains("K") && castles.contains("Q") {
        //     fen.castle_rights[Color::White.to_index()] = CastleRights::Both;
        // } else if castles.contains("K") {
        //     fen.castle_rights[Color::White.to_index()] = CastleRights::KingSide;
        // } else if castles.contains("Q") {
        //     fen.castle_rights[Color::White.to_index()] = CastleRights::QueenSide;
        // } else {
        //     fen.castle_rights[Color::White.to_index()] = CastleRights::NoRights;
        // }

        // if castles.contains("k") && castles.contains("q") {
        //     fen.castle_rights[Color::Black.to_index()] = CastleRights::Both;
        // } else if castles.contains("k") {
        //     fen.castle_rights[Color::Black.to_index()] = CastleRights::KingSide;
        // } else if castles.contains("q") {
        //     fen.castle_rights[Color::Black.to_index()] = CastleRights::QueenSide;
        // } else {
        //     fen.castle_rights[Color::Black.to_index()] = CastleRights::NoRights;
        // }

        // if let Ok(sq) = Square::from_str(&ep) {
        //     fen = fen.en_passant(Some(sq.get_file()));
        // }

        Ok(board)
    }
}