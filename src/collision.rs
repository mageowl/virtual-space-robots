use std::collections::HashMap;

use raylib::{
    collision,
    math::{Rectangle, Vector2},
};

use crate::object::Object;

pub type Circle = (Vector2, f32);

pub fn check_collision_circles(circle1: Circle, circle2: Circle) -> bool {
    if circle1.1 == 0.0 || circle2.1 == 0.0 || circle1 == circle2 {
        false
    } else {
        collision::check_collision_circles(circle1.0, circle1.1, circle2.0, circle2.1)
    }
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
    const MAX_RAY_LENGTH: f32 = 1000.0;

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

    pub fn raycast(
        &self,
        layers: Vec<&str>,
        pos: Vector2,
        rotation: f32,
        radius: f32,
    ) -> (String, f32) {
        self.raycast_step(layers, pos, rotation, radius, 0.0)
    }

    fn raycast_step(
        &self,
        layers: Vec<&str>,
        pos: Vector2,
        rotation: f32,
        radius: f32,
        dist: f32,
    ) -> (String, f32) {
        let mut mut_layers = layers.clone();
        while !mut_layers.is_empty() {
            let name = mut_layers.pop().unwrap();
            let Some(layer) = self.layers.get(name) else {
                continue;
            };
            if layer.check_collision((pos, radius)) {
                return (String::from(name), dist + radius);
            }
        }

        if dist > CollisionFrame::MAX_RAY_LENGTH {
            (String::from("none"), dist)
        } else if !Rectangle::new(0.0, 0.0, 1280.0, 960.0).check_collision_point_rec(pos) {
            (String::from("wall"), dist)
        } else {
            self.raycast_step(
                layers,
                pos + Vector2::new(rotation.to_radians().cos(), rotation.to_radians().sin())
                    * radius,
                rotation,
                radius,
                dist + radius,
            )
        }
    }
}
