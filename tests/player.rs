use tictactoe::Player;

#[test]
fn cross_to_char() {
    assert_eq!(Player::Cross.to_char(), 'X')
}

#[test]
fn zero_to_char() {
    assert_eq!(Player::Zero.to_char(), '0')
}
