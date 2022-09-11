use std::slice::ChunksExactMut;

use super::ant::Ant;
use war_of_ants::NEIGHBOURS;
use war_of_ants::Coordinate;

pub struct Canvas {
    HEIGHT: i16,
    WIDTH: i16,
}

impl Canvas {
    pub fn new(height: i16, width: i16) -> Canvas {
        Canvas {    
            HEIGHT: height,
            WIDTH: width,
        }
    }

    pub fn clear(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let ax = (i % self.WIDTH as usize) as i16;
            let ay = (i / self.WIDTH as usize) as i16;

            let rgba = [0x00, 0x00, 0x00, 0xff];
            pixel.copy_from_slice(&rgba);
        }   
    }
        
    pub fn draw_circle(&mut self, x: i16, y: i16,  dia: i16, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let ax = (i % self.WIDTH as usize) as i16;
            let ay = (i / self.WIDTH as usize) as i16;

            let rgba = [0xff, 0x00, 0x00, 0xff];

            let d = ((((x-ax) as i64).pow(2) + ((y-ay) as i64).pow(2)) as f32).sqrt() as i16;

            if d < dia as i16 {
                pixel.copy_from_slice(&rgba);
            }
        }   
    }
    
    pub fn draw_ants(&mut self, ants: &Vec<Ant>, frame: &mut [u8]) { 
        let mut cords: Vec<(i16, i16)> = Vec::new();
        for ant in ants {
            let coord = ant.coordinate_get();
            let coord = NEIGHBOURS[ant.direction_get() as usize].add(&Coordinate::new(coord.0, coord.1));
            cords.push((coord.x, coord.y));
            cords.push(ant.coordinate_get());
        }
        
        let rgba = [0xff, 0x00, 0x00, 0xff];
        
        for cord in cords {
            //let pos: usize =  (cord.0 as f64 + (self.WIDTH + 4) as f64 * cord.1 as f64) as usize;
            let jmp = self.WIDTH as f64 * (cord.1 - 1) as f64 + (cord.0 as f64);
            let pos: usize = (4.0 * jmp) as usize;
            if pos+3 < frame.len() {
                frame[pos]   = 255; // r
                frame[pos+1] =   0; // g
                frame[pos+2] =   0; // b
                frame[pos+3] = 255; // a
            }
        }
        
        /*
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let ax = (i % self.WIDTH as usize) as i16;
            let ay = (i / self.WIDTH as usize) as i16;

            let rgba = [0xff, 0x00, 0x00, 0xff];
            for (x, y) in &cords {
                if  ax == *x && ay == *y {
                    pixel.copy_from_slice(&rgba);
                }
            }
        }
        */

    }

    pub fn draw_ant(&mut self, x: i16, y: i16, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let ax = (i % self.WIDTH as usize) as i16;
            let ay = (i / self.WIDTH as usize) as i16;

            let rgba = [0xff, 0x00, 0x00, 0xff];

            if  ax == x && ay == y {
                pixel.copy_from_slice(&rgba);
            }
        } 
    }

}


