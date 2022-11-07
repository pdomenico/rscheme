use colored::Colorize;
use std::fmt;
use std::io;
use std::io::Write;

mod primitive_procedures;
use primitive_procedures as pp;
mod environment;

use crate::environment::Env;

#[derive(Clone)]
pub enum Value {
    Text(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Pair(Box<Value>, Box<Value>),
    Procedure(Vec<String>, String),
    PrimitiveProcedure(String),
    Error(String),
    Nothing,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Text(s) => write!(f, "{}", s),
            Value::Integer(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Error(m) => write!(f, "{}", format!("ERROR: {}", m).red().bold()),
            Value::Boolean(v) => write!(f, "{}", v),
            //Value::Procedure(parameters, body) => write!(f, ""),
            Value::Pair(car, cdr) => write!(f, "({} . {})", car, cdr),
            Value::Nothing => Ok(()),
            _ => write!(f, "Printing this type not implemented yet!\n"),
        }
    }
}

fn main() {
    let mut env = environment::default_env();
    loop {
        print!("{}", format!("rscheme> ").purple().bold());
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Invalid command");

        if line.trim() == "quit" {
            println!("Bye!");
            break;
        }

        print!("{}\n", eval(&line, &mut env));
    }
}

fn eval(s: &str, env: &mut Env) -> Value {
    let s = s.trim();
    // println!("Evaluating {}", s);

    // See if it's self evaluating
    if let Ok(v) = s.parse::<i64>() {
        return Value::Integer(v);
    }

    if let Ok(v) = s.parse::<f64>() {
        return Value::Float(v);
    }

    if s.to_string().starts_with('\'') {
        return Value::Text(s[1..].to_string());
    }

    // If it starts with a parenthesis
    if s.to_string().starts_with('(') {
        let exp: Vec<String>;
        match get_exp_inside_paren(s) {
            Some(vec) => {
                exp = vec;
            }
            None => return Value::Error(String::from("Parenthesis mismatch")),
        }

        if exp[0] == "nil" {
            return Value::Nothing;
        }

        // Check for special expression (define, let, if, cond, lambda...)
        match exp[0].as_str() {
            "define" => {
                let n_of_args = exp.len();

                // see if it's a procedure definition
                if exp[1].to_string().starts_with('(') {
                    if n_of_args != 3 {
                        return Value::Error(String::from("Wrong number of arguments"));
                    }

                    // extract procedure name and parameters
                    let mut new_expression = String::from("(define ");
                    match get_exp_inside_paren(&exp[1]) {
                        Some(second_part) => {
                            new_expression.push_str(&second_part[0]);
                            new_expression.push_str(" (lambda (");
                            for i in 1..second_part.len() {
                                new_expression.push_str(&second_part[i]);
                                if i != second_part.len() - 1 {
                                    new_expression.push_str(" ");
                                }
                            }
                            new_expression.push_str(") ");
                            new_expression.push_str(&exp[2]);
                            new_expression.push_str("))");
                            return eval(&new_expression, env);
                        }
                        None => return Value::Error(String::from("Wrong define statement")),
                    }
                }

                let var_name = exp[1].as_str();
                environment::add_binding(var_name.to_string(), eval(exp[2].as_str(), env), env);
                return Value::Nothing;
            }
            "let" => {
                if exp.len() != 3 {
                    return Value::Error(String::from("Wrong number of arguments"));
                }
                let assignments = get_exp_inside_paren(&exp[1]).unwrap();
                let parameters: Vec<String> = assignments
                    .iter()
                    .map(|x| get_exp_inside_paren(x).unwrap()[0].clone())
                    .collect();
                let values = assignments
                    .iter()
                    .map(|x| get_exp_inside_paren(x).unwrap()[1].clone())
                    .collect::<Vec<String>>();

                let mut new_expression = String::from("((lambda (");
                for (i, par) in parameters.iter().enumerate() {
                    new_expression.push_str(par);
                    if i != parameters.len() - 1 {
                        new_expression.push_str(" ");
                    } else {
                        new_expression.push_str(") ");
                    }
                }
                new_expression.push_str(&exp[2]);
                new_expression.push_str(") ");
                for (i, val) in values.iter().enumerate() {
                    new_expression.push_str(val);
                    if i != values.len() - 1 {
                        new_expression.push_str(" ");
                    } else {
                        new_expression.push_str(")");
                    }
                }
                return eval(&new_expression, env);
            }
            "if" => {
                let n_of_args = exp.len();
                if n_of_args < 3 || n_of_args > 4 {
                    return Value::Error(String::from("Wrong number of arguments!"));
                }
                match eval(&exp[1], env) {
                    Value::Boolean(v) => {
                        if v {
                            return eval(&exp[2], env);
                        } else if n_of_args == 4 {
                            return eval(&exp[3], env);
                        } else {
                            return Value::Nothing;
                        }
                    }
                    _ => return Value::Error(String::from("Condition is not a bool!")),
                }
            }
            "cond" => {
                let n_of_args = exp.len();
                if n_of_args < 2 {
                    return Value::Nothing;
                }
                // get the first expression
                let first = get_exp_inside_paren(&exp[1]).unwrap();
                if first.len() != 2 {
                    return Value::Error(String::from("Wrong cond expression!"));
                }
                // eval the first expression, if it's true, return the eval of the second
                match eval(&first[0], env) {
                    Value::Boolean(v) => {
                        if v {
                            // println!("condition is true");
                            return eval(&first[1], env);
                        } else {
                            // println!("Condition is false");
                            let mut new_exp = String::from("(cond ");
                            // append the rest of the original expression
                            for i in 2..exp.len() {
                                new_exp.push_str(&exp[i]);
                                if i != exp.len() - 1 {
                                    new_exp.push_str(" ");
                                }
                            }
                            new_exp.push_str(")");
                            // println!("New expression: {}", new_exp);
                            return eval(&new_exp, env);
                        }
                    }
                    _ => return Value::Error(String::from("Condition is not a bool!")),
                }
            }
            "lambda" => {
                let n_of_args = exp.len();
                if n_of_args != 3 {
                    return Value::Error(String::from(
                        "Wrong number of arguments for a lambda expression",
                    ));
                }

                let parameters: Vec<String>;
                match get_exp_inside_paren(&exp[1]) {
                    Some(vec) => parameters = vec,
                    None => parameters = Vec::new(),
                }
                let body: String = exp[2].to_string();
                return Value::Procedure(parameters, body);
            }
            _ => (),
        }

        // Apply the procedure TODO
        let args: Vec<Value> = exp[1..]
            .iter()
            .map(|subexp| eval(subexp.as_str(), env))
            .collect();
        return apply(eval(exp[0].as_str(), env), &args, &env);
    }

    // See if it's a var bound in current env
    match environment::find_in_env(s, env) {
        Some(v) => return v,
        None => return Value::Error(String::from("This variable does not exist!")),
    }

    // return Value::Text(String::from("Expression not yet implemented"));
}

