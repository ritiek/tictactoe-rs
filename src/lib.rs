pub mod ai;
pub mod game;
pub mod grid;

pub use ai::AI;
pub use game::TicTacToe;
pub use grid::Grid;

use std::ops::Add;

#[derive(Debug)]
pub struct Side(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Zero,
    Cross,
}

impl Player {
    pub fn to_char(&self) -> char {
        match self {
            Self::Zero => '0',
            Self::Cross => 'X',
        }
    }
}

pub enum InputEvent {
    Direction(Direction),
    Mark,
    Quit,
}

#[derive(Debug)]
// Marked this as non-exhaustive because it's possible to have variants for diagonal
// movements.
#[non_exhaustive]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_relative_coords(&self) -> Coordinates {
        match &self {
            Direction::Up => Coordinates { x: 0, y: -1 },
            Direction::Down => Coordinates { x: 0, y: 1 },
            Direction::Left => Coordinates { x: -1, y: 0 },
            Direction::Right => Coordinates { x: 1, y: 0 },
            _ => panic!("diagonal movement is not yet implemented!"),
        }
    }
}
