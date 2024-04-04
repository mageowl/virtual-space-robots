use std::f64::consts::PI;
use std::path::PathBuf;

use bean_script::error::{BeanResult, ErrorSource};
use bean_script::modules::registry::ModuleRegistry;
use bean_script::modules::CustomModule;
use bean_script::util::make_ref;
use bean_script::{evaluator, lexer, parser};
use godot::engine::utilities::move_toward;
use godot::engine::{INode, Node};
use godot::prelude::*;

const UPDATE_DELAY: f64 = 0.05;

#[derive(GodotClass)]
#[class(base=Node)]
struct ShipController {
    scope: Option<CustomModule>,
    update_timer: f64,

    base: Base<Node>,
}

#[godot_api]
impl INode for ShipController {
    fn init(base: Base<Node>) -> Self {
        Self {
            scope: None,
            update_timer: 0.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if let Some(scope) = &self.scope {
            self.update_timer = move_toward(self.update_timer, 0.0, delta);
            if self.update_timer == 0.0 {
                self.update_timer = UPDATE_DELAY;
                godot_print!("update smth");
            }
        }
    }
}

#[godot_api]
impl ShipController {
    #[func]
    fn set_code(&mut self, file: String, path: String) {
        let tokens = lexer::tokenize(file);

        let tree = parser::parse(tokens);
        if let Err(error) = tree {
            println!(
                "\x1b[31;1merror\x1b[0m: {}",
                error.trace(ErrorSource::File(path.clone()))
            );
            return;
        }
        let tree = tree.unwrap();

        let mut dir_path = PathBuf::from(path.clone());
        dir_path.pop();

        let registry = make_ref(ModuleRegistry::new());
        let program_scope = CustomModule::new(registry, dir_path);
        let result = evaluator::evaluate(&tree, make_ref(program_scope));
        if let Err(error) = result {
            println!(
                "\x1b[31;1merror\x1b[0m: {}",
                error.trace(ErrorSource::File(path.clone()))
            );
        }
    }
}
