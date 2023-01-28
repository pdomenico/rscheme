use core::fmt;
use std::{cmp::Ordering, rc::Rc};

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Pair(Box<Value>, Box<Value>),
    Procedure(Vec<String>, Vec<String>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(true) => write!(f, "#t"),
            Value::Boolean(false) => write!(f, "#f"),
            Value::Null => write!(f, "()"),
            Value::Procedure(_, _) => write!(f, ""),
            Value::Pair(car, cdr) => match **cdr {
                Value::Pair(_, _) => {
                    let mut s = String::new();
                    s.push_str("(");
                    s.push_str(&format!("{}", car));
                    let mut current_pair = cdr.clone();
                    loop {
                        match *current_pair {
                            Value::Pair(ref car, ref cdr) => {
                                s.push_str(&format!(" {}", car));
                                current_pair = cdr.clone();
                            }
                            Value::Null => break,
                            _ => {
                                s.push_str(&format!(" . {}", cdr));
                                break;
                            }
                        }
                    }
                    s.push_str(")");
                    write!(f, "{}", s)
                }
                _ => write!(f, "({} . {})", car, cdr),
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.eq(b),
            (Value::Integer(a), Value::Float(b)) => (*a as f64).eq(b),
            (Value::Float(a), Value::Integer(b)) => a.eq(&(*b as f64)),
            (Value::Float(a), Value::Float(b)) => a.eq(b),
            (Value::String(a), Value::String(b)) => a.eq(b),
            (Value::Boolean(a), Value::Boolean(b)) => a.eq(b),
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
    fn ne(&self, other: &Value) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Some(a.cmp(b)),
            (Value::Integer(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Integer(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            _ => None,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => true,
            _ => false,
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => true,
            _ => false,
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

pub type PrimitiveProcedure = Rc<dyn Fn(Vec<Value>) -> Result<Option<Value>, &'static str>>;

pub fn check_for_floats(args: &Vec<Value>) -> Result<bool, ()> {
    for val in args {
        match val {
            Value::Integer(_) => (),
            Value::Float(_) => return Ok(true),
            _ => return Err(()),
        }
    }
    Ok(false)
}
