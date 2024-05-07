use std::env;

use bean_script::util::make_ref;
use bullet::BulletPool;
use collision::{CollisionFrame, CollisionLayer};
use object::Object;
use raylib::prelude::*;
use rock::Rock;
use ship::Ship;

mod assets;
mod bullet;
mod collision;
mod object;
mod rock;
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
    let mut rocks: Vec<Rock> = (0..get_random_value(3, 4)).map(|_| Rock::new()).collect();

    while !rl.window_should_close() {
        // UPDATE //
        let collision_frame = CollisionFrame::new(vec![
            ("ship", CollisionLayer::from(&ships)),
            ("bullet", bullet_pool.borrow_mut().collision_layer()),
            ("rock", CollisionLayer::from(&rocks)),
        ]);

        ships.update(&rl, &collision_frame);
        rocks.update(&rl, &collision_frame);
        bullet_pool.borrow_mut().update(&rl, &collision_frame);

        // DRAW  //
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        ships.draw(&mut d, &assets);
        rocks.draw(&mut d, &assets);
        bullet_pool.borrow().draw(&mut d, &assets);
    }
}
