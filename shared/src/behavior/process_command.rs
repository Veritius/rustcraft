use std::ops::{Add, Sub};
use crate::protocol::{KeyCommand, Position};

const SQUARE_SPEED: f32 = 3.0;

pub fn process_command(key_command: &KeyCommand, position: &mut Position) {
    if *key_command.w {
        *position.y = position.y.sub(SQUARE_SPEED);
    }
    if *key_command.s {
        *position.y = position.y.add(SQUARE_SPEED);
    }
    if *key_command.a {
        *position.x = position.x.sub(SQUARE_SPEED);
    }
    if *key_command.d {
        *position.x = position.x.add(SQUARE_SPEED);
    }
}
