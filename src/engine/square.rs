use thiserror::Error;
use std::str::FromStr;
use crate::error::ChessError;


#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
pub struct Square {
    pub row: PositionRow,
    pub column: PositionColumn
}

#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
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

#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
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
impl FromStr for Square {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(ChessError::WrongFen);
        }
        let ch: Vec<char> = s.chars().collect();
        match ch[0] {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => {},
            _ => {
                return Err(ChessError::WrongFen);
            }
        }
        match ch[1] {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {},
            _ => {
                return Err(ChessError::WrongFen);
            }
        };

        Ok(Self{
            row: ((ch[1] as i32) - ('1' as i32)).into(),
            column: ((ch[0] as i32) - ('a' as i32)).into(),
        })
    }
    }