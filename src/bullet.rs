use std::collections::VecDeque;

use raylib::{
    color::Color,
    drawing::RaylibDraw,
    math::{Rectangle, Vector2},
    prelude::{RaylibDrawHandle, RaylibHandle},
};

use crate::{assets::Assets, object::Object};

pub struct Bullet {
    pos: Vector2,
    rotation: f32,
    lifetime: f32,
    sleep_queued: bool,
}

impl Bullet {
    const SPEED: f32 = 400.0;
    const LIFETIME: f32 = 3.0;

    fn new() -> Self {
        Self {
            pos: Vector2::zero(),
            rotation: 0.0,
            lifetime: Self::LIFETIME,
            sleep_queued: false,
        }
    }
}

impl Object for Bullet {
    fn update(&mut self, rl: &RaylibHandle) {
        self.lifetime -= rl.get_frame_time();

        self.pos += Vector2::new(
            self.rotation.to_radians().sin(),
            self.rotation.to_radians().cos(),
        ) * Self::SPEED
            * rl.get_frame_time();

        if self.lifetime <= 0.0 {
            self.sleep_queued = true
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        d.draw_texture_pro(
            &assets.bullet,
            Rectangle::new(0.0, 0.0, 50.0, 50.0),
            Rectangle::new(self.pos.x, self.pos.y, 50.0, 50.0),
            Vector2::new(25.0, 25.0),
            self.rotation,
            Color::RED,
        );
    }
}

pub struct BulletPool {
    asleep: Vec<Bullet>,
    awake: VecDeque<Bullet>,
}

impl BulletPool {
    pub fn new(count: usize) -> Self {
        let mut asleep = Vec::new();
        for _ in 0..count {
            asleep.push(Bullet::new());
        }

        Self {
            asleep,
            awake: VecDeque::new(),
        }
    }

    pub fn shoot(&mut self, pos: Vector2, rotation: f32) -> Result<(), String> {
        let mut bullet = self
            .asleep
            .pop()
            .ok_or(String::from("Ran out of bullets."))?;
        bullet.pos = pos;
        bullet.rotation = rotation;
        self.awake.push_front(bullet);

        Ok(())
    }
}

impl Object for BulletPool {
    fn update(&mut self, rl: &RaylibHandle) {
        let mut sleep = Vec::new();

        for (i, obj) in &mut self.awake.iter_mut().enumerate() {
            obj.update(rl);
            if obj.sleep_queued {
                obj.sleep_queued = false;
                sleep.push(i);
            }
        }
        for i in sleep {
            self.asleep.push(self.awake.remove(i).unwrap());
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        self.awake.draw(d, assets)
    }
}
