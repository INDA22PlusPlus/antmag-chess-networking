
use crate::piece::Definitions::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Content {
    Occupied(Piece),
    Empty
}
impl Content {
    pub fn get_symbol(&self) -> &'static str {
        match self {
            Content::Empty => "_",
            Content::Occupied(p) => {
                match p.color {
                    Color::White => {
                        match p.piece_type {
                            PieceType::Pawn => "♙",
                            PieceType::Bishop => "♗",
                            PieceType::Knight => "♘",
                            PieceType::Rook => "♖",
                            PieceType::Queen => "♕",
                            PieceType::King => "♔",
                        }
                    },
                    Color::Black => {
                        match p.piece_type {
                            PieceType::Pawn => "♟",
                            PieceType::Bishop => "♝",
                            PieceType::Knight => "♞",
                            PieceType::Rook => "♜",
                            PieceType::Queen => "♛",
                            PieceType::King => "♚",
                        }
                    }
                }
            }
        }
    }
}