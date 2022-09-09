#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod canvas; 

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

struct Coordinate {
    x: Vec<i16>,
    y: Vec<i16>, 
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate {
            x: Vec::new(),
            y: Vec::new(),
        }
    }
    fn len(&self) -> usize {
        self.x.len()
    }
    
    fn add(&mut self, x: i16, y: i16) {
        if self.x.len() > 1000 { 
            self.x.remove(0);  
            self.y.remove(0);
        }
        self.x.push(x);
        self.y.push(y);
    }
}



fn draw(frame: &mut [u8], input: &WinitInputHelper, coordinate: &mut Coordinate) {
    let mut canvas = canvas::Canvas::new(HEIGHT as i16, WIDTH as i16, frame);
    canvas.clear();
    canvas.draw_circle(100, 100, 10);
    canvas.draw_circle(200, 200, 50);
    if input.mouse_held(0) {
        match input.mouse() {
            Some(cord) => {
                coordinate.add(cord.0 as i16, cord.1 as i16); 
                canvas.draw_circle(cord.0 as i16, cord.1 as i16, 5)
            },
            None => (),
        }
        
    }

    for i in 0..coordinate.len() {
        canvas.draw_circle(coordinate.x[i as usize], coordinate.y[i as usize], 5)
    }
}


fn main() -> Result<(), Error> {
    let mut coordinate = Coordinate::new();
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("War of Ants")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame(), &input, &mut coordinate);
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
        }
        // Handle input events
        if input.update(&event) {
            // Close events;
            if input.key_pressed(VirtualKeyCode::Escape)  
                || input.key_pressed(VirtualKeyCode::Q)
                || input.quit() 
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }   

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

