use std::env;

use bean_script::error::{BeanResult, ErrorSource};
use raylib::prelude::*;
use ship::Ship;

mod assets;
mod ship;

fn main() {
    set_trace_log(TraceLogLevel::LOG_ERROR);

    let (mut rl, thread) = raylib::init().size(1280, 960).title("Hello, World").build();
    let assets = assets::load(&mut rl, &thread);

    let path = env::args().nth(1).expect("Expected path to bean file");
    let mut ship = match Ship::new(path.clone()) {
        Err(error) => {
            println!(
                "\x1b[31;1merror\x1b[0m: {}",
                error.trace(ErrorSource::File(path))
            );
            return;
        }
        Ok(ship) => ship,
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        ship.draw(&mut d, &assets);
    }
}
