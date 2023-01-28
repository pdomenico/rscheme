use std::fs::File;
use std::io::{self, BufRead};

pub fn read(file: &str) -> Result<Vec<String>, &'static str> {
    let file = match File::open(file) {
        Ok(file) => file,
        Err(_) => return Err("Can't read from file!"),
    };
    let mut reader = io::BufReader::new(file).lines();
    let mut res: Vec<String> = Vec::new();
    let mut p_count = 0;

    while let Some(Ok(line)) = reader.next() {
        line.chars().for_each(|c| match c {
            '(' => p_count += 1,
            ')' => p_count -= 1,
            _ => (),
        });
        if p_count == 0 {
            match res.last_mut() {
                Some(last) => last.push_str(&format!("{}", line.trim())),
                None => res.push(format!("{} ", line)),
            }
            res.push(line);
        } else {
            match res.last_mut() {
                Some(last) => last.push_str(&format!("{} ", line.trim())),
                None => res.push(format!("{} ", line)),
            }
        }
    }

    if p_count != 0 {
        return Err("Mismatched parentheses in file!");
    }

    Ok(res)
}
