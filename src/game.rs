use crate::view_board::ViewBoard;
use crate::pieces::{Color};
use crate::engine::square::{Square};
use crate::error::{ChessError};
use std::str::FromStr;


#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum BoardStatus {
    Ongoing,
    Stalemate,
    Checkmate,
}
const PLAYERS: usize = 2;

#[derive(Clone, Debug)]
pub struct Game {
    pub board: ViewBoard,
    pub cur_color: Color,
    pub castle_rights: [CastleRights; PLAYERS],
    // pinned: BitBoard,
    // checkers: BitBoard,
    // hash: u64,
    pub en_passant: Option<Square>,
}
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Move{
    Capture(usize, usize),
    Move(usize, Square)
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum CastleRights {
    NoRights,
    KingSide,
    QueenSide,
    Both,
}

impl Default for Game {
    #[inline]
    fn default() -> Self {
        Self::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Valid Position")
    }
}

impl Game {
    pub fn perfome_move(&self, m: Move) -> Game {
        let mut new_state = self.clone();
        match m {
            Move::Capture(p1, p2) => {
                new_state.board.pieces[p1].position = self.board.pieces[p2].position;
                new_state.board.pieces.remove(p2);
            },
            Move::Move(p1, pos) => {
                new_state.board.pieces[p1].position = pos;
            }
        }
        new_state.cur_color = new_state.cur_color.next();
        new_state
    }
}

impl FromStr for Game {
    type Err = ChessError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // let mut cur_row = PositionRow::Eight;
        // let mut cur_column = PositionColumn::A;
        let mut board = match ViewBoard::from_str(value) {
            Ok(b) => b,
            Err(_)=> return Err(ChessError::WrongFen)
        };;
        let mut game = Self{
            board: board,
            castle_rights: [CastleRights::NoRights, CastleRights::NoRights],
            cur_color: Color::White,
            en_passant: None
        };


        let tokens: Vec<&str> = value.split(' ').collect();
        if tokens.len() < 4 {
            return Err(ChessError::WrongFen);
        }

        let pieces = tokens[0];
        let side = tokens[1];
        let castles = tokens[2];
        let ep = tokens[3];
        let mut row = 7;
        let mut column = 0;

        match side {
            "w" | "W" => game.cur_color = Color::White,
            "b" | "B" => game.cur_color = Color::White,
            _ => {
                return Err(ChessError::WrongFen)
            }
        };

        if castles.contains("K") && castles.contains("Q") {
            game.castle_rights[0] = CastleRights::Both;
        } else if castles.contains("K") {
            game.castle_rights[0] = CastleRights::KingSide;
        } else if castles.contains("Q") {
            game.castle_rights[0] = CastleRights::QueenSide;
        } else {
            game.castle_rights[0] = CastleRights::NoRights;
        }

        if castles.contains("k") && castles.contains("q") {
            game.castle_rights[1] = CastleRights::Both;
        } else if castles.contains("k") {
            game.castle_rights[1] = CastleRights::KingSide;
        } else if castles.contains("q") {
            game.castle_rights[1] = CastleRights::QueenSide;
        } else {
            game.castle_rights[1] = CastleRights::NoRights;
        }

        if let Ok(sq) = Square::from_str(&ep) {
            game.en_passant = Some(sq);
        }


        Ok(game)
    }
}
