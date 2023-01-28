use crate::types::check_for_floats;
use crate::types::PrimitiveProcedure;
use crate::types::Value;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::{collections::HashMap, rc::Rc};

pub struct Environment {
    bindings: RefCell<HashMap<String, Value>>,
    enclosing_env: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            bindings: RefCell::new(HashMap::new()),
            enclosing_env: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<Environment>) -> Self {
        Environment {
            bindings: RefCell::new(HashMap::new()),
            enclosing_env: Some(enclosing.clone()),
        }
    }

    pub fn add_value(&self, s: &str, val: Value) {
        self.bindings.borrow_mut().insert(s.to_string(), val);
    }

    pub fn get_value(&self, str: &String) -> Option<Value> {
        match self.bindings.borrow().get(str) {
            Some(value) => Some(value.clone()),
            None => match &self.enclosing_env {
                Some(enclosing) => enclosing.get_value(str),
                None => None,
            },
        }
    }
}

pub fn check_primitive_procedures(
    proc: &str,
) -> Option<Box<dyn Fn(Vec<Value>) -> Result<Value, &'static str>>> {
    match proc {
        "+" => Some(Box::new(|args| match check_for_floats(&args) {
            Ok(true) => Ok(Value::Float(
                args.iter()
                    .map(|v| match v {
                        Value::Integer(n) => *n as f64,
                        Value::Float(n) => *n,
                        _ => unreachable!(),
                    })
                    .sum(),
            )),
            Ok(false) => Ok(Value::Integer(
                args.iter()
                    .map(|v| match v {
                        Value::Integer(n) => *n,
                        _ => unreachable!(),
                    })
                    .sum(),
            )),
            Err(_) => Err("+: Wrong argument types!"),
        })),
        "-" => Some(Box::new(|args| match check_for_floats(&args) {
            Ok(true) => Ok(Value::Float({
                let mut iter = args.iter();
                let a = match iter.next() {
                    Some(Value::Integer(n)) => *n as f64,
                    Some(Value::Float(n)) => *n,
                    _ => 0 as f64,
                };
                let rest: f64 = iter
                    .map(|v| match v {
                        Value::Integer(n) => *n as f64,
                        Value::Float(n) => *n,
                        _ => unreachable!(),
                    })
                    .sum();
                a - rest
            })),
            Ok(false) => Ok(Value::Integer({
                let mut iter = args.iter();
                let a = match iter.next() {
                    Some(Value::Integer(n)) => *n,
                    _ => 0,
                };
                let rest: i64 = iter
                    .map(|v| match v {
                        Value::Integer(n) => *n,
                        _ => unreachable!(),
                    })
                    .sum();
                a - rest
            })),
            Err(_) => Err("-: Wrong argument types!"),
        })),
        "*" => Some(Box::new(|args| match check_for_floats(&args) {
            Ok(true) => Ok(Value::Float({
                args.iter()
                    .map(|v| match v {
                        Value::Float(n) => *n,
                        Value::Integer(n) => *n as f64,
                        _ => unreachable!(),
                    })
                    .reduce(|acc, x| x * acc)
                    .unwrap_or_else(|| 1f64)
            })),
            Ok(false) => Ok(Value::Integer({
                args.iter()
                    .map(|v| match v {
                        Value::Integer(n) => *n,
                        _ => unreachable!(),
                    })
                    .reduce(|acc, x| acc * x)
                    .unwrap_or_else(|| 1i64)
            })),
            Err(_) => Err("*: Wrong argument types!"),
        })),
        "/" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err("/: Wrong argument number for division!");
            }
            let (a, b) = match (&args[0], &args[1]) {
                (Value::Integer(x), Value::Integer(y)) => (*x as f64, *y as f64),
                (Value::Integer(x), Value::Float(y)) => (*x as f64, *y),
                (Value::Float(x), Value::Integer(y)) => (*x, *y as f64),
                (Value::Float(x), Value::Float(y)) => (*x, *y),
                _ => return Err("/: Wrong argument types for division!"),
            };
            if b == 0f64 {
                return Err("/: Cannot divide by 0!");
            }
            Ok(Value::Float(a / b))
        })),
        "%" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err("%: Wrong argument number!");
            }
            match check_for_floats(&args) {
                Ok(true) => {
                    let (a, b) = match (&args[0], &args[1]) {
                        (Value::Integer(x), Value::Integer(y)) => (*x as f64, *y as f64),
                        (Value::Integer(x), Value::Float(y)) => (*x as f64, *y),
                        (Value::Float(x), Value::Integer(y)) => (*x, *y as f64),
                        (Value::Float(x), Value::Float(y)) => (*x, *y),
                        _ => unreachable!(),
                    };
                    if b == 0f64 {
                        return Err("%: Cannot divide by 0");
                    }
                    Ok(Value::Float(a % b))
                }
                Ok(false) => {
                    let (a, b) = match (&args[0], &args[1]) {
                        (Value::Integer(x), Value::Integer(y)) => (*x, *y),
                        _ => unreachable!(),
                    };
                    if b == 0 {
                        return Err("%: Cannot divide by 0!");
                    }
                    Ok(Value::Integer(a % b))
                }
                Err(_) => Err("%: Wrong argument types!"),
            }
        })),
        "=" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err("=: Wrong argument number!");
            }
            return Ok(Value::Boolean(args[0].eq(&args[1])));
        })),
        ">" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err(">: Wrong argument number!");
            }
            return match args[0].partial_cmp(&args[1]) {
                Some(Ordering::Greater) => Ok(Value::Boolean(true)),
                Some(_) => Ok(Value::Boolean(false)),
                _ => Err(">: Can't compare these two values!"),
            };
        })),
        ">=" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err(">=: Wrong argument number!");
            }
            return match args[0].partial_cmp(&args[1]) {
                Some(Ordering::Greater) | Some(Ordering::Equal) => Ok(Value::Boolean(true)),
                Some(_) => Ok(Value::Boolean(false)),
                _ => Err(">=: Can't compare these two values!"),
            };
        })),
        "<" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err("<: Wrong argument number!");
            }
            return match args[0].partial_cmp(&args[1]) {
                Some(Ordering::Less) => Ok(Value::Boolean(true)),
                Some(_) => Ok(Value::Boolean(false)),
                _ => Err("<: Can't compare these two values!"),
            };
        })),
        "<=" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err("<=: Wrong argument number!");
            }
            return match args[0].partial_cmp(&args[1]) {
                Some(Ordering::Less) | Some(Ordering::Equal) => Ok(Value::Boolean(true)),
                Some(_) => Ok(Value::Boolean(false)),
                _ => Err("<=: Can't compare these two values!"),
            };
        })),
        "not" => Some(Box::new(|args| {
            if args.len() != 1 {
                return Err("not: Wrong argument number!");
            }
            return match args[0] {
                Value::Boolean(b) => Ok(Value::Boolean(!b)),
                _ => Err("not: Wrong argument type!"),
            };
        })),
        "and" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err("and: Wrong argument number!");
            }
            return match (&args[0], &args[1]) {
                (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
                _ => Err("and: Wrong argument types!"),
            };
        })),
        "or" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err("or: Wrong argument number!");
            }
            return match (&args[0], &args[1]) {
                (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
                _ => Err("or: Wrong argument types!"),
            };
        })),
        "cons" => Some(Box::new(|args| {
            if args.len() != 2 {
                return Err("cons: Wrong argument number!");
            }
            return Ok(Value::Pair(
                Box::new(args[0].clone()),
                Box::new(args[1].clone()),
            ));
        })),
        "car" => Some(Box::new(|args| {
            if args.len() != 1 {
                return Err("car: Wrong argument number!");
            }
            return match &args[0] {
                Value::Pair(a, _) => Ok(*a.clone()),
                _ => Err("car: Wrong argument type!"),
            };
        })),
        "cdr" => Some(Box::new(|args| {
            if args.len() != 1 {
                return Err("cdr: Wrong argument number!");
            }
            return match &args[0] {
                Value::Pair(_, b) => Ok(*b.clone()),
                _ => Err("cdr: Wrong argument type!"),
            };
        })),
        "list" => Some(Box::new(|args| {
            let mut list = Value::Null;
            for arg in args.iter().rev() {
                list = Value::Pair(Box::new(arg.clone()), Box::new(list));
            }
            return Ok(list);
        })),
        "pair?" => Some(Box::new(|args| {
            if args.len() != 1 {
                return Err("pair?: Wrong argument number!");
            }
            return match &args[0] {
                Value::Pair(_, _) => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            };
        })),
        "null?" => Some(Box::new(|args| {
            if args.len() != 1 {
                return Err("null?: Wrong argument number!");
            }
            match args[0] {
                Value::Null => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            }
        })),
        _ => None,
    }
}
