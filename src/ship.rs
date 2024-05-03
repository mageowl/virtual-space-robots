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
    get_random_value,
    math::{Rectangle, Vector2},
    RaylibHandle,
};

use self::api::APIRequest;
use crate::{assets::Assets, bullet::BulletPool, object::Object};

mod api;

enum Action {
    Waiting,
    Moving(f32),
    Turning(f32),
    Shooting(f32),
}

impl Action {
    fn from_req(req: APIRequest) -> Self {
        match req {
            APIRequest::Move(dist) => Action::Moving(dist),
            APIRequest::Turn(dist) => Action::Turning(dist),
            APIRequest::Shoot => Action::Shooting(Ship::SHOOT_COOLDOWN),
        }
    }
}

pub struct Ship {
    pos: Vector2,
    rotation: f32,
    thread: JoinHandle<()>,
    rx: Receiver<APIRequest>,
    state: Action,
    bullet_pool: BulletPool,
}

impl Ship {
    const MOVE_SPEED: f32 = 100.0;
    const TURN_SPEED: f32 = 70.0;
    const SHOOT_OFFSET: f32 = 20.0;
    const SHOOT_COOLDOWN: f32 = 1.0;

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
            pos: (
                get_random_value::<i64>(80, 1200) as f32,
                get_random_value::<i64>(80, 880) as f32,
            )
                .into(),
            rotation: 0.0,
            thread,
            rx,
            state: Action::Waiting,
            bullet_pool: BulletPool::new(10),
        }
    }

    fn next(&mut self) {
        self.state = Action::Waiting;
        self.thread.thread().unpark()
    }
}

impl Object for Ship {
    fn update(&mut self, rl: &RaylibHandle) {
        match &self.state {
            Action::Waiting => {
                let received = self.rx.try_recv();
                if let Ok(msg) = received {
                    if let APIRequest::Shoot = &msg {
                        self.bullet_pool
                            .shoot(
                                self.pos
                                    + Vector2::new(
                                        self.rotation.to_radians().sin(),
                                        self.rotation.to_radians().cos(),
                                    ) * Self::SHOOT_OFFSET,
                                self.rotation,
                            )
                            .unwrap();
                    }
                    self.state = Action::from_req(msg);
                }
            }
            Action::Moving(dist) => {
                let dist_moved = dist.abs().min(Self::MOVE_SPEED * rl.get_frame_time());
                self.pos += Vector2::new(
                    self.rotation.to_radians().sin(),
                    self.rotation.to_radians().cos(),
                ) * dist_moved
                    * dist.signum();
                if dist_moved < dist.abs() {
                    self.state = Action::Moving(dist - dist_moved * dist.signum());
                } else {
                    self.next();
                }
            }
            Action::Turning(dist) => {
                let dist_moved = dist.abs().min(Self::TURN_SPEED * rl.get_frame_time());
                self.rotation += dist_moved * dist.signum();
                if dist_moved < dist.abs() {
                    self.state = Action::Turning(dist - dist_moved * dist.signum());
                } else {
                    self.next();
                }
            }
            Action::Shooting(cooldown) => {
                if rl.get_frame_time() < *cooldown {
                    self.state = Action::Shooting(cooldown - rl.get_frame_time())
                } else {
                    self.next();
                }
            }
        }

        self.bullet_pool.update(rl);
    }
    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        d.draw_texture_pro(
            &assets.ship,
            Rectangle::new(0.0, 0.0, 50.0, 50.0),
            Rectangle::new(self.pos.x, self.pos.y, 50.0, 50.0),
            Vector2::new(25.0, 25.0),
            self.rotation,
            Color::WHITE,
        );

        self.bullet_pool.draw(d, assets);
    }
}