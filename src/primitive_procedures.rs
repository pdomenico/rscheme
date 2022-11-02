#[derive(Clone)]
pub enum NumberType {
    Integer(i32),
    Float(f64),
}

pub fn sum(ns: Vec<&NumberType>) -> NumberType {
    let mut sum_int = 0;
    let mut sum_float = 0.0;
    for n in ns {
        match n {
            NumberType::Integer(v) => sum_int += v,
            NumberType::Float(v) => sum_float += v,
        }
    }

    if sum_float == 0.0 {
        return NumberType::Integer(sum_int);
    }

    return NumberType::Float(sum_float + (sum_int as f64));
}

pub fn subtract(ns: Vec<&NumberType>) -> NumberType {
    let res = sum(ns[1..].to_vec());
    match (&ns[0], res) {
        (NumberType::Integer(a), NumberType::Integer(b)) => return NumberType::Integer(*a - b),
        (NumberType::Integer(a), NumberType::Float(b)) => return NumberType::Float(*a as f64 - b),
        (NumberType::Float(a), NumberType::Integer(b)) => return NumberType::Float(*a - b as f64),
        (NumberType::Float(a), NumberType::Float(b)) => return NumberType::Float(*a - b),
    }
}

pub fn mul(ns: Vec<&NumberType>) -> NumberType {
    let mut prod_int = 1;
    let mut prod_float = 1.0;

    for n in ns {
        match n {
            NumberType::Integer(v) => prod_int *= v,
            NumberType::Float(v) => prod_float *= v,
        }
    }
    if prod_float == 1.0 {
        return NumberType::Integer(prod_int);
    }
    return NumberType::Float(prod_float * (prod_int as f64));
}

pub fn div(n: &NumberType, d: &NumberType) -> NumberType {
    match (n, d) {
        (NumberType::Integer(a), NumberType::Integer(b)) => {
            return NumberType::Float(*a as f64 / *b as f64)
        }
        (NumberType::Integer(a), NumberType::Float(b)) => return NumberType::Float(*a as f64 / *b),
        (NumberType::Float(a), NumberType::Integer(b)) => return NumberType::Float(*a / *b as f64),
        (NumberType::Float(a), NumberType::Float(b)) => return NumberType::Float(*a / *b),
    }
}

pub fn equal(a: &NumberType, b: &NumberType) -> bool {
    match (a, b) {
        (NumberType::Integer(va), NumberType::Integer(vb)) => return *va == *vb,
        (NumberType::Float(va), NumberType::Float(vb)) => return *va == *vb,
        (NumberType::Integer(va), NumberType::Float(vb)) => return *va as f64 == *vb,
        (NumberType::Float(va), NumberType::Integer(vb)) => return *va == *vb as f64,
    }
}
