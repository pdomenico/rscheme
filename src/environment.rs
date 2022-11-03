use crate::Value;
use std::collections::HashMap;
use crate::primitive_procedures;

pub type Env = Vec<HashMap<String, Value>>;

pub fn default_env() -> Env {
    return vec![HashMap::new()];
}

pub fn add_binding(key: String, value: Value, env: &mut Env) -> &mut Env {
    match env.last_mut() {
        Some(last) => last.insert(key, value),
        None => panic!(),
    };
    return env;
}

pub fn extend_env(env: &mut Env) -> &mut Env {
    env.push(HashMap::new());
    env
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
