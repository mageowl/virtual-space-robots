use std::env;

use object::Object;
use raylib::prelude::*;
use ship::Ship;

mod assets;
mod bullet;
mod object;
mod ship;

fn main() {
    set_trace_log(TraceLogLevel::LOG_ERROR);

    let (mut rl, thread) = raylib::init().size(1280, 960).title("Hello, World").build();

    let assets = assets::load(&mut rl, &thread);
    let mut ships: Vec<Ship> = env::args().skip(1).map(|p| Ship::new(p)).collect();

    while !rl.window_should_close() {
        // UPDATE //
        ships.update(&rl);

        // DRAW  //
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        ships.draw(&mut d, &assets);
    }
}
