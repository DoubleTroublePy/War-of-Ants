pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

impl  Coordinate {
    pub fn new(x_t: i16, y_t: i16) -> Coordinate {
        Coordinate {
            x: x_t,
            y: y_t,
        }
    }

    pub fn update(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }

    pub fn add(&self, coordinate: &Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + coordinate.x,
            y: self.y + coordinate.y, 
        }
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinate {}

impl Copy for Coordinate {}

impl Clone for Coordinate {
    fn clone(&self) -> Coordinate {
        *self 
    }
}

// start > N -> O -> S -> E -
pub const NEIGHBOURS: [Coordinate; 3] = [
    Coordinate { x:  1, y:  1}, // 2 -> NO
    Coordinate { x:  1, y:  0},  // 1 -> N
    Coordinate { x:  1, y: -1}, // 8 -> NE
    ];

