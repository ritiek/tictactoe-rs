use tictactoe::{Side, Coordinates, Player, AI};
use std::collections::HashMap;

#[test]
fn get_marker() {
    let side = Side(3);
    let marked_positions: HashMap<Coordinates, Player> = [
        (Coordinates { x: 1, y: 0 }, Player::Cross),
        (Coordinates { x: 2, y: 1 }, Player::Zero),
        (Coordinates { x: 1, y: 2 }, Player::Cross),
    ].iter().cloned().collect();
    let marker = AI::Random.get_marker(&marked_positions, &side);
    let positions: Vec<Coordinates> = marked_positions.keys().cloned().collect();
    let is_new_move = !positions.contains(&marker);
    assert!(is_new_move)
}
