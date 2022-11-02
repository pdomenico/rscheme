use colored::Colorize;
use std::fmt;
use std::io;
use std::io::Write;

mod primitive_procedures;
use primitive_procedures as pp;
mod environment;

use crate::environment::Env;
use primitive_procedures::NumberType;

#[derive(Clone)]
pub enum Value {
    Text(String),
    Number(NumberType),
    Boolean(bool),
    Pair(Box<Value>, Box<Value>),
    Procedure(Vec<String>, String),
    Error(String),
    Nothing,
}

static ONLY_NUMBERS_PROCS: [&str; 9] = ["+", "-", "*", "/", "=", ">", "<", ">=", "<="];

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Text(s) => write!(f, "{}\n", s),
            Value::Number(NumberType::Integer(v)) => write!(f, "{}\n", v),
            Value::Number(NumberType::Float(v)) => write!(f, "{}\n", v),
            Value::Error(m) => write!(f, "{}", format!("ERROR: {}\n", m).red().bold()),
            Value::Boolean(v) => write!(f, "{}\n", v),
            Value::Procedure(parameters, body) => write!(
                f,
                "Procedure with {} arguments and body: {}",
                parameters.len(),
                body
            ),
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

        print!("{}", eval(&line, &mut env));
    }
}

fn eval(s: &str, env: &mut Env) -> Value {
    let s = s.trim();
    // println!("Evaluating {}", s);

    // See if it's self evaluating
    if let Ok(v) = s.parse::<i32>() {
        return Value::Number(NumberType::Integer(v));
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

        // Check for special expression (define, let, if, cond, lambda...)
        match exp[0].as_str() {
            "define" => {
                let n_of_args = exp.len();
                if n_of_args != 3 {
                    return Value::Error(String::from("Wrong number of arguments for define!"));
                }

                let var_name = exp[1].as_str();
                environment::add_binding(var_name.to_string(), eval(exp[2].as_str(), env), env);
                return Value::Nothing;
            }
            "let" => return Value::Text(String::from("let expression")),
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
            "cond" => return Value::Text(String::from("cond expression")),
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
        return apply(exp[0].as_str(), &args);
    }

    // See if it's a var bound in current env
    match environment::find_in_env(s, env) {
        Some(v) => return v,
        None => return Value::Error(String::from("This variable does not exist!")),
    }

    return Value::Text(String::from("Expression not yet implemented"));
}

fn get_exp_inside_paren(s: &str) -> Option<Vec<String>> {
    if s == "()" {
        return None;
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

//fn self_evaluating(s: &str) -> Option<T> {}

fn apply(proc: &str, args: &Vec<Value>) -> Value {
    // check for primitive procedures
    if ONLY_NUMBERS_PROCS.contains(&proc) {
        let mut n_args: Vec<&NumberType> = Vec::new();
        for arg in args {
            match arg {
                Value::Number(n) => n_args.push(n),
                _ => return Value::Error("Only numbers allowed!".to_string()),
            }
        }
        match proc {
            "+" => return Value::Number(pp::sum(n_args)),
            "-" => return Value::Number(pp::subtract(n_args)),
            "*" => return Value::Number(pp::mul(n_args)),
            "/" => {
                if n_args.len() != 2 {
                    return Value::Error(String::from("Wrong number of arguments for division!"));
                } else {
                    return Value::Number(pp::div(&n_args[0], &n_args[1]));
                }
            }
            "=" => {
                if n_args.len() != 2 {
                    return Value::Error(String::from("Wrong number of arguments!"));
                }
                return Value::Boolean(pp::equal(&n_args[0], &n_args[1]));
            }
            _ => return Value::Error(String::from("Procedure not yet implemented!")),
        }
    }

    if proc == "p" || proc == "print" {
        let mut to_print = String::new();
        for arg in args {
            to_print.push_str(format!("{}", arg).as_str());
        }
        print!("{}", to_print);
        return Value::Nothing;
    }
    return Value::Error(String::from("Not yet implemented"));
}
