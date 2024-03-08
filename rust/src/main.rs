enum Bearing {
    N, S, E, W
}
struct Position {
    x: i32,
    y: i32,
}
struct MarsRover {
    position: Position,
    bearing: Bearing,
}

impl MarsRover {
    fn new() -> Self {
        MarsRover { 
            position: Position{x: 0, y: 0},
            bearing: Bearing::N, 
        }
    }
    // fn move(move) 
}

fn main() {
    println!("Hello, world!");
}

// -- Tests

#[cfg(test)]
mod tests {
    use crate::{MarsRover, Position};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_moves_1_position() {
        let mars = MarsRover::new();
        mars.move("M");
        
        assert_eq!(Position {x:0, y: 1} , mars.position());
    }
}