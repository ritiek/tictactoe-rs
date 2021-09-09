// use std::fmt;
use std::io;
use std::io::{stdout, Write};
// use std::{thread, time};

use crossterm::event::{read, Event};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, event, execute, terminal, ExecutableCommand};

// trait Error: fmt::Debug + fmt::Display {}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

#[derive(Debug)]
struct Side(u16);

#[derive(Debug)]
struct Coordinates {
    x: i16,
    y: i16,
}

//0123456789
//.0...1...2.

// 0 |   | X
//   | X |
// 0 |   |

//   |   |
//-----------
// X |   | 0
//-----------
//   |   |

#[derive(Debug)]
struct Grid {
    side: Side,
}

#[derive(Debug)]
enum Player {
    Zero,
    Cross,
}

#[derive(Debug)]
// Marked this as non-exhaustive because it's possible to have variants for diagonal
// movements.
#[non_exhaustive]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    // UpLeft,
    // UpRight,
    // DownLeft,
    // DownRight,
}

enum InputEvent {
    Direction(Direction),
    Mark,
    Quit,
}

struct TicTacToe {
    grid: Grid,
    marked_positions: Vec<Coordinates>,
}

impl Grid {
    fn new(side: Side) -> Self {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        Self { side: side }
    }

    fn draw(&mut self) -> crossterm::Result<&mut Self> {
        let Side(side) = self.side;
        let grid_length = side * 4 - 1;
        let grid_background = std::iter::repeat(" ")
            .take(grid_length.into())
            .collect::<String>();
        for y in 0..side {
            execute!(
                stdout(),
                cursor::MoveTo(0, y),
                SetBackgroundColor(Color::White),
                Print(&grid_background),
                ResetColor
            )?;
            for x in 0..(side - 1) {
                let boundary_position = x * 4 + 3;
                execute!(
                    stdout(),
                    cursor::MoveTo(boundary_position, y),
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(Color::White),
                    Print("|"),
                    ResetColor
                )?;
            }
        }
        Ok(self)
    }

    fn grid_coords_to_screen_coords(position: &Coordinates) -> Coordinates {
        Coordinates {
            x: position.x * 4 + 1,
            y: position.y,
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self { side: Side(3) }
    }
}

// impl fmt::Display for Grid {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         Ok(())
//     }
// }

// impl Error for Grid {}

impl Direction {
    fn get_relative_coordinates(&self) -> Coordinates {
        match &self {
            Direction::Up => Coordinates { x: 0, y: 1 },
            Direction::Down => Coordinates { x: 0, y: -1 },
            Direction::Left => Coordinates { x: -1, y: 0 },
            Direction::Right => Coordinates { x: 1, y: 0 },
            _ => panic!("Diagonal movement is not yet implemented!"),
        }
    }
}

impl TicTacToe {
    fn from(grid: Grid) -> Self {
        Self {
            grid: grid,
            marked_positions: vec![],
        }
    }

    fn handle_player_input(&mut self) -> crossterm::Result<()> {
        loop {
            if let Event::Key(k) = read()? {
                let key = match k.code {
                    event::KeyCode::Enter => Some(InputEvent::Mark),
                    event::KeyCode::Char('w') => Some(InputEvent::Direction(Direction::Up)),
                    event::KeyCode::Char('s') => Some(InputEvent::Direction(Direction::Down)),
                    event::KeyCode::Char('a') => Some(InputEvent::Direction(Direction::Left)),
                    event::KeyCode::Char('d') => Some(InputEvent::Direction(Direction::Right)),
                    event::KeyCode::Esc => Some(InputEvent::Quit),
                    _ => None,
                };
            };
        }
        Ok(())
    }

    fn mark_cross_at(mut self, position: Coordinates) -> io::Result<Self> {
        // fn mark_cross(mut self, position: Coordinates) -> Result<Self, Box<dyn Error>> {
        self.mark_at(position, 'X')
    }

    fn mark_zero_at(mut self, position: Coordinates) -> io::Result<Self> {
        // fn mark_zero(mut self, position: Coordinates) -> Result<Self, Box<dyn Error>> {
        self.mark_at(position, '0')
    }

    fn mark_at(mut self, position: Coordinates, marker: char) -> io::Result<Self> {
        // fn mark(mut self, position: Coordinates, marker: char) -> Result<Self, Box<dyn Error>> {
        let Side(side) = &self.grid.side;
        let position = {
            if side >= &(position.x as u16) && side >= &(position.y as u16) {
                Ok(position)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "position coordinates are out of bounds from the grid area",
                ))
            }
        }?;
        let screen_coords = Grid::grid_coords_to_screen_coords(&position);
        self.marked_positions.push(position);
        execute!(
            stdout(),
            cursor::MoveTo(screen_coords.x as u16, screen_coords.y as u16),
            SetForegroundColor(Color::Red),
            SetBackgroundColor(Color::White),
            Print(marker),
            ResetColor
        )?;
        // FIXME: below is for testing only
        execute!(
            stdout(),
            cursor::MoveTo(10, 10),
            SetForegroundColor(Color::Red),
            SetBackgroundColor(Color::White),
            Print(marker),
            ResetColor
        )?;
        Ok(self)
    }

    fn check_for_victory() -> Option<Player> {
        // TODO
        Some(Player::Zero)
    }
}

fn main() -> crossterm::Result<()> {
    let mut grid: Grid = Default::default();
    grid.draw();
    // let two_s = time::Duration::from_secs(2);
    // thread::sleep(two_s);
    // grid.mark_cross_at(Coordinates { x: 1, y: 0 });
    let mut game = TicTacToe::from(grid);
    game.handle_player_input();
    Ok(())
}
