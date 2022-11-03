use crate::primitive_procedures;
use crate::Value;
use std::collections::HashMap;

pub type Env = Vec<HashMap<String, Value>>;

pub fn default_env() -> Env {
    let mut outer_env = HashMap::new();
    for sym in [
        "+", "-", "*", "/", "=", ">", "<", ">=", "<=", "cons", "car", "cdr",
    ]
    .iter()
    {
        outer_env.insert(sym.to_string(), Value::PrimitiveProcedure(sym.to_string()));
    }
    return vec![outer_env];
}

pub fn add_binding(key: String, value: Value, env: &mut Env) -> &mut Env {
    match env.last_mut() {
        Some(last) => last.insert(key, value),
        None => panic!(),
    };
    return env;
}

pub fn extend_env(env: &Env) -> Env {
    let mut new_env = env.clone();
    new_env.push(HashMap::new());
    return new_env;
}

pub fn find_in_env<'a>(key: &'a str, env: &'a Env) -> Option<Value> {
    for subenv in env {
        match subenv.get(&key.to_string()) {
            Some(v) => return Some(v.clone()),
            _ => (),
        }
    }
    None
}
