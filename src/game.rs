use super::ai::AI;
use super::grid::Grid;
use super::{Coordinates, Direction, InputEvent, Player, Side};

use crossterm::event::{read, Event};
use crossterm::{cursor, event, execute};
use std::collections::HashMap;
use std::io;
use std::io::stdout;

pub struct TicTacToe {
    cursor: Coordinates,
    grid: Grid,
    ai_algo: AI,
    marked_positions: HashMap<Coordinates, Player>,
}

impl TicTacToe {
    pub fn from(grid: Grid, ai_algo: AI) -> crossterm::Result<Self> {
        let initial_grid_coords = Coordinates { x: 0, y: 0 };
        Self::move_cursor_to_grid(&initial_grid_coords)?;
        let Side(side) = grid.side;
        let marked_positions: HashMap<Coordinates, Player> =
            HashMap::with_capacity(side.pow(2).into());
        Ok(Self {
            cursor: initial_grid_coords,
            grid: grid,
            ai_algo: ai_algo,
            marked_positions: marked_positions,
        })
    }

    pub fn handle_keyboard_input(&mut self) -> crossterm::Result<()> {
        let mut event: InputEvent;
        loop {
            event = self.read_input_event()?;
            match event {
                InputEvent::Direction(direction) => {
                    self.handle_direction(direction)?;
                }
                InputEvent::Quit => {
                    break;
                }
                InputEvent::Mark => {
                    let marked = self.mark_cross();
                    // Let's ignore if the player sets a mark at an already marked position.
                    if marked.is_err() {
                        continue;
                    }
                    let player_has_won = self.check_for_victory(&Player::Cross);
                    if player_has_won {
                        println!("Winner Cross");
                        break;
                    }
                    let game_has_drawed = !self.grid_has_empty_boxes();
                    if game_has_drawed {
                        println!("Draw!");
                        break;
                    }
                    let player_cursor = self.cursor;
                    let ai_cursor = self
                        .ai_algo
                        .get_marker(&self.marked_positions, &self.grid.side);
                    self.set_cursor_to_grid(&ai_cursor)?;
                    self.mark_zero()?;
                    let ai_has_won = self.check_for_victory(&Player::Zero);
                    if ai_has_won {
                        println!("Winner Zero");
                        break;
                    }
                    self.set_cursor_to_grid(&player_cursor)?;
                }
            }
        }
        Ok(())
    }

    fn handle_direction(&mut self, direction: Direction) -> crossterm::Result<()> {
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
        Self::move_cursor_to_grid(&grid_coords)?;
        self.cursor = grid_coords;
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
                    event::KeyCode::Char('d') => {
                        return Ok(InputEvent::Direction(Direction::Right))
                    }
                    event::KeyCode::Esc => return Ok(InputEvent::Quit),
                    _ => {}
                };
            };
        }
    }

    fn set_cursor_to_grid(&mut self, position: &Coordinates) -> crossterm::Result<()> {
        Self::move_cursor_to_grid(position)?;
        self.cursor = position.clone();
        Ok(())
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

    fn mark(&mut self, player: Player) -> crossterm::Result<&Self> {
        if self.marked_positions.get(&self.cursor).is_none() {
            self.grid.mark_at(self.cursor, player.to_char())?;
            self.marked_positions.insert(self.cursor, player);
            // The cursor automatically increments in x-axis after placing the mark.
            // Let's decrement the cursor back to bring back to its original position.
            Self::move_cursor_to_grid(&self.cursor)?;
            Ok(self)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "the position has already been marked",
            ))
        }
    }

    fn mark_cross(&mut self) -> crossterm::Result<&Self> {
        Ok(self.mark(Player::Cross)?)
    }

    fn mark_zero(&mut self) -> crossterm::Result<&Self> {
        Ok(self.mark(Player::Zero)?)
    }

    fn grid_has_empty_boxes(&self) -> bool {
        let Side(side) = self.grid.side;
        self.marked_positions.len() != side.pow(2).into()
    }

    fn check_for_victory(&self, player: &Player) -> bool {
        let Side(side) = self.grid.side;
        let mut victory = true;

        // Check if any vertical pattern is complete
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

        // Check if any horizontal pattern is complete
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

        // Check if top-left to bottom-right pattern is complete
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

        // Check if bottom-left to top-right pattern is complete
        for x in 0..(side as i16) {
            victory = true;
            let y = side as i16 - x - 1;
            if self.marked_positions.get(&Coordinates { x, y }) != Some(player) {
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
