use std::fmt;
use std::io;
use std::io::{stdout, Write};

use crossterm::{
    cursor, event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand,
};

// trait Error: fmt::Debug + fmt::Display {}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

#[derive(Debug)]
struct Side(u16);

#[derive(Debug)]
struct Coordinates {
    x: u16,
    y: u16,
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
    marked_positions: Vec<Coordinates>,
}

impl Grid {
    fn new(side: Side) -> Self {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        Grid {
            side: side,
            marked_positions: vec![],
        }
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
        let Side(side) = &self.side;
        let position = {
            if side >= &position.x && side >= &position.y {
                Ok(position)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "position coordinates are out of bounds",
                ))
            }
        }?;
        self.marked_positions.push(position);
        Ok(self)
    }

    fn grid_coords_to_screen_coords(position: Coordinates) -> Coordinates {
        Coordinates {
            x: position.x * 4 + 2,
            y: position.y,
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            side: Side(3),
            marked_positions: vec![],
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

// impl Error for Grid {}

fn main() -> crossterm::Result<()> {
    let mut grid: Grid = Default::default();
    grid.draw();
    Ok(())
}
