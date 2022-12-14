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
mod ant;
use ant::Ant;

use war_of_ants::Coordinate; 

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

// drawing main
fn draw(frame: &mut [u8], input: &WinitInputHelper) {
    let mut canvas = canvas::Canvas::new(WIDTH as i16, HEIGHT as i16);
    canvas.clear(frame);
}


fn main() -> Result<(), Error> {
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

    let pheromones_mt: Vec<Coordinate> = Vec::new();
    let mut ants: Vec<Ant> = Vec::new();
    for i in 0..100000 {
        // randominess
        use rand::distributions::{Distribution, Uniform};
        let between = Uniform::from(0..8);
        let mut rng = rand::thread_rng();
        let rnd: i16 = (between.sample(&mut rng)) as i16;
        let mut sel = 0;
        ants.push(Ant::new(i, Coordinate::new((WIDTH/2) as i16, (HEIGHT/2) as i16), 30, 10, rnd));
    }   

    let mut canvas = canvas::Canvas::new(HEIGHT as i16, WIDTH as i16);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
            // draw(pixels.get_frame(), &input);
        if let Event::RedrawRequested(_) = event { 
            canvas.clear(pixels.get_frame());
           
            ants.retain(
                |&ant| 
                ant.coordinate_get().0 < (WIDTH-1) as i16 
                &&
                ant.coordinate_get().1 < (HEIGHT-1) as i16
            );
        
            canvas.draw_ants(&ants, pixels.get_frame());

            for ant in &mut ants {
                ant.update(&pheromones_mt);
            }

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            window.request_redraw();
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
            if input.key_pressed(VirtualKeyCode::R) {
                let mut ants: Vec<Ant> = Vec::new();
                for i in 0..100000 {
                    // randominess
                    use rand::distributions::{Distribution, Uniform};
                    let between = Uniform::from(0..8);
                    let mut rng = rand::thread_rng();
                    let rnd: i16 = (between.sample(&mut rng)) as i16;
                    let mut sel = 0;   
                    ants.push(Ant::new(i, Coordinate::new((WIDTH/2) as i16, (HEIGHT/2) as i16), 30, 10, rnd));
                }
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

