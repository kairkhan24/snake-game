use crate::direction::Direction;

pub enum Command {
    Turn(Direction),
    Quit,
}