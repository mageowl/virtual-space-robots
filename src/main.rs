use std::env;

use bean_script::util::{make_ref, MutRc};
use bullet::BulletPool;
use collision::{CollisionFrame, CollisionLayer};
use itertools::Itertools;
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
    let mut ships: Vec<Ship> = make_ships(&mut positions, &bullet_pool);
    let mut rocks: Vec<Rock> = make_rocks(&mut positions);

    rocks.append(
        &mut ships
            .iter()
            .combinations(2)
            .flat_map(|pair| {
                let [s1, s2] = &pair[..] else {
                    return Vec::new();
                };
                let pos = s1.get_pos() + (s2.get_pos() - s1.get_pos()) * 0.5;

                if positions
                    .iter()
                    .any(|(x, y)| (pos.x - x).abs() + (pos.y - y).abs() < 150.0)
                {
                    Vec::new()
                } else {
                    vec![Rock::new(pos.x, pos.y)]
                }
            })
            .collect(),
    );

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
            ship_did_win = true;
        }

        // DRAW //
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        rocks.draw(&mut d, &assets);
        ships.draw(&mut d, &assets);
        bullet_pool.borrow().draw(&mut d, &assets);

        if ship_did_win {
            d.draw_text(
                &(ships
                    .iter()
                    .filter(|s| !s.is_destroyed())
                    .next()
                    .map(|s| s.name.clone())
                    .unwrap_or(String::new())
                    + " won."),
                10,
                936,
                24,
                Color::GREEN,
            );
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
