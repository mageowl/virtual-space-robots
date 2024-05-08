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

    let mut positions: Vec<(f32, f32)> = Vec::new();
    let mut rocks: Vec<Rock> = (0..get_random_value(13, 16))
        .map(|_| {
            let mut pos = (
                get_random_value::<i64>(80, 1200) as f32,
                get_random_value::<i64>(80, 880) as f32,
            );
            while positions
                .iter()
                .any(|(x, y)| (pos.0 - x).abs() + (pos.1 - y).abs() < 200.0)
            {
                pos = (
                    get_random_value::<i64>(80, 1200) as f32,
                    get_random_value::<i64>(80, 880) as f32,
                );
            }

            positions.push(pos);
            Rock::new(pos.0, pos.1)
        })
        .collect();

    let mut ships: Vec<Ship> = args
        .skip(1)
        .map(|p| {
            let mut pos = (
                get_random_value::<i64>(80, 1200) as f32,
                get_random_value::<i64>(80, 880) as f32,
            );
            while positions
                .iter()
                .any(|(x, y)| (pos.0 - x).abs() + (pos.1 - y).abs() < 200.0)
            {
                pos = (
                    get_random_value::<i64>(80, 1200) as f32,
                    get_random_value::<i64>(80, 880) as f32,
                );
            }

            positions.push(pos);
            Ship::new(p, bullet_pool.clone(), pos.0, pos.1)
        })
        .collect();

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
        rocks.draw(&mut d, &assets);
        ships.draw(&mut d, &assets);
        bullet_pool.borrow().draw(&mut d, &assets);
    }
}
