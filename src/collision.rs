use std::collections::HashMap;

use raylib::{collision, math::Vector2};

use crate::object::Object;

pub type Circle = (Vector2, f32);

pub fn check_collision_circles(circle1: Circle, circle2: Circle) -> bool {
    collision::check_collision_circles(circle1.0, circle1.1, circle2.0, circle2.1)
}

pub struct CollisionLayer {
    shapes: Vec<Circle>,
}

impl CollisionLayer {
    pub fn from(collection: &[impl Object]) -> Self {
        let mut s = CollisionLayer { shapes: Vec::new() };
        for obj in collection {
            s.shapes.push(obj.get_shape());
        }
        s
    }

    pub fn check_collision(&self, circle: Circle) -> bool {
        for shape in &self.shapes {
            if check_collision_circles(*shape, circle) {
                return true;
            }
        }
        false
    }
}

pub struct CollisionFrame {
    layers: HashMap<&'static str, CollisionLayer>,
}

impl CollisionFrame {
    pub fn new(layers: Vec<(&'static str, CollisionLayer)>) -> Self {
        CollisionFrame {
            layers: layers.into_iter().collect(),
        }
    }

    pub fn check_collision(&self, mut layers: Vec<&str>, circle: Circle) -> bool {
        while !layers.is_empty() {
            let Some(layer) = self.layers.get(layers.pop().unwrap()) else {
                continue;
            };
            if layer.check_collision(circle) {
                return true;
            }
        }
        false
    }
}
