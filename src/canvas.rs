#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub struct Canvas <'a> {
    FRAME: &'a mut [u8],
    HEIGHT: i16,
    WIDTH: i16,
}

impl Canvas <'_>{
    pub fn new(height: i16, width: i16, frame: &mut [u8]) -> Canvas {
        Canvas {
            FRAME: frame,
            HEIGHT: height,
            WIDTH: width,
        }
    }
    
    pub fn clear(&mut self) {
        for (i, pixel) in self.FRAME.chunks_exact_mut(4).enumerate() {
            let ax = (i % self.WIDTH as usize) as i16;
            let ay = (i / self.WIDTH as usize) as i16;

            let rgba = [0x00, 0x00, 0x00, 0xff];
            pixel.copy_from_slice(&rgba);
        }   
    }
        
    pub fn draw_circle(&mut self, x: i16, y: i16,  dia: i16) {
        for (i, pixel) in self.FRAME.chunks_exact_mut(4).enumerate() {
            let ax = (i % self.WIDTH as usize) as i16;
            let ay = (i / self.WIDTH as usize) as i16;

            let rgba = [0xff, 0x00, 0x00, 0xff];

            let d = ((((x-ax) as i64).pow(2) + ((y-ay) as i64).pow(2)) as f32).sqrt() as i16;

            if d < dia as i16 {
                pixel.copy_from_slice(&rgba);
            }
        }   
    }

    pub fn draw_ant(&mut self, x: i16, y: i16) {
        for (i, pixel) in self.FRAME.chunks_exact_mut(4).enumerate() {
            let ax = (i % self.WIDTH as usize) as i16;
            let ay = (i / self.WIDTH as usize) as i16;

            let rgba = [0xff, 0x00, 0x00, 0xff];

            if  ax == x && ay == y {
                pixel.copy_from_slice(&rgba);
            }
        } 
    }

}

