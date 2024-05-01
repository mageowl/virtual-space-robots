use std::rc::Rc;

use bean_script::{
    data::Data,
    error::{Error, ErrorSource},
    modules::ModuleBuilder,
    scope::{function::Function, Scope, ScopeRef},
    util::{make_ref, MutRc},
};

pub fn construct(module: &mut ModuleBuilder) {
    module.function("tick", fn_tick);
}

#[derive(Debug)]
pub enum APICall {
    Forward(f64),
    Turn(f64),
    Shoot,
}

pub enum RaycastResult {
    Rock,
    Opponent,
    None,
}

impl Into<f64> for RaycastResult {
    fn into(self) -> f64 {
        match self {
            RaycastResult::Rock => 0.0,
            RaycastResult::Opponent => 1.0,
            RaycastResult::None => 2.0,
        }
    }
}

#[derive(Debug)]
pub struct RobotScope {
    pub calls: Vec<APICall>,
    parent: MutRc<dyn Scope>,
    raycast: f64,
}

impl Scope for RobotScope {
    fn has_function(&self, name: &str) -> bool {
        self.parent.borrow().has_function(name)
    }

    fn get_function(&self, name: &str) -> Option<Function> {
        self.parent.borrow().get_function(name)
    }

    fn set_function(&mut self, name: &str, function: Function) {
        self.parent.borrow_mut().set_function(name, function)
    }

    fn delete_function(&mut self, name: &str) {
        self.parent.borrow_mut().delete_function(name)
    }

    fn set_return_value(&mut self, value: Data) {
        self.parent.borrow_mut().set_return_value(value)
    }

    fn set_if_state(&mut self, state: bean_script::scope::block_scope::IfState) {
        self.parent.borrow_mut().set_if_state(state)
    }

    fn get_function_list(&self) -> std::collections::HashMap<String, Function> {
        self.parent.borrow().get_function_list()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn fn_tick(_a: Vec<Data>, body: Option<Function>, scope: ScopeRef) -> Result<Data, Error> {
    let body = body.ok_or_else(|| {
        Error::new(
            "Expected body for tick function.",
            ErrorSource::Builtin(String::from("robot_api:tick")),
        )
    })?;
    let parent = Rc::clone(&scope);
    scope.borrow_mut().set_function(
        "__tick",
        Function::BuiltIn {
            callback: Rc::new(move |args: Vec<Data>, _b: Option<Function>, _s: ScopeRef| {
                let robot_scope = make_ref(RobotScope {
                    calls: Vec::new(),
                    parent: Rc::clone(&parent),
                    raycast: if let Data::Number(n) = args[0] {
                        n
                    } else {
                        return Err(Error::new(
                            "Expected to get a number as raycast result. [INTERNAL]",
                            ErrorSource::Builtin(String::from("robot_api:tick")),
                        ));
                    },
                }) as ScopeRef;
                body.call_direct(Vec::new(), None, Rc::clone(&robot_scope))?;

                Ok(Data::Scope(robot_scope))
            }),
        },
    );

    Ok(Data::None)
}
