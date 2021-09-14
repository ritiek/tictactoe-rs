use tictactoe::{Side, Coordinates, Player, Grid, AI, TicTacToe};
use crossterm::Result;
use std::collections::HashMap;

fn tictactoe_from_settings() -> Result<TicTacToe> {
    TicTacToe::from(Grid::from(Side(3)), AI::Random)
}

#[test]
fn settings() -> Result<()> {
    let tictactoe = tictactoe_from_settings()?;
    assert_eq!(tictactoe.cursor, Coordinates { x: 0, y: 0 });
    assert_eq!(tictactoe.grid, Grid::from(Side(3)));
    assert_eq!(tictactoe.ai_algo, AI::Random);
    assert_eq!(tictactoe.marked_positions, HashMap::new());
    Ok(())
}

#[test]
fn set_cursor_to_grid() -> Result<()> {
    let mut tictactoe = tictactoe_from_settings()?;
    let grid_coords = Coordinates { x: 2, y: 1 };
    tictactoe.set_cursor_to_grid(&grid_coords)?;
    assert_eq!(tictactoe.cursor, grid_coords);
    // Move the cursor as far in the bottom right corner as possible.
    // Otherwise the terminal test output gets messed up.
    TicTacToe::move_cursor_to_grid(&Coordinates { x: 1920, y: 1080 })?;
    Ok(())
}

#[test]
fn mark_and_overwrites() -> Result<()> {
    let mut tictactoe = tictactoe_from_settings()?;
    let grid_coords = Coordinates { x: 1, y: 1 };
    tictactoe.set_cursor_to_grid(&grid_coords)?;
    assert!(tictactoe.mark_cross().is_ok());
    // Marking again at the same grid coordinates should error out.
    assert!(tictactoe.mark_cross().is_err());
    assert!(tictactoe.mark_zero().is_err());
    // Move the cursor as far in the bottom right corner as possible.
    // Otherwise the terminal test output gets messed up.
    TicTacToe::move_cursor_to_grid(&Coordinates { x: 1920, y: 1080 })?;
    Ok(())
}

#[test]
fn marked_positions() -> Result<()> {
    let mut tictactoe = tictactoe_from_settings()?;
    tictactoe.set_cursor_to_grid(&Coordinates { x: 1, y: 1 })?;
    tictactoe.mark_cross()?;
    tictactoe.set_cursor_to_grid(&Coordinates { x: 2, y: 0 })?;
    tictactoe.mark_zero()?;
    tictactoe.set_cursor_to_grid(&Coordinates { x: 0, y: 0 })?;
    tictactoe.mark_cross()?;
    let marked_positions: HashMap<Coordinates, Player> = [
        (Coordinates { x: 1, y: 1 }, Player::Cross),
        (Coordinates { x: 2, y: 0 }, Player::Zero),
        (Coordinates { x: 0, y: 0 }, Player::Cross),
    ].iter().cloned().collect();
    assert_eq!(tictactoe.marked_positions, marked_positions);
    // Move the cursor as far in the bottom right corner as possible.
    // Otherwise the terminal test output gets messed up.
    TicTacToe::move_cursor_to_grid(&Coordinates { x: 1920, y: 1080 })?;
    Ok(())
}

#[test]
fn grid_has_empty_boxes() -> Result<()> {
    let mut tictactoe = tictactoe_from_settings()?;
    tictactoe.set_cursor_to_grid(&Coordinates { x: 2, y: 2 })?;
    tictactoe.mark_cross()?;
    tictactoe.set_cursor_to_grid(&Coordinates { x: 2, y: 1 })?;
    tictactoe.mark_zero()?;
    tictactoe.set_cursor_to_grid(&Coordinates { x: 1, y: 0 })?;
    tictactoe.mark_cross()?;
    assert!(tictactoe.grid_has_empty_boxes());
    // Move the cursor as far in the bottom right corner as possible.
    // Otherwise the terminal test output gets messed up.
    TicTacToe::move_cursor_to_grid(&Coordinates { x: 1920, y: 1080 })?;
    Ok(())
}

#[test]
fn grid_does_not_has_empty_boxes() -> Result<()> {
    let mut tictactoe = tictactoe_from_settings()?;
    let Side(side) = tictactoe.grid.side;
    let mut iteration = 0;
    for x in 0..(side as i16) {
        for y in 0..(side as i16) {
            tictactoe.set_cursor_to_grid(&Coordinates { x, y })?;
            if iteration % 2 == 0 {
                tictactoe.mark_cross()?;
            } else {
                tictactoe.mark_zero()?;
            }
            iteration += 1;
        }
    }
    assert!(!tictactoe.grid_has_empty_boxes());
    // Move the cursor as far in the bottom right corner as possible.
    // Otherwise the terminal test output gets messed up.
    TicTacToe::move_cursor_to_grid(&Coordinates { x: 1920, y: 1080 })?;
    Ok(())
}

#[test]
fn check_for_victory() -> Result<()> {
    let mut tictactoe = tictactoe_from_settings()?;
    let Side(side) = tictactoe.grid.side;
    for z in 0..(side as i16) {
        tictactoe.set_cursor_to_grid(&Coordinates { x: z, y: z })?;
        tictactoe.mark_cross()?;
    }
    assert!(tictactoe.check_for_victory(&Player::Cross));
    // Move the cursor as far in the bottom right corner as possible.
    // Otherwise the terminal test output gets messed up.
    TicTacToe::move_cursor_to_grid(&Coordinates { x: 1920, y: 1080 })?;
    Ok(())
}

#[test]
fn check_for_not_victory() -> Result<()> {
    let mut tictactoe = tictactoe_from_settings()?;
    let Side(side) = tictactoe.grid.side;
    for z in 0..(side as i16 - 1) {
        tictactoe.set_cursor_to_grid(&Coordinates { x: z, y: z })?;
        tictactoe.mark_cross()?;
    }
    assert!(!tictactoe.check_for_victory(&Player::Cross));
    // Move the cursor as far in the bottom right corner as possible.
    // Otherwise the terminal test output gets messed up.
    TicTacToe::move_cursor_to_grid(&Coordinates { x: 1920, y: 1080 })?;
    Ok(())
}
