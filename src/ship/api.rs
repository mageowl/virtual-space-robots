use std::{
    cell::RefCell,
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
};

use bean_script::{
    arg_check, as_type,
    data::Data,
    error::{BeanResult, Error, ErrorSource},
    modules::{registry::ModuleRegistry, CustomModule, ModuleBuilder},
    scope::{function::Function, ScopeRef},
};

use super::ShipHandle;

pub enum APIRequest {
    Move(f32),
    Turn(f32),
    Shoot,
}

fn get_sender(registry: &ModuleRegistry) -> Result<&Sender<APIRequest>, Error> {
    registry
        .metadata
        .get("sender")
        .ok_or(Error::new(
            "Couldn't access API message sender.",
            ErrorSource::Internal,
        ))?
        .downcast_ref::<Sender<APIRequest>>()
        .ok_or(Error::new(
            "API message sender was incorrect type.",
            ErrorSource::Internal,
        ))
}

fn get_mutex(registry: &ModuleRegistry) -> Result<Arc<Mutex<ShipHandle>>, Error> {
    registry
        .metadata
        .get("mutex")
        .ok_or(Error::new(
            "Couldn't access API mutex.",
            ErrorSource::Internal,
        ))?
        .downcast_ref::<Arc<Mutex<ShipHandle>>>()
        .ok_or(Error::new(
            "API mutex was incorrect type.",
            ErrorSource::Internal,
        ))
        .map(Arc::clone)
}

pub fn construct(module: &mut ModuleBuilder) {
    module
        .function("move", fn_move)
        .function("turn", fn_turn)
        .function("shoot", fn_shoot)
        .function("raycast", fn_raycast)
        .function("raycast_dist", fn_raycast_dist)
        .function("x", fn_x)
        .function("y", fn_y);
}

fn fn_move(args: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    arg_check!(args[0] => Data::Number(d), "Expected a number, but instead got a {}.", "robot_api:move");
    let binding = RefCell::borrow(&scope).get_file_module().ok_or(Error::new(
        "Cannot connect to api outside of module.",
        ErrorSource::Builtin(String::from("robot_api:move")),
    ))?;
    let borrowed = RefCell::borrow(&binding);
    let registry = RefCell::borrow(
        &as_type!(borrowed => CustomModule, "Returned non-CustomModule from get_file_module")
            .registry,
    );
    let sender =
        get_sender(&registry).trace(ErrorSource::Builtin(String::from("robot_api:move")))?;
    sender.send(APIRequest::Move(d as f32)).map_err(|_| {
        Error::new(
            "Failed to send API request.",
            ErrorSource::Builtin(String::from("robot_api:move")),
        )
    })?;
    thread::park();

    Ok(Data::None)
}

fn fn_turn(args: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    arg_check!(args[0] => Data::Number(d), "Expected a number, but instead got a {}.", "robot_api:turn");
    let binding = RefCell::borrow(&scope).get_file_module().ok_or(Error::new(
        "Cannot connect to api outside of module.",
        ErrorSource::Builtin(String::from("robot_api:turn")),
    ))?;
    let borrowed = RefCell::borrow(&binding);
    let registry = RefCell::borrow(
        &as_type!(borrowed => CustomModule, "Returned non-CustomModule from get_file_module")
            .registry,
    );
    let sender =
        get_sender(&registry).trace(ErrorSource::Builtin(String::from("robot_api:turn")))?;
    sender.send(APIRequest::Turn(d as f32)).map_err(|_| {
        Error::new(
            "Failed to send API request.",
            ErrorSource::Builtin(String::from("robot_api:turn")),
        )
    })?;
    thread::park();

    Ok(Data::None)
}

fn fn_shoot(_a: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    let binding = RefCell::borrow(&scope).get_file_module().ok_or(Error::new(
        "Cannot connect to api outside of module.",
        ErrorSource::Builtin(String::from("robot_api:shoot")),
    ))?;
    let borrowed = RefCell::borrow(&binding);
    let registry = RefCell::borrow(
        &as_type!(borrowed => CustomModule, "Returned non-CustomModule from get_file_module")
            .registry,
    );
    let sender =
        get_sender(&registry).trace(ErrorSource::Builtin(String::from("robot_api:shoot")))?;
    sender.send(APIRequest::Shoot).map_err(|_| {
        Error::new(
            "Failed to send API request.",
            ErrorSource::Builtin(String::from("robot_api:shoot")),
        )
    })?;
    thread::park();

    Ok(Data::None)
}

fn fn_raycast(_a: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    let binding = RefCell::borrow(&scope).get_file_module().ok_or(Error::new(
        "Cannot connect to api outside of module.",
        ErrorSource::Builtin(String::from("robot_api:raycast_dist")),
    ))?;
    let borrowed = RefCell::borrow(&binding);
    let registry = RefCell::borrow(
        &as_type!(borrowed => CustomModule, "Returned non-CustomModule from get_file_module")
            .registry,
    );

    let mutex =
        get_mutex(&registry).trace(ErrorSource::Builtin(String::from("robot_api:raycast")))?;
    let mutex_lock = mutex.lock().unwrap();

    Ok(Data::String(mutex_lock.raycast.clone()))
}

fn fn_raycast_dist(_a: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    let binding = RefCell::borrow(&scope).get_file_module().ok_or(Error::new(
        "Cannot connect to api outside of module.",
        ErrorSource::Builtin(String::from("robot_api:raycast")),
    ))?;
    let borrowed = RefCell::borrow(&binding);
    let registry = RefCell::borrow(
        &as_type!(borrowed => CustomModule, "Returned non-CustomModule from get_file_module")
            .registry,
    );

    let raycast =
        get_mutex(&registry).trace(ErrorSource::Builtin(String::from("robot_api:raycast")))?;
    let raycast_guard = raycast.lock().unwrap();

    Ok(Data::Number(raycast_guard.raycast_dist as f64))
}

fn fn_x(_a: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    let binding = RefCell::borrow(&scope).get_file_module().ok_or(Error::new(
        "Cannot connect to api outside of module.",
        ErrorSource::Builtin(String::from("robot_api:raycast")),
    ))?;
    let borrowed = RefCell::borrow(&binding);
    let registry = RefCell::borrow(
        &as_type!(borrowed => CustomModule, "Returned non-CustomModule from get_file_module")
            .registry,
    );

    let raycast =
        get_mutex(&registry).trace(ErrorSource::Builtin(String::from("robot_api:raycast")))?;
    let raycast_guard = raycast.lock().unwrap();

    Ok(Data::Number(raycast_guard.pos.x as f64))
}

fn fn_y(_a: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    let binding = RefCell::borrow(&scope).get_file_module().ok_or(Error::new(
        "Cannot connect to api outside of module.",
        ErrorSource::Builtin(String::from("robot_api:raycast")),
    ))?;
    let borrowed = RefCell::borrow(&binding);
    let registry = RefCell::borrow(
        &as_type!(borrowed => CustomModule, "Returned non-CustomModule from get_file_module")
            .registry,
    );

    let raycast =
        get_mutex(&registry).trace(ErrorSource::Builtin(String::from("robot_api:raycast")))?;
    let raycast_guard = raycast.lock().unwrap();

    Ok(Data::Number(raycast_guard.pos.y as f64))
}
