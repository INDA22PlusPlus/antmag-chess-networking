
pub mod Definitions {
    #[derive(Clone)]
    pub struct Move {
        pub move_vecs: Vec<(i32, i32)>,
        pub move_type: MoveType
    }

    #[derive(Copy, Clone)]
    pub enum MoveType {
        Once,
        Inf
    }


    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Color {
        Black,
        White
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum PieceType {
        Pawn,
        Bishop,
        Knight,
        Rook,
        Queen,
        King,

    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Piece {
        pub color: Color,
        pub piece_type: PieceType,
        pub times_moved: u32,
    }


    impl Piece {
        pub fn get_moves(&self) -> Move {
            match self.piece_type {
                PieceType::Pawn => {
                    Move {
                        move_vecs: vec![(1, 0)],
                        move_type: MoveType::Once
                    }
                },
                PieceType::Bishop => {
                    Move {
                        move_vecs: vec![(1, 1), (1, -1), (-1, 1), (-1, -1)],
                        move_type: MoveType::Inf
                    }
                },
                PieceType::Knight => {
                    Move {
                        move_vecs: vec![(2, 1), (2, -1), (1, 2), (1, -2), (-2, 1), (-2, -1), (-1, 2), (-1, -2)],
                        move_type: MoveType::Once 
                    }
                },
                PieceType::Rook => {
                    Move {
                        move_vecs: vec![(0, 1), (0, -1), (1, 0), (0, -1)],
                        move_type: MoveType::Inf
                    }
                },
                PieceType::Queen => {
                    Move {
                        move_vecs: vec![(0, 1), (0, -1), (1, 0), (1, 1), (1, -1), (-1, 0), (-1, 1), (-1, -1)],
                        move_type: MoveType::Inf
                    }
                },
                PieceType::King => {
                    Move {
                        move_vecs: vec![(0, 1), (0, -1), (1, 0), (1, 1), (1, -1), (-1, 0), (-1, 1), (-1, -1)],
                        move_type: MoveType::Once
                    }
                }
            }
        }
    }

}

