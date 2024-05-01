use std::{fs, path::PathBuf, rc::Rc};

use bean_script::{
    error::Error,
    modules::{
        registry::{ModuleRegistry, RegistryFeatures},
        BuiltinModule, CustomModule,
    },
    util::{make_ref, MutRc},
};
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
};

use crate::assets::Assets;

mod api;

pub struct Ship {
    pos: Vector2,
    rotation: f32,
    scope: MutRc<CustomModule>,
}

impl Ship {
    pub fn new(path: String) -> Result<Self, Error> {
        let file = fs::read_to_string(path.clone()).expect("Failed to open file");

        let mut dir_path = PathBuf::from(path.clone());
        dir_path.pop();

        let mut registry = ModuleRegistry::new(RegistryFeatures::default());
        registry.register_initialized_builtin(
            String::from("robot_api"),
            BuiltinModule::new(api::construct, registry.features),
        );

        let scope = make_ref(CustomModule::new(make_ref(registry), dir_path));

        let result = bean_script::interpret(file, Rc::clone(&scope));

        result.map(|_| Self {
            pos: Vector2::zero(),
            rotation: 0.0,
            scope,
        })
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        d.draw_texture_pro(
            &assets.ship,
            Rectangle::new(0.0, 0.0, 50.0, 50.0),
            Rectangle::new(self.pos.x, self.pos.y, 50.0, 50.0),
            Vector2::new(25.0, 25.0),
            self.rotation,
            Color::WHITE,
        );
    }
}
