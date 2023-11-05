use std::collections::HashMap;
use std::fmt::Display;

use crate::board::piece::PieceType;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum MoveType {
    Normal,
    Castle { kingside: bool },
    DoublePush,
    EnPassant,
    Promotion(PieceType),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(start_square: usize, end_square: usize, move_type: MoveType) -> Move {
        if start_square > 63 {
            panic!("start square can't be larger than 63");
        }
        if end_square > 63 {
            panic!("end square can't be larger than 63");
        }
        Move {
            from: start_square,
            to: end_square,
            move_type,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_chars = HashMap::from([(PieceType::Knight, 'n'), (PieceType::Bishop, 'b'), (PieceType::Rook, 'r'), (PieceType::Queen, 'q')]);
        let files = "abcdefgh";
        let start_file = files.chars().nth(self.from % 8).unwrap();
        let start_rank = (8 - self.from / 8).to_string();
        let end_file = files.chars().nth(self.to % 8).unwrap();
        let end_rank = (8 - self.to / 8).to_string();
        write!(f, "{}{}{}{}", start_file, start_rank, end_file, end_rank).unwrap();
        if let MoveType::Promotion(piece) = self.move_type {
            write!(f, "{}", piece_chars.get(&piece).unwrap()).unwrap();
        }
        Ok(())
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let from_order = self.from.cmp(&other.from);
        let to_order = self.to.cmp(&other.to);

        from_order.then(to_order)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_to_long_algebraic_notation() {
        let mov = Move::new(0, 4, MoveType::Normal);
        assert_eq!(mov.to_string(), "a8e8");
    }
}