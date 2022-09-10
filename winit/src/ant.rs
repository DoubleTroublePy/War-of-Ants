use rand::distributions::{Distribution, Uniform};

use war_of_ants::Coordinate;
use war_of_ants::NEIGHBOURS; 
use war_of_ants::Direction;

/// formica 
pub struct Ant {
    id: i32,
    coordinate: Coordinate,
    lib_cof: i32,
    att_cof: i32,
    transport: bool,
    direction: Direction,
}

impl Ant {
    pub fn new(id_t:i32, coordinate_t: Coordinate, lib_cof_t: i32, att_cof_t: i32, direction_t: i16) -> Ant {
        Ant {
            id: id_t,
            coordinate: coordinate_t,
            lib_cof: lib_cof_t,
            att_cof: att_cof_t,
            transport: false,
            direction: Direction::new(direction_t, 7), // direction between 0, 8
        }   
    }

    pub fn update(&mut self, pheromones_mt: &Vec<Coordinate>) {
        const pos_l: usize = 4;
        
        // stupid an probably useless function that calculates if a value is in between a percent
        fn in_range(rnd: f64, probs: [f64; pos_l], i: usize) -> bool { //TODO: implement in more correct way this shit
                rnd > probs[i] && rnd < probs[i+1]
        }

        // calcolation of the absolute coordinatate of the neighboards tiles
        let f_neighbors = self.neighborhood(self.coordinate);
        
        // randominess
        let between = Uniform::from(0..100);
        let mut rng = rand::thread_rng();
        let mut probs: [f64; pos_l] = [0.0, 33.3, 66.6, 100.0];
        let rnd: f64 = (between.sample(&mut rng)) as f64;
        let mut sel = 0;
        
        for (i, _) in probs.iter().enumerate() {
            if in_range(rnd, probs, i) {
                sel = i+1;
                break;
            }
        }
        
        if sel != 0 {
            if sel == 1 {
                self.direction.sub(1)
            } else if sel == 3 {
                 self.direction.add(1)
            }
            self.coordinate = f_neighbors[self.direction.d as usize];
        }
    }

    /*
    pub fn update_v1(&mut self, pheromones_mt: &Vec<Coordinate>) {
        const pos_l: usize = 4;
        
        fn in_range(rnd: f64, probs: [f64; pos_l], i: usize) -> bool { //TODO: implement in more correct way this shit
                rnd > probs[i] && rnd < probs[i+1]
        }
        
        fn direction_update(i: usize, direction: i16) -> i16 {
            if i <= 4 {
                i as i16
            } else {
                (4-i) as i16
            }
        }

        let between = Uniform::from(0..100);
        let mut rng = rand::thread_rng();
        let f_neighbors: [Coordinate; 8]; 
        f_neighbors = self.neighborhood(self.coordinate);
        let mut probs: [f64; pos_l] = [0.0, 33.3, 66.6, 100.0]; // probs => probabilities
        /*
        for (i, f_neighbor) in f_neighbors.iter().enumerate() {
            if pheromones_mt.contains(&f_neighbor) {
                probs[i] = probs[i] + self.att_cof as f64;
            } else {
                probs[i] = probs[i]-self.att_cof as f64/7.0;
            }
        }
        */
        for (i, prob) in probs.iter().enumerate() {
            let rnd: f64 = (between.sample(&mut rng)) as f64;
            if i > pos_l { break }
            if in_range(rnd, probs, i) {
                if i < f_neighbors.len() {
                    self.coordinate = f_neighbors[(i as i16 + self.direction) as usize];
                } else {
                    self.coordinate = f_neighbors[(self.direction) as usize];
                }
                self.direction = direction_update(i, self.direction)+i as i16;
                println!("{} {}", self.direction+1, i+1);
            }
        }
    }
    */

    pub fn coordinate_get(&self) -> (i16, i16) {
        (self.coordinate.x, self.coordinate.y)
    }

    pub fn coordinate_set(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }

    fn neighborhood(&self, coordinate: Coordinate) -> [Coordinate; 8] {
        let mut f_neighbors = [Coordinate::new(0, 0); 8];
        for (i, neighbour) in NEIGHBOURS.iter().enumerate() {
            f_neighbors[i] = coordinate.add(neighbour);
        }
        f_neighbors
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighborhood () {
        let ant = Ant::new(1, Coordinate::new(0, 0), 30, 10, 0);

        let coordinate: Coordinate = Coordinate::new(10 ,10);
        let f_coordinate: [Coordinate; 8] = [
                    Coordinate::new(11, 10),
                    Coordinate::new(11, 11),
                    Coordinate::new(10, 11),
                    Coordinate::new( 9, 11),
                    Coordinate::new( 9, 10),
                    Coordinate::new( 9,  9),
                    Coordinate::new(10,  9),
                    Coordinate::new(11,  9),
        ];

        assert_eq!(f_coordinate, ant.neighborhood(coordinate))
    }
}
