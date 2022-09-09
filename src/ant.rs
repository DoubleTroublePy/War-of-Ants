use rand::distributions::{Distribution, Uniform};

use war_of_ants::Coordinate;
use war_of_ants::NEIGHBOURS; 

pub struct Ant {
    id: i32,
    coordinate: Coordinate,
    lib_cof: i32,
    att_cof: i32,
    transport: bool,
}

impl Ant {
    pub fn new(id_t:i32, coordinate_t: Coordinate, lib_cof_t: i32, att_cof_f: i32) -> Ant {
        Ant {
            id: id_t,
            coordinate: coordinate_t,
            lib_cof: lib_cof_t,
            att_cof: att_cof_f,
            transport: false,
        }   
    }

    pub fn update(&mut self, pheromones_mt: &Vec<Coordinate>) {
        const pos_l: usize = 4;
        
        fn in_range(rnd: f64, probs: [f64; pos_l], i: usize) -> bool { //TODO: implement in more correct way this shit
                rnd > probs[i] && rnd > probs[i+1]
        }
        
        let between = Uniform::from(0..100);
        let mut rng = rand::thread_rng();
        let f_neighbors: [Coordinate; 3]; 
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
                self.coordinate = f_neighbors[i];
            }
        }
    }
    
    pub fn coordinate(&self) -> (i16, i16) {
        (self.coordinate.x, self.coordinate.y)
    }
    pub fn neighborhood(&self, coordinate: Coordinate) -> [Coordinate; 3] {
        let mut f_neighbors = [Coordinate::new(0, 0); 3];
        for (i, neighbour) in NEIGHBOURS.iter().enumerate() {
            f_neighbors[i] = coordinate.add(neighbour);
        }
        f_neighbors
    }

}

