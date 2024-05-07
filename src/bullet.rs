use std::collections::VecDeque;

use raylib::{
    color::Color,
    drawing::RaylibDraw,
    math::{Rectangle, Vector2},
    prelude::{RaylibDrawHandle, RaylibHandle},
};

use crate::{
    assets::Assets,
    collision::{Circle, CollisionFrame, CollisionLayer},
    object::Object,
};

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
    fn update(&mut self, rl: &RaylibHandle, collision_frame: &CollisionFrame) {
        self.lifetime -= rl.get_frame_time();

        if collision_frame.check_collision(vec!["ship", "rock"], self.get_shape()) {
            self.sleep_queued = true
        }

        self.pos += Vector2::new(
            self.rotation.to_radians().cos(),
            self.rotation.to_radians().sin(),
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
            self.rotation + 90.0,
            Color::RED,
        );
    }

    fn get_shape(&self) -> Circle {
        (self.pos, 10.0)
    }
}

pub struct BulletPool {
    asleep: Vec<Bullet>,
    pub awake: VecDeque<Bullet>,
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

    pub fn collision_layer(&mut self) -> CollisionLayer {
        CollisionLayer::from(self.awake.make_contiguous())
    }
}

impl Object for BulletPool {
    fn update(&mut self, rl: &RaylibHandle, collision_frame: &CollisionFrame) {
        let mut sleep = Vec::new();

        for (i, obj) in self.awake.iter_mut().enumerate() {
            obj.update(rl, collision_frame);
            if obj.sleep_queued {
                obj.sleep_queued = false;
                obj.lifetime = Bullet::LIFETIME;
                sleep.push(i);
            }
        }

        sleep.sort_unstable();
        sleep.reverse();
        for i in sleep {
            self.asleep.push(self.awake.remove(i).unwrap());
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        self.awake.draw(d, assets)
    }

    fn is_colliding(&self, other: &dyn Object) -> bool {
        self.awake.is_colliding(other)
    }
    fn get_shape(&self) -> Circle {
        (Vector2::zero(), 0.0)
    }
}
