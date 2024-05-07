use std::{
    fs,
    path::PathBuf,
    rc::Rc,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use bean_script::{
    error::{BeanResult, ErrorSource},
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
    RaylibHandle,
};

use self::api::APIRequest;
use crate::{
    assets::Assets,
    bullet::BulletPool,
    collision::{Circle, CollisionFrame},
    object::Object,
};

mod api;
enum State {
    Destroyed,
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
            APIRequest::Shoot => State::Shooting(Ship::SHOOT_COOLDOWN),
        }
    }
}

pub struct ShipHandle {
    raycast: String,
    raycast_dist: f32,
    pos: Vector2,
}

pub struct Ship {
    name: String,
    pos: Vector2,
    rotation: f32,
    thread: JoinHandle<()>,
    rx: Receiver<APIRequest>,
    handle: Arc<Mutex<ShipHandle>>,
    state: State,
    bullet_pool: MutRc<BulletPool>,
}

impl Ship {
    const MOVE_SPEED: f32 = 100.0;
    const TURN_SPEED: f32 = 360.0;
    const SHOOT_OFFSET: f32 = 40.1;
    const SHOOT_COOLDOWN: f32 = 1.0;

    pub fn new(path: String, bullet_pool: MutRc<BulletPool>, x: f32, y: f32) -> Self {
        let (tx, rx) = mpsc::channel();
        let handle = Arc::new(Mutex::new(ShipHandle {
            raycast: String::from("none"),
            raycast_dist: -1.0,
            pos: Vector2::new(x, y),
        }));
        let handle_read = Arc::clone(&handle);
        let name = PathBuf::from(path.clone())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let thread = thread::spawn(move || {
            let file = fs::read_to_string(path.clone()).expect("Failed to open file");

            let mut dir_path = PathBuf::from(path.clone());
            dir_path.pop();

            let mut registry = ModuleRegistry::new(RegistryFeatures::default());
            registry
                .metadata
                .insert(String::from("sender"), Box::new(tx));
            registry
                .metadata
                .insert(String::from("mutex"), Box::new(handle_read));
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
            name,
            pos: Vector2::new(x, y),
            rotation: 0.0,
            thread,
            rx,
            handle,
            state: State::Waiting,
            bullet_pool,
        }
    }

    fn next(&mut self) {
        self.state = State::Waiting;
        self.thread.thread().unpark()
    }

    fn make_handle(&self, collision_frame: &CollisionFrame) -> ShipHandle {
        let (raycast, raycast_dist) = collision_frame.raycast(
            vec!["ship", "rock"],
            self.pos
                + Vector2::new(
                    self.rotation.to_radians().cos(),
                    self.rotation.to_radians().sin(),
                ) * Self::SHOOT_OFFSET,
            self.rotation,
            20.0,
        );

        ShipHandle {
            raycast,
            raycast_dist,
            pos: self.pos,
        }
    }
}

impl Object for Ship {
    fn update(&mut self, rl: &RaylibHandle, collision_frame: &CollisionFrame) {
        let mut should_unpark = false;
        match &self.state {
            State::Waiting => {
                let received = self.rx.try_recv();
                if let Ok(msg) = received {
                    if let APIRequest::Shoot = &msg {
                        self.bullet_pool
                            .borrow_mut()
                            .shoot(
                                self.pos
                                    + Vector2::new(
                                        self.rotation.to_radians().cos(),
                                        self.rotation.to_radians().sin(),
                                    ) * Self::SHOOT_OFFSET,
                                self.rotation,
                            )
                            .unwrap();
                    }
                    self.state = State::from_req(msg);
                }
            }
            State::Moving(dist) => {
                let dist_moved = dist.abs().min(Self::MOVE_SPEED * rl.get_frame_time());
                self.pos += Vector2::new(
                    self.rotation.to_radians().cos(),
                    self.rotation.to_radians().sin(),
                ) * dist_moved
                    * dist.signum();
                if dist_moved < dist.abs() {
                    self.state = State::Moving(dist - dist_moved * dist.signum());
                } else {
                    should_unpark = true;
                }
            }
            State::Turning(dist) => {
                let dist_moved = dist.abs().min(Self::TURN_SPEED * rl.get_frame_time());
                self.rotation += dist_moved * dist.signum();
                if dist_moved < dist.abs() {
                    self.state = State::Turning(dist - dist_moved * dist.signum());
                } else {
                    should_unpark = true;
                }
            }
            State::Shooting(cooldown) => {
                if rl.get_frame_time() < *cooldown {
                    self.state = State::Shooting(cooldown - rl.get_frame_time())
                } else {
                    should_unpark = true;
                }
            }
            State::Destroyed => return,
        }

        if collision_frame.check_collision(vec!["bullet", "rock"], self.get_shape()) {
            self.state = State::Destroyed;
        } else {
            let mut raycast_lock = self.handle.lock().unwrap();
            *raycast_lock = self.make_handle(collision_frame);
            drop(raycast_lock);

            if should_unpark {
                self.next();
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets) {
        if let State::Destroyed = self.state {
            d.draw_texture_pro(
                &assets.ship_dead,
                Rectangle::new(0.0, 0.0, 50.0, 50.0),
                Rectangle::new(self.pos.x, self.pos.y, 50.0, 50.0),
                Vector2::new(25.0, 25.0),
                self.rotation + 90.0,
                Color::WHITE,
            );
        } else {
            d.draw_texture_pro(
                &assets.ship,
                Rectangle::new(0.0, 0.0, 50.0, 50.0),
                Rectangle::new(self.pos.x, self.pos.y, 50.0, 50.0),
                Vector2::new(25.0, 25.0),
                self.rotation + 90.0,
                Color::WHITE,
            );
        }
        d.draw_text(
            &self.name,
            self.pos.x as i32 - 50,
            self.pos.y as i32 - 50,
            18,
            Color::GREEN,
        )
    }

    fn get_shape(&self) -> Circle {
        if let State::Destroyed = &self.state {
            (Vector2::zero(), 0.0)
        } else {
            (self.pos, 20.0)
        }
    }
}
