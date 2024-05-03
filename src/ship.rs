use std::{
    fs,
    path::PathBuf,
    rc::Rc,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

use bean_script::{
    error::{BeanResult, ErrorSource},
    modules::{
        registry::{ModuleRegistry, RegistryFeatures},
        BuiltinModule, CustomModule,
    },
    util::make_ref,
};
use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    RaylibHandle,
};

use self::api::APIRequest;
use crate::assets::Assets;

mod api;

enum State {
    Waiting,
    Moving(f32),
    Turning(f32),
    Shooting(f32),
}

impl State {
    fn from_req(req: APIRequest) -> Self {
        match req {
            APIRequest::Move(dist) => State::Moving(dist),
            APIRequest::Turn(dist) => State::Turning(dist),
            APIRequest::Shoot => State::Shooting(3.0),
        }
    }
}

pub struct Ship {
    pos: Vector2,
    rotation: f32,
    thread: JoinHandle<()>,
    rx: Receiver<APIRequest>,
    state: State,
}

impl Ship {
    const MOVE_SPEED: f32 = 100.0;
    const TURN_SPEED: f32 = 70.0;

    pub fn new(path: String) -> Self {
        let (tx, rx) = mpsc::channel();

        let thread = thread::spawn(move || {
            let file = fs::read_to_string(path.clone()).expect("Failed to open file");

            let mut dir_path = PathBuf::from(path.clone());
            dir_path.pop();

            let mut registry = ModuleRegistry::new(RegistryFeatures::default());
            registry
                .metadata
                .insert(String::from("sender"), Box::new(tx));
            registry.register_initialized_builtin(
                String::from("robot_api"),
                BuiltinModule::new(api::construct, registry.features),
            );

            let scope = make_ref(CustomModule::new(make_ref(registry), dir_path));
            let result = bean_script::interpret(file, Rc::clone(&scope));

            if let Err(error) = result {
                println!(
                    "\x1b[31;1merror\x1b[0m: {}",
                    error.trace(ErrorSource::File(path))
                );
            }
        });

        Self {
            pos: (640.0, 480.0).into(),
            rotation: 0.0,
            thread,
            rx,
            state: State::Waiting,
        }
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

    pub fn update(&mut self, rl: &RaylibHandle) {
        match &self.state {
            State::Waiting => {
                let received = self.rx.try_recv();
                if let Ok(msg) = received {
                    self.state = State::from_req(msg);
                }
            }
            State::Moving(dist) => {
                let dist_moved = dist.abs().min(Self::MOVE_SPEED * rl.get_frame_time());
                self.pos += <(f32, f32) as Into<Vector2>>::into((
                    self.rotation.to_radians().sin(),
                    self.rotation.to_radians().cos(),
                )) * dist_moved
                    * dist.signum();
                if dist_moved < dist.abs() {
                    self.state = State::Moving(dist - dist_moved * dist.signum());
                } else {
                    self.next();
                }
            }
            State::Turning(dist) => {
                let dist_moved = dist.abs().min(Self::TURN_SPEED * rl.get_frame_time());
                self.rotation += dist_moved * dist.signum();
                if dist_moved < dist.abs() {
                    self.state = State::Turning(dist - dist_moved * dist.signum());
                } else {
                    self.next();
                }
            }
            State::Shooting(cooldown) => {
                if rl.get_frame_time() < *cooldown {
                    self.state = State::Shooting(cooldown - rl.get_frame_time())
                } else {
                    self.next();
                }
            }
        }
    }

    fn next(&mut self) {
        self.state = State::Waiting;
        self.thread.thread().unpark()
    }
}
