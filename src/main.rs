mod canvas;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const BG_COLOR: Color = Color::RGB(200,220,140);
const WINDOW_WIDTH: u32 = 1400;
const WINDOW_HEIGHT: u32 = 700;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = canvas::make_canvas(&sdl_context, WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        
        canvas.set_draw_color(BG_COLOR);
        canvas.clear();
        canvas.present();
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
