use std::{cell::RefCell, sync::mpsc::Sender, thread};

use bean_script::{
    arg_check, as_type,
    data::Data,
    error::{BeanResult, Error, ErrorSource},
    modules::{registry::ModuleRegistry, CustomModule, ModuleBuilder},
    scope::{function::Function, ScopeRef},
};

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
            ErrorSource::Builtin(String::from("robot_api:[internal]")),
        ))?
        .downcast_ref::<Sender<APIRequest>>()
        .ok_or(Error::new(
            "API message sender was incorrect type.",
            ErrorSource::Builtin(String::from("robot_api:[internal]")),
        ))
}

pub fn construct(module: &mut ModuleBuilder) {
    module
        .function("move", fn_move)
        .function("turn", fn_turn)
        .function("shoot", fn_shoot)
        .function("raycast", fn_raycast);
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
    sender
        .send(APIRequest::Turn((d * 360.0) as f32))
        .map_err(|_| {
            Error::new(
                "Failed to send API request.",
                ErrorSource::Builtin(String::from("robot_api:move")),
            )
        })?;
    thread::park();

    Ok(Data::None)
}

fn fn_shoot(_a: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
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
    sender.send(APIRequest::Shoot).map_err(|_| {
        Error::new(
            "Failed to send API request.",
            ErrorSource::Builtin(String::from("robot_api:move")),
        )
    })?;
    thread::park();

    Ok(Data::None)
}

fn fn_raycast(_a: Vec<Data>, _b: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    let binding = RefCell::borrow(&scope).get_file_module().ok_or(Error::new(
        "Cannot connect to api outside of module.",
        ErrorSource::Builtin(String::from("robot_api:move")),
    ))?;
    let borrowed = RefCell::borrow(&binding);
    let registry = RefCell::borrow(
        &as_type!(borrowed => CustomModule, "Returned non-CustomModule from get_file_module")
            .registry,
    );
    let raycast = registry
        .metadata
        .get("sender")
        .ok_or(Error::new(
            "Couldn't access API message sender.",
            ErrorSource::Builtin(String::from("robot_api:[internal]")),
        ))?
        .downcast_ref::<Sender<APIRequest>>()
        .ok_or(Error::new(
            "API message sender was incorrect type.",
            ErrorSource::Builtin(String::from("robot_api:[internal]")),
        ));

    Ok(Data::None)
}
