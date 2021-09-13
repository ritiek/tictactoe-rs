// use std::fmt;
use std::io;
use std::io::{stdout, Write};
use std::ops;
use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i16,
    y: i16,
}

impl ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    Zero,
    Cross,
}

impl Player {
    fn to_char(&self) -> char {
        match self {
            Self::Zero => '0',
            Self::Cross => 'X',
        }
    }
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
    cursor: Coordinates,
    grid: Grid,
    // marked_positions: Vec<Coordinates>,
    marked_positions: HashMap<Coordinates, Player>,
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

    fn mark_at(&mut self, position: Coordinates, marker: char) -> io::Result<&Self> {
        // fn mark(mut self, position: Coordinates, marker: char) -> Result<Self, Box<dyn Error>> {
        let Side(side) = &self.side;
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
        Self { side: Side(7) }
    }
}

// impl fmt::Display for Grid {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         Ok(())
//     }
// }

// impl Error for Grid {}

impl Direction {
    fn get_relative_coords(&self) -> Coordinates {
        match &self {
            Direction::Up => Coordinates { x: 0, y: -1 },
            Direction::Down => Coordinates { x: 0, y: 1 },
            Direction::Left => Coordinates { x: -1, y: 0 },
            Direction::Right => Coordinates { x: 1, y: 0 },
            _ => panic!("Diagonal movement is not yet implemented!"),
        }
    }
}

impl TicTacToe {
    fn from(grid: Grid) -> Self {
        let initial_grid_coords = Coordinates { x: 0, y: 0 };
        Self::move_cursor_to_grid(&initial_grid_coords);
        let Side(side) = grid.side;
        let mut marked_positions: HashMap<Coordinates, Player> = HashMap::new();
        marked_positions.reserve(side.pow(2).into());
        Self {
            cursor: initial_grid_coords,
            grid: grid,
            marked_positions: marked_positions,
        }
    }

    fn handle_keyboard_input(&mut self) -> crossterm::Result<()> {
        let mut event: InputEvent;
        loop {
            event = self.read_input_event()?;
            match event {
                InputEvent::Direction(direction) => {
                    let Side(side) = &self.grid.side;
                    let mut grid_coords = self.cursor + direction.get_relative_coords();
                    if grid_coords.x >= *side as i16 {
                        grid_coords.x = *side as i16 - 1;
                    }
                    if grid_coords.y >= *side as i16 {
                        grid_coords.y = *side as i16 - 1;
                    }
                    if grid_coords.x < 0 {
                        grid_coords.x = 0;
                    }
                    if grid_coords.y < 0 {
                        grid_coords.y = 0;
                    }
                    Self::move_cursor_to_grid(&grid_coords);
                    self.cursor = grid_coords;
                },
                InputEvent::Mark => {
                    self.mark_cross()?;
                    // The cursor automatically increments in x-axis after placing the mark.
                    // Let's decrement the cursor back to bring back to its original position.
                    Self::move_cursor_to_grid(&self.cursor);
                },
                InputEvent::Quit => {
                    break;
                },
            }
        }
        Ok(())
    }

    fn read_input_event(&self) -> crossterm::Result<InputEvent> {
        loop {
            if let Event::Key(k) = read()? {
                match k.code {
                    event::KeyCode::Enter => return Ok(InputEvent::Mark),
                    event::KeyCode::Char('w') => return Ok(InputEvent::Direction(Direction::Up)),
                    event::KeyCode::Char('s') => return Ok(InputEvent::Direction(Direction::Down)),
                    event::KeyCode::Char('a') => return Ok(InputEvent::Direction(Direction::Left)),
                    event::KeyCode::Char('d') => return Ok(InputEvent::Direction(Direction::Right)),
                    event::KeyCode::Esc => return Ok(InputEvent::Quit),
                    _ => {},
                };
            };
        }
    }

    fn handle_input_event(&mut self, event: InputEvent) {
            // execute!(
            //     stdout(),
            //     cursor::MoveTo( as u16, as u16),
            //     SetForegroundColor(Color::Red),
            //     SetBackgroundColor(Color::White),
            //     Print(marker),
            //     ResetColor
            // )?;
    }

    fn move_cursor_to_grid(position: &Coordinates) -> crossterm::Result<()> {
        let screen_coords = Grid::grid_coords_to_screen_coords(position);
        Ok(Self::move_cursor_to_screen(&screen_coords)?)
    }

    fn move_cursor_to_screen(position: &Coordinates) -> crossterm::Result<()> {
        execute!(
            stdout(),
            cursor::MoveTo(position.x as u16, position.y as u16)
        )?;
        Ok(())
    }

    fn mark(&mut self, player: Player) -> io::Result<&Self> {
        self.grid.mark_at(self.cursor, player.to_char())?;
        self.marked_positions.insert(self.cursor, player);
        let player_has_won = self.check_for_victory(&player);
        if player_has_won {
            println!("Winner: {:?}", player);
        }
        let game_has_drawed = !self.grid_has_empty_boxes();
        if game_has_drawed {
            println!("Draw!");
        }

        Ok(self)
    }

    fn mark_cross(&mut self) -> io::Result<&Self> {
        Ok(self.mark(Player::Cross)?)
    }

    fn mark_zero(&mut self) -> io::Result<&Self> {
        Ok(self.mark(Player::Zero)?)
    }

    fn grid_has_empty_boxes(&self) -> bool {
        let Side(side) = self.grid.side;
        self.marked_positions.len() != side.pow(2).into()
    }

    fn check_for_victory(&self, player: &Player) -> bool {
        let Side(side) = self.grid.side;
        let mut victory = true;

        for x in 0..(side as i16) {
            victory = true;
            for y in 0..(side as i16) {
                if self.marked_positions.get(&Coordinates { x, y }) != Some(player) {
                    victory = false;
                    break;
                }
            }
            if victory {
                return true;
            }
        }

        for y in 0..(side as i16) {
            victory = true;
            for x in 0..(side as i16) {
                if self.marked_positions.get(&Coordinates { x, y }) != Some(player) {
                    victory = false;
                    break;
                }
            }
            if victory {
                return true;
            }
        }

        for z in 0..(side as i16) {
            victory = true;
            if self.marked_positions.get(&Coordinates { x: z, y: z }) != Some(player) {
                victory = false;
                break;
            }
        }
        if victory {
            return true;
        }

        for z in 0..(side as i16) {
            victory = true;
            if self.marked_positions.get(&Coordinates { x: z, y: z }) != Some(player) {
                victory = false;
                break;
            }
        }
        if victory {
            return true;
        }

        false
    }
}

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    let mut grid: Grid = Default::default();
    grid.draw();
    // let two_s = time::Duration::from_secs(2);
    // thread::sleep(two_s);
    // grid.mark_cross_at(Coordinates { x: 1, y: 0 });
    let mut game = TicTacToe::from(grid);
    game.handle_keyboard_input();
    Ok(())
}
