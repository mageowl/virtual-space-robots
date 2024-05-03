use std::env;

use bean_script::util::make_ref;
use bullet::BulletPool;
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

    let args = env::args();
    let bullet_pool = make_ref(BulletPool::new(10 * args.len()));
    let mut ships: Vec<Ship> = args
        .skip(1)
        .map(|p| Ship::new(p, bullet_pool.clone()))
        .collect();

    while !rl.window_should_close() {
        // UPDATE //
        ships.update(&rl);
        bullet_pool.borrow_mut().update(&rl);
        bullet_pool.borrow_mut().collide(&ships);

        // DRAW  //
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        ships.draw(&mut d, &assets);
        bullet_pool.borrow().draw(&mut d, &assets);
    }
}
