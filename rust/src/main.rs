enum Bearing {
    N, S, E, W
}
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
struct MarsRover {
    position: Position,
    _bearing: Bearing,
}

impl MarsRover {
    fn new() -> Self {
        MarsRover { 
            position: Position{x: 0, y: 0},
            _bearing: Bearing::N, 
        }
    }
    pub fn exec_move(&mut self, instructions: &str) {
        self.position = Position {x:0, y: instructions.len() as i32};
    } 

    pub fn position(&self) -> Position {
        self.position
    }
}

fn main() {
    println!("Hello, world!");
}

// -- Tests

#[cfg(test)]
mod tests {
    use crate::{MarsRover, Position};

    #[test]
    fn it_moves_1_position() {
        let mut rover = MarsRover::new();
        rover.exec_move("M");
        
        assert_eq!(Position {x:0, y: 1} , rover.position());
    }

    #[test]
    fn it_moves_3_position() {
        let mut rover = MarsRover::new();
        rover.exec_move("MMM");
        
        assert_eq!(Position {x:0, y: 3} , rover.position());
    }
}