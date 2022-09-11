#![deny(clippy::all)]
#![forbid(unsafe_code)]


#[derive(Debug)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

impl Coordinate {
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

#[derive(Copy, Clone)]
pub struct Direction {
    pub d: i16, // direction 
    n: i16, // nugmber of directions
}

impl Direction {
    pub fn new(d_f: i16, n_f: i16) -> Direction {
        Direction {
            d: d_f,
            n: n_f,
        }
    }

    pub fn add(&mut self, i: i16) {
        if self.d+i > self.n {
            self.d = i - (self.n - self.d) - 1;
        } else {
            self.d = self.d + i;
        }
    }
    pub fn sub(&mut self, i: i16) {
        if self.d-i < 0 {
           self.d = i-1;
        } else {
            self.d = self.d-i;
        }
    }
}

// start > N -> O -> S -> E -
pub const NEIGHBOURS: [Coordinate; 8] = [
    Coordinate { x:  1, y:  0},  // 1 -> N
    Coordinate { x:  1, y:  1}, // 2 -> NO
    Coordinate { x:  0, y:  1}, // 3 -> O
    Coordinate { x: -1, y:  1}, // 4 -> SO
    Coordinate { x: -1, y:  0}, // 5 -> S
    Coordinate { x: -1, y: -1}, // 6 -> SE
    Coordinate { x:  0, y: -1}, // 7 -> E
    Coordinate { x:  1, y: -1}, // 8 -> NE

];

#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test]
    fn test_direction_add() {
        let mut direction = Direction::new(0, 7);
        direction.add(1);
        assert_eq!(direction.d, 1);

    }
        
    #[test]
    fn test_direction_add_overflow() {
        let mut direction = Direction::new(7, 7);
        direction.add(1);
        assert_eq!(direction.d, 0);
    }
    

    #[test]
    fn test_direction_sub() {
        let mut direction = Direction::new(7, 7);
        direction.sub(1);
        assert_eq!(direction.d, 6);

    }
    #[test]
    fn test_direction_sub_underflow() {
        let mut direction = Direction::new(0, 7);
        direction.sub(1);
        assert_eq!(direction.d, 0);

    }
}
