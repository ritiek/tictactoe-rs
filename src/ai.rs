use super::{Coordinates, Player, Side};

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum AI {
    Random,
    MiniMax,
}

impl AI {
    pub fn get_marker(
        &self,
        marked_positions: &HashMap<Coordinates, Player>,
        side: &Side,
    ) -> Coordinates {
        match self {
            Self::Random => Self::random_mark(marked_positions, side),
            // Self::MiniMax => Self::minimax_mark(marked_positions, side),
            _ => panic!("AI algorithm {:?} is not implemented yet!", self),
        }
    }

    fn random_mark(marked_positions: &HashMap<Coordinates, Player>, side: &Side) -> Coordinates {
        let Side(side) = side;
        let all_moves = {
            let mut all_moves = HashSet::with_capacity(side.pow(2).into());
            for x in 0..(*side as i16) {
                for y in 0..(*side as i16) {
                    all_moves.insert(Coordinates { x, y });
                }
            }
            all_moves
        };
        let marked_positions: HashSet<Coordinates> = marked_positions.keys().cloned().collect();
        let remaining_positions: Vec<&Coordinates> = all_moves
            .difference(&marked_positions)
            .into_iter()
            .collect();
        let result = remaining_positions.choose(&mut rand::thread_rng()).unwrap();
        *result.clone()
    }

    // fn minimax_mark(
    //     marked_positions: &HashMap<Coordinates, Player>,
    //     side: &Side,
    // ) -> Coordinates {
    //     // FIXME
    //     Self::random_mark(marked_positions, side)
    // }
}
