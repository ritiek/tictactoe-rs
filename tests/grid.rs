use tictactoe::{Coordinates, Grid};

#[test]
fn grid_coords_to_screen_coords() {
    let grid_coords = Coordinates { x: 2, y: 1 };
    let screen_coords = Grid::grid_coords_to_screen_coords(&grid_coords);
    assert_eq!(screen_coords, Coordinates { x: 9, y: 1 });
}
