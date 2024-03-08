#[derive(PartialEq, Debug, Clone, Copy)]
enum Bearing {
    N, S, E, W
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct MarsRover {
    position: Position,
    bearing: Bearing,
}

impl MarsRover {
    fn new(position: Position, bearing: Bearing) -> Self {
        MarsRover { 
            position,
            bearing, 
        }
    }
    pub fn exec_move(&mut self, instructions: &str) {
        match instructions {
            "R" => {
                self.bearing = Bearing::E;
            },
            "RR" => {
                self.bearing = Bearing::S;
            },
            "RRR" => {
                self.bearing = Bearing::W;
            },
            "RRRR" => {
                self.bearing = Bearing::N;
            },
            _ => {
                self.position = Position {
                    x:0, 
                    y: instructions.len() as i32
                };
            }
        }
    } 

    pub fn position(&self) -> Position {
        self.position
    }
}

impl Default for MarsRover {
    fn default() -> MarsRover {
        MarsRover { 
            position: Position{x: 0, y: 0},
            bearing: Bearing::N, 
        }
    }
}

fn main() {
    println!("Hello, world!");
}

// -- Tests

#[cfg(test)]
mod tests {
    use crate::{Bearing, MarsRover, Position};

    #[test]
    fn it_moves_1_position() {
        let mut rover = MarsRover::default();
        rover.exec_move("M");
        
        assert_eq!(Position {x:0, y: 1} , rover.position());
    }

    #[test]
    fn it_moves_3_position() {
        let mut rover = MarsRover::default();
        rover.exec_move("MMM");
        
        assert_eq!(Position {x:0, y: 3} , rover.position());
    }

    #[test]
    fn it_rotates_right_once() {
        let mut rover = MarsRover::default();
        rover.exec_move("R");
        
        assert_eq!(MarsRover::new(Position {x:0, y: 0}, Bearing::E) , rover);
    }

    #[test]
    fn it_rotates_right_twice() {
        let mut rover = MarsRover::default();
        rover.exec_move("RR");
        
        assert_eq!(MarsRover::new(Position {x:0, y: 0}, Bearing::S) , rover);
    }

    #[test]
    fn it_rotates_right_three_times() {
        let mut rover = MarsRover::default();
        rover.exec_move("RRR");
        
        assert_eq!(MarsRover::new(Position {x:0, y: 0}, Bearing::W) , rover);
    }

    #[test]
    fn it_rotates_right_four_times() {
        let mut rover = MarsRover::default();
        rover.exec_move("RRRR");
        
        assert_eq!(MarsRover::new(Position {x:0, y: 0}, Bearing::N) , rover);
    }
}