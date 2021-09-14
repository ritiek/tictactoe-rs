use tictactoe::{Coordinates, Direction};

#[test]
fn add_operator() {
    let c1 = Coordinates { x: 5, y: 10 };
    let c2 = Coordinates { x: 20, y: 30 };
    assert_eq!(c1 + c2, Coordinates { x: 25, y: 40 });
}

#[test]
fn direction_relative_coords_up() {
    assert_eq!(
        Direction::Up.get_relative_coords(),
        Coordinates { x: 0, y: -1 }
    );
}

#[test]
fn direction_relative_coords_down() {
    assert_eq!(
        Direction::Down.get_relative_coords(),
        Coordinates { x: 0, y: 1 }
    );
}

#[test]
fn direction_relative_coords_left() {
    assert_eq!(
        Direction::Left.get_relative_coords(),
        Coordinates { x: -1, y: 0 }
    );
}

#[test]
fn direction_relative_coords_right() {
    assert_eq!(
        Direction::Right.get_relative_coords(),
        Coordinates { x: 1, y: 0 }
    );
}
