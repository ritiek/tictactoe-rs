use crossterm::Result;
use tictactoe::{Grid, Side, TicTacToe, AI};

fn main() -> Result<()> {
    // let mut grid: Grid = Default::default();
    let mut grid = Grid::from(Side(3));
    grid.draw()?;
    let mut tictactoe = TicTacToe::from(grid, AI::Random).expect("could not initialize game");
    tictactoe.game_loop()?;
    Ok(())
}
