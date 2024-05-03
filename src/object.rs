use std::collections::VecDeque;

use raylib::{drawing::RaylibDrawHandle, RaylibHandle};

use crate::assets::Assets;

pub trait Object: Sized {
    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets);
}

impl<T: Object> Object for Vec<T> {
    fn update(&mut self, rl: &RaylibHandle) {
        for obj in self {
            obj.update(rl);
        }
    }
    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        for obj in self {
            obj.draw(d, assets);
        }
    }
}

impl<T: Object> Object for VecDeque<T> {
    fn update(&mut self, rl: &RaylibHandle) {
        for obj in self {
            obj.update(rl);
        }
    }
    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        for obj in self {
            obj.draw(d, assets);
        }
    }
}
