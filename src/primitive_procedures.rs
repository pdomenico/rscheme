use crate::Value;

pub fn sum(ns: &Vec<Value>) -> Option<Value> {
    let mut int_sum = 0;
    let mut f_sum: f64 = 0.0;

    for v in ns {
        match *v {
            Value::Integer(i) => int_sum += i,
            Value::Float(f) => f_sum += f,
            _ => return None,
        }
    }

    if f_sum == 0.0 {
        return Some(Value::Integer(int_sum));
    }
    return Some(Value::Float(int_sum as f64 + f_sum));
}

pub fn subtract(ns: &Vec<Value>) -> Option<Value> {
    let res = sum(&ns[1..].to_vec());
    match (&ns[0], res) {
        (Value::Integer(a), Some(Value::Integer(b))) => return Some(Value::Integer(*a - b)),
        (Value::Integer(a), Some(Value::Float(b))) => return Some(Value::Float(*a as f64 - b)),
        (Value::Float(a), Some(Value::Integer(b))) => return Some(Value::Float(*a - b as f64)),
        (Value::Float(a), Some(Value::Float(b))) => return Some(Value::Float(*a - b)),
        _ => return None,
    }
}

pub fn mul(ns: &Vec<Value>) -> Option<Value> {
    let mut prod_int = 1;
    let mut prod_float = 1.0;

    for n in ns {
        match n {
            Value::Integer(i) => prod_int *= i,
            Value::Float(f) => prod_float *= f,
            _ => return None,
        }
    }
    if prod_float == 1.0 {
        return Some(Value::Integer(prod_int));
    }
    return Some(Value::Float(prod_int as f64 * prod_float));
}

pub fn div(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 2 {
        return None;
    }

    match (&ns[0], &ns[1]) {
        (Value::Integer(a), Value::Integer(b)) => return Some(Value::Integer(*a / *b)),
        (Value::Integer(a), Value::Float(b)) => return Some(Value::Float(*a as f64 / *b)),
        (Value::Float(a), Value::Integer(b)) => return Some(Value::Float(*a / *b as f64)),
        (Value::Float(a), Value::Float(b)) => return Some(Value::Float(*a / *b)),
        _ => return None,
    }
}

pub fn equal(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 2 {
        return None;
    }

    match (&ns[0], &ns[1]) {
        (Value::Integer(a), Value::Integer(b)) => return Some(Value::Boolean(*a == *b)),
        (Value::Integer(a), Value::Float(b)) => return Some(Value::Boolean(*a as f64 == *b)),
        (Value::Float(a), Value::Integer(b)) => return Some(Value::Boolean(*a == *b as f64)),
        (Value::Float(a), Value::Float(b)) => return Some(Value::Boolean(*a == *b)),
        _ => return None,
    }
}

pub fn greater_than(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 2 {
        return None;
    }

    match (&ns[0], &ns[1]) {
        (Value::Integer(a), Value::Integer(b)) => return Some(Value::Boolean(*a > *b)),
        (Value::Integer(a), Value::Float(b)) => return Some(Value::Boolean(*a as f64 > *b)),
        (Value::Float(a), Value::Integer(b)) => return Some(Value::Boolean(*a > *b as f64)),
        (Value::Float(a), Value::Float(b)) => return Some(Value::Boolean(*a > *b)),
        _ => return None,
    }
}

pub fn less_than(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 2 {
        return None;
    }

    match (&ns[0], &ns[1]) {
        (Value::Integer(a), Value::Integer(b)) => return Some(Value::Boolean(*a < *b)),
        (Value::Integer(a), Value::Float(b)) => return Some(Value::Boolean((*a as f64) < *b)),
        (Value::Float(a), Value::Integer(b)) => return Some(Value::Boolean(*a < *b as f64)),
        (Value::Float(a), Value::Float(b)) => return Some(Value::Boolean(*a < *b)),
        _ => return None,
    }
}

pub fn greater_or_equal_than(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 2 {
        return None;
    }

    match (&ns[0], &ns[1]) {
        (Value::Integer(a), Value::Integer(b)) => return Some(Value::Boolean(*a >= *b)),
        (Value::Integer(a), Value::Float(b)) => return Some(Value::Boolean(*a as f64 >= *b)),
        (Value::Float(a), Value::Integer(b)) => return Some(Value::Boolean(*a >= *b as f64)),
        (Value::Float(a), Value::Float(b)) => return Some(Value::Boolean(*a >= *b)),
        _ => return None,
    }
}

pub fn less_or_equal_than(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 2 {
        return None;
    }

    match (&ns[0], &ns[1]) {
        (Value::Integer(a), Value::Integer(b)) => return Some(Value::Boolean(*a <= *b)),
        (Value::Integer(a), Value::Float(b)) => return Some(Value::Boolean(*a as f64 <= *b)),
        (Value::Float(a), Value::Integer(b)) => return Some(Value::Boolean(*a <= *b as f64)),
        (Value::Float(a), Value::Float(b)) => return Some(Value::Boolean(*a <= *b)),
        _ => return None,
    }
}

pub fn cons(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 2 {
        return None;
    }
    return Some(Value::Pair(
        Box::new(ns[0].clone()),
        Box::new(ns[1].clone()),
    ));
}

pub fn car(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 1 {
        return None;
    }
    match &ns[0] {
        Value::Pair(a, _) => return Some(*a.clone()),
        _ => return None,
    }
}

pub fn cdr(ns: &Vec<Value>) -> Option<Value> {
    if ns.len() != 1 {
        return None;
    }
    match &ns[0] {
        Value::Pair(_, b) => return Some(*b.clone()),
        _ => return None,
    }
}
