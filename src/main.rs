use colored::*;
use std::rc::Rc;

use rscheme::environment::Environment;
use rscheme::read_from_file;
use rscheme::types::Value;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::Result as RsResult;

fn main() -> RsResult<()> {
    println!("Welcome to RScheme!\nPrompt \"quit\" to quit the interpreter");
    let mut rl = Editor::<()>::new()?;
    let env = Rc::new(Environment::new());
    'outer: loop {
        let readline = rl.readline("\x1b[33;1mrscheme>\x1b[0m ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.trim() == "quit" {
                    break;
                }
                // Count the parentheses, if they are not equal, then we need to read more lines
                let mut paren_count = 0;
                for c in line.chars() {
                    if c == '(' {
                        paren_count += 1;
                    } else if c == ')' {
                        paren_count -= 1;
                    }
                }
                let mut exp = String::from(line.trim());
                if paren_count != 0 {
                    'inner: loop {
                        let readline = rl.readline("");
                        match readline {
                            Ok(line) => {
                                rl.add_history_entry(line.as_str());
                                for c in line.chars() {
                                    if c == '(' {
                                        paren_count += 1;
                                    } else if c == ')' {
                                        paren_count -= 1;
                                    }
                                }
                                exp.push_str(line.trim());
                                exp.push_str(" ");
                                if paren_count == 0 {
                                    break 'inner;
                                }
                            }
                            Err(ReadlineError::Interrupted) => {
                                // println!("CTRL-C");
                                continue 'outer;
                            }
                            Err(ReadlineError::Eof) => {
                                println!("CTRL-D");
                                break 'outer;
                            }
                            Err(err) => {
                                println!("Error: {:?}", err);
                                break 'outer;
                            }
                        }
                    }
                }

                match eval(exp.trim(), env.clone()) {
                    Ok(val) => println!("{}", val),
                    Err(e) => println!("{}", format!("ERROR: {e}").red()),
                }
            }
            Err(ReadlineError::Interrupted) => {
                // println!("CTRL-C");
                continue 'outer;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn eval(exp: &str, env: Rc<Environment>) -> Result<Value, &'static str> {
    if exp.len() == 0 {
        return Ok(Value::Null);
    }

    // Handle self-evaluating expressions
    if let Ok(n) = exp.parse::<i64>() {
        return Ok(Value::Integer(n));
    }
    if let Ok(n) = exp.parse::<f64>() {
        return Ok(Value::Float(n));
    }
    if exp.starts_with('\'') {
        return Ok(Value::String(String::from(&exp[1..])));
    }
    match exp {
        "#t" => return Ok(Value::Boolean(true)),
        "#f" => return Ok(Value::Boolean(false)),
        _ => (),
    }

    // Check in the environment
    if let Some(val) = env.get_value(&exp.to_string()) {
        return Ok(val);
    }

    let tokens = get_exp_inside_paren(exp)?;
    if tokens.len() == 0 {
        return Ok(Value::Null);
    }

    // Special forms
    match tokens[0] {
        "load" => {
            if tokens.len() != 2 {
                return Err("Wrong number of arguments to load");
            }
            let filename = tokens[1].trim();
            let lines = read_from_file::read(filename)?;
            let mut last = Ok(Value::Null);
            for line in lines {
                last = eval(line.as_str(), env.clone());
            }
            return last;
        }
        "define" => {
            if tokens[1].starts_with('(') {
                if tokens.len() < 3 {
                    return Err("Wrong number of arguments to define");
                }
                let mut exp = get_exp_inside_paren(tokens[1])?.into_iter();
                if exp.len() < 1 {
                    return Err("Wrong form for define!");
                }
                let proc_name = exp.next().unwrap();
                let mut expression = "(define ".to_string() + proc_name + " (lambda (";
                exp.for_each(|e| expression.push_str((e.to_string() + " ").as_str()));
                expression = expression.trim().to_string() + ") " + tokens[2] + "))";
                return eval(expression.as_str(), env);
            }
            if tokens.len() != 3 {
                return Err("Wrong argument number for define!");
            }
            let (var_name, var_value) = (tokens[1], eval(tokens[2], env.clone())?);
            env.add_value(var_name, var_value.clone());
            return Ok(var_value);
        }
        "lambda" => {
            if tokens.len() < 3 {
                return Err("Wrong argument number for lambda!");
            }
            if !tokens[1].starts_with('(') || !tokens[2].starts_with('(') {
                return Err("Wrong argument form for lambda!");
            }
            let args: Vec<String> = get_exp_inside_paren(tokens[1])?
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            let body = tokens.iter().skip(2).map(|s| s.to_string()).collect();
            return Ok(Value::Procedure(args, body));
        }
        "if" => {
            if tokens.len() != 3 && tokens.len() != 4 {
                return Err("Wrong argument number for if!");
            }

            return match eval(tokens[1], env.clone()) {
                Ok(Value::Boolean(b)) => {
                    if b {
                        eval(tokens[2], env.clone())
                    } else {
                        match tokens.iter().nth(3) {
                            Some(exp) => eval(exp, env.clone()),
                            None => Ok(Value::Null),
                        }
                    }
                }
                _ => Err("Not a valid boolean condition for if!"),
            };
        }
        "cond" => {
            if tokens.len() < 2 {
                return Err("Wrong argument number for cond!");
            }
            let first_expression = get_exp_inside_paren(tokens[1])?;
            if first_expression.len() != 2 {
                return Err("Wrong argument number for cond!");
            }
            return match eval(first_expression[0], env.clone())? {
                Value::Boolean(true) => eval(first_expression[1], env.clone()),
                Value::Boolean(false) => {
                    if tokens.len() == 2 {
                        return Ok(Value::Null);
                    } else {
                        let mut new_exp = String::from("(cond ");
                        for i in 2..tokens.len() {
                            new_exp.push_str((tokens[i].to_string() + " ").as_str());
                        }
                        new_exp.push_str(")");
                        return eval(new_exp.as_str(), env.clone());
                    }
                }
                _ => Err("Not a valid boolean condition for cond!"),
            };
        }
        _ => (),
    }

    // Handle procedure call
    if exp.starts_with("(") {
        return match tokens.len() {
            0 => Ok(Value::Null),
            1 => apply(tokens[0], None, env),
            _ => apply(tokens[0], Some(tokens[1..].to_vec()), env),
        };
    }

    Ok(Value::Null)
}

fn apply(proc: &str, args: Option<Vec<&str>>, env: Rc<Environment>) -> Result<Value, &'static str> {
    // Evaluate the arguments
    let evaled_args: Vec<Value> = match args {
        None => Vec::new(),
        Some(args) => {
            let mut ret = Vec::new();
            for arg in args {
                match eval(arg, env.clone()) {
                    Ok(val) => ret.push(val),
                    Err(e) => return Err(e),
                }
            }
            ret
        }
    };

    // Check for primitive procedures
    if let Some(primitive_proc) = rscheme::environment::check_primitive_procedures(proc) {
        return primitive_proc(evaled_args);
    }

    // Handle custom procedures
    if let Value::Procedure(param, body) = eval(proc, env.clone())? {
        if param.len() != evaled_args.len() {
            return Err("Wrong argument number for {proc}!");
        }
        let new_env = Rc::new(Environment::new_with_enclosing(env.clone()));
        param
            .into_iter()
            .zip(evaled_args.into_iter())
            .for_each(|(param, arg)| {
                new_env.add_value(param.as_str(), arg);
            });
        let mut res = Value::Null;
        for exp in body {
            res = eval(exp.as_str(), new_env.clone())?;
        }
        return Ok(res);
    }

    Err("{proc} is not a procedure!")
}
fn get_exp_inside_paren(exp: &str) -> Result<Vec<&str>, &'static str> {
    fn helper(exp: &str) -> Result<Vec<&str>, &'static str> {
        if exp.len() == 0 {
            return Ok(Vec::new());
        }
        if exp == ")" {
            return Ok(Vec::new());
        }

        for (i, char) in exp.chars().enumerate() {
            match char {
                '(' => {
                    let mut p_count = 1;
                    let mut j = i + 1;
                    while p_count > 0 {
                        if j > exp.len() - 1 {
                            return Err("Parenthesis mismatch");
                        }
                        match exp.chars().nth(j) {
                            Some('(') => {
                                p_count += 1;
                                j += 1;
                            }
                            Some(')') => {
                                p_count -= 1;
                                j += 1;
                            }
                            _ => j += 1,
                        }
                    }
                    let (token, rest) = exp.split_at(j);
                    let mut res = helper(rest)?;
                    res.push(token.trim());
                    return Ok(res);
                }
                ')' => return Err("Parenthesis mismatch"),
                ' ' => {
                    let (token, rest) = exp.split_at(i + 1);
                    let mut res = helper(rest)?;
                    let token = token.trim();
                    if token != "" {
                        res.push(token.trim());
                    }
                    return Ok(res);
                }
                _ => (),
            }
        }

        Ok(vec![exp])
    }

    let last = exp.chars().last();
    if last != Some(')') {
        return Err("Parenthesis mismatch");
    }

    let mut res = helper(&exp[1..(exp.len() - 1)])?;
    res.reverse();
    Ok(res)
}

// fn main() {
//     let res = eval("(+ 1 5)", &Environment::new());
//     match res {
//         Ok(Value::Integer(n)) => println!("{n}"),
//         _ => println!("Not what you wanted!"),
//     }
// }
