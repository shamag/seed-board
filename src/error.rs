use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChessError {


    #[error("Fen contains incorrect data")]
    WrongFen,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}