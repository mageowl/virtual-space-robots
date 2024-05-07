use raylib::{
    color::Color,
    drawing::RaylibDraw,
    get_random_value,
    math::{Rectangle, Vector2},
};

use crate::object::Object;

pub struct Rock {
    pos: Vector2,
    hp: u8,
}

impl Rock {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            hp: 2,
        }
    }
}

impl Object for Rock {
    fn update(
        &mut self,
        _rl: &raylib::prelude::RaylibHandle,
        collision_frame: &crate::collision::CollisionFrame,
    ) {
        if collision_frame.check_collision(vec!["bullet"], self.get_shape()) && self.hp > 0 {
            self.hp -= 1;
        }
    }

    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle, assets: &crate::assets::Assets) {
        if self.hp > 0 {
            d.draw_texture_pro(
                &assets.rock,
                Rectangle::new(0.0, 0.0, 100.0, 100.0),
                Rectangle::new(self.pos.x, self.pos.y, 100.0, 100.0),
                Vector2::new(50.0, 50.0),
                0.0,
                Color::WHITE,
            );
        }
    }

    fn get_shape(&self) -> crate::collision::Circle {
        if self.hp > 0 {
            (self.pos, 45.0)
        } else {
            (Vector2::zero(), 0.0)
        }
    }
}