fn get_exp_inside_paren(s: &str) -> Option<Vec<String>> {
    if s == "()" {
        return Some(vec![String::from("nil")]);
    }
    // println!("Exp is {}", s);
    let mut subexps: Vec<String> = Vec::new();
    let mut expression_chars: Vec<char> = s.chars().collect();
    let mut paren_count = 0;
    let mut i = 0;
    subexps.push(String::from(""));
    loop {
        // println!("matching {}", expression_chars[i]);
        match expression_chars[i] {
            '(' => {
                paren_count += 1;
                if paren_count == 2 {
                    let x = subexps.last_mut().unwrap();
                    x.push('(');
                    i += 1;
                    while paren_count > 1 {
                        // println!("Inner matching {}", expression_chars[i]);
                        match expression_chars[i] {
                            '(' => {
                                x.push('(');
                                paren_count += 1;
                            }
                            ')' => {
                                x.push(')');
                                paren_count -= 1;
                            }
                            c => x.push(c),
                        }
                        i += 1;
                        if i == expression_chars.len() {
                            let mut another_line = String::new();
                            io::stdin()
                                .read_line(&mut another_line)
                                .expect("Invalid command");
                            expression_chars.push(' ');
                            for c in another_line.trim().chars() {
                                expression_chars.push(c);
                            }
                        }
                    }
                    continue;
                }
            }
            ')' => paren_count -= 1,
            ' ' => {
                subexps.push(String::from(""));
            }
            c => {
                let x = subexps.last_mut();
                if let Some(v) = x {
                    v.push(c)
                }
            }
        }
        i += 1;
        if i == expression_chars.len() {
            if paren_count == 0 {
                break;
            }
            // println!("here");
            let mut another_line = String::new();
            io::stdin()
                .read_line(&mut another_line)
                .expect("Invalid command");
            expression_chars.push(' ');
            for c in another_line.trim().chars() {
                expression_chars.push(c);
                // subexps.push(String::from(""));
            }
        }
    }

    // println!("Paren count is {paren_count}");
    if paren_count != 0 {
        return None;
    } else {
        return Some(subexps);
    }
}

