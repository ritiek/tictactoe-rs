use std::io;
use std::io::{stdout, Write};
use std::fmt;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    cursor,
    event,
    terminal,
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
        for y in 0..side {
            for x in 0..(side-1) {
                execute!(
                    stdout(),
                    cursor::MoveTo((x*4)+3, y),
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(Color::White),
                    Print("|"),
                    ResetColor
                )?;
            }
        }
        cursor::MoveTo(0, 3);
        Ok(self)
    }

    fn mark_cross(mut self, position: Coordinates) -> io::Result<Self> {
    // fn mark_cross(mut self, position: Coordinates) -> Result<Self, Box<dyn Error>> {
        self.mark(position, 'X')
    }

    fn mark_zero(mut self, position: Coordinates) -> io::Result<Self> {
    // fn mark_zero(mut self, position: Coordinates) -> Result<Self, Box<dyn Error>> {
        self.mark(position, '0')
    }

    fn mark(mut self, position: Coordinates, marker: char) -> io::Result<Self> {
    // fn mark(mut self, position: Coordinates, marker: char) -> Result<Self, Box<dyn Error>> {
        let Side(side) = &self.side;
        let position = {
            if side >= &position.x && side >= &position.y {
                Ok(position)
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "position coordinates are out of bounds"))
            }
        }?;
        self.marked_positions.push(position);
        Ok(self)
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
