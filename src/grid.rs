use super::{Coordinates, Side};

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, execute, terminal};
use std::io;
use std::io::stdout;

#[derive(Debug)]
pub struct Grid {
    pub side: Side,
}

impl Grid {
    pub fn from(side: Side) -> Self {
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        Self { side: side }
    }

    pub fn draw(&mut self) -> crossterm::Result<&mut Self> {
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

    pub fn grid_coords_to_screen_coords(position: &Coordinates) -> Coordinates {
        Coordinates {
            x: position.x * 4 + 1,
            y: position.y,
        }
    }

    pub fn mark_at(&mut self, position: Coordinates, marker: char) -> crossterm::Result<&Self> {
        // fn mark(mut self, position: Coordinates, marker: char) -> Result<Self, Box<dyn Error>> {
        let Side(side) = &self.side;
        let _position = {
            if side >= &(position.x as u16) && side >= &(position.y as u16) {
                Ok(position)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "position coordinates are out of bounds from the grid area",
                ))
            }
        }?;
        // Self::move_cursor_to_grid(&position);
        execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            SetBackgroundColor(Color::White),
            Print(marker),
            ResetColor
        )?;
        Ok(self)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::from(Side(3))
    }
}
