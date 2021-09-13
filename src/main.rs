use crossterm::Result;
use tictactoe::{Grid, Side, TicTacToe, AI};

fn main() -> Result<()> {
    // let mut grid: Grid = Default::default();
    let mut grid = Grid::from(Side(3));
    grid.draw()?;
    let mut game = TicTacToe::from(grid, AI::Random).expect("could not initialize game");
    game.handle_keyboard_input()?;
    Ok(())
}
