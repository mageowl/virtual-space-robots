use std::collections::VecDeque;

use raylib::{collision, drawing::RaylibDrawHandle, math::Vector2, RaylibHandle};

use crate::{
    assets::Assets,
    collision::{Circle, CollisionFrame},
};

pub trait Object {
    fn update(&mut self, rl: &RaylibHandle, collision_frame: &CollisionFrame);
    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets);
    fn get_shape(&self) -> Circle;

    fn is_colliding(&self, other: &dyn Object) -> bool {
        let shape1 = self.get_shape();
        let shape2 = other.get_shape();
        collision::check_collision_circles(shape1.0, shape1.1, shape2.0, shape2.1)
    }
}

impl<T: Object> Object for Vec<T> {
    fn update(&mut self, rl: &RaylibHandle, collision_frame: &CollisionFrame) {
        for obj in self {
            obj.update(rl, collision_frame);
        }
    }
    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        for obj in self {
            obj.draw(d, assets);
        }
    }
    fn is_colliding(&self, other: &dyn Object) -> bool {
        self.iter().any(|obj| obj.is_colliding(other))
    }

    fn get_shape(&self) -> Circle {
        (Vector2::zero(), 0.0)
    }
}

impl<T: Object> Object for VecDeque<T> {
    fn update(&mut self, rl: &RaylibHandle, collision_frame: &CollisionFrame) {
        for obj in self {
            obj.update(rl, collision_frame);
        }
    }
    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        for obj in self {
            obj.draw(d, assets);
        }
    }
    fn is_colliding(&self, other: &dyn Object) -> bool {
        self.iter().any(|obj| obj.is_colliding(other))
    }

    fn get_shape(&self) -> Circle {
        (Vector2::zero(), 0.0)
    }
}
