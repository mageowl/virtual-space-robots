use godot::prelude::*;

mod ship_controller;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