fn apply(proc: Value, args: &Vec<Value>, env: &Env) -> Value {
    // Check for primitive procedures
    if let Value::PrimitiveProcedure(ref name) = proc {
        let name = name.as_str();
        match name {
            "+" => match pp::sum(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for +")),
            },
            "-" => match pp::subtract(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for -")),
            },
            "*" => match pp::mul(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for *")),
            },
            "/" => match pp::div(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for /")),
            },
            "eq?" | "=" => match pp::equal(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for eq?")),
            },
            ">" => match pp::greater_than(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for >")),
            },
            ">=" => match pp::greater_or_equal_than(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for >=")),
            },
            "<" => match pp::less_than(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for <")),
            },
            "<=" => match pp::less_or_equal_than(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for <=")),
            },
            "%" | "mod" | "modulo" => match pp::modulo(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for modulo")),
            },
            "and" => {
                for arg in args {
                    if let Value::Boolean(b) = arg {
                        if !b {
                            return Value::Boolean(false);
                        }
                    } else {
                        return Value::Error(String::from("Wrong arguments for and"));
                    }
                }
                return Value::Boolean(true);
            }
            "or" => {
                for arg in args {
                    if let Value::Boolean(b) = arg {
                        if *b {
                            return Value::Boolean(true);
                        }
                    } else {
                        return Value::Error(String::from("Wrong arguments for or"));
                    }
                }
                return Value::Boolean(false);
            }
            "not" => {
                if args.len() != 1 {
                    return Value::Error(String::from("Wrong arguments for not"));
                }
                if let Value::Boolean(b) = &args[0] {
                    return Value::Boolean(!b);
                } else {
                    return Value::Error(String::from("Wrong arguments for not"));
                }
            }
            "cons" => match pp::cons(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for cons")),
            },
            "car" => match pp::car(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for car")),
            },
            "cdr" => match pp::cdr(args) {
                Some(v) => return v,
                None => return Value::Error(String::from("Wrong arguments for cdr")),
            },
            _ => (),
        }
    }

    // Check for user-defined procedures
    if let Value::Procedure(parameters, body) = proc {
        // extend the current environment with the new bindings
        let mut new_env = environment::extend_env(env);
        for (i, p) in parameters.iter().enumerate() {
            environment::add_binding(p.to_string(), args[i].clone(), &mut new_env);
        }
        // return evalutation of the body of the procedure in the new environment
        return eval(body.as_str(), &mut new_env);
    }

    return Value::Error(String::from("Procedure not yet implemented"));
}
