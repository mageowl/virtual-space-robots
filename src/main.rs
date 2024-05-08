use std::env;

use bean_script::util::{make_ref, MutRc};
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

    let bullet_pool = make_ref(BulletPool::new(60));

    let mut positions: Vec<(f32, f32)> = Vec::new();
    let mut rocks: Vec<Rock> = make_rocks(&mut positions);
    let mut ships: Vec<Ship> = make_ships(&mut positions, &bullet_pool);
    let mut scores = vec![0usize; ships.len()];
    let mut ship_did_win = false;

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

        if ships.iter().filter(|s| !s.is_destroyed()).count() == 1 && !ship_did_win {
            for (i, _) in ships.iter().enumerate().filter(|s| !s.1.is_destroyed()) {
                scores[i] += 1;
            }
            ship_did_win = true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            positions = Vec::new();
            rocks = make_rocks(&mut positions);
            ships = make_ships(&mut positions, &bullet_pool);
            ship_did_win = false;
        }

        // DRAW  //
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        rocks.draw(&mut d, &assets);
        ships.draw(&mut d, &assets);
        bullet_pool.borrow().draw(&mut d, &assets);

        if ship_did_win {
            d.draw_text(
                &(ships[0].name.clone() + " won."),
                10,
                936,
                24,
                Color::GREEN,
            );

            let mut y = 10;
            for (i, ship) in ships.iter().enumerate() {
                d.draw_text(
                    &(ship.name.clone() + ": " + &scores[i].to_string()),
                    10,
                    y,
                    24,
                    Color::WHITE,
                );
                y += 34;
            }
        }
    }
}

fn make_rocks(positions: &mut Vec<(f32, f32)>) -> Vec<Rock> {
    (0..get_random_value(13, 16))
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
        .collect()
}

fn make_ships(positions: &mut Vec<(f32, f32)>, bullet_pool: &MutRc<BulletPool>) -> Vec<Ship> {
    env::args()
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
        .collect()
}
