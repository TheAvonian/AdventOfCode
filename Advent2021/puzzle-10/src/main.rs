use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;


const PARENTH: i64 = 3;
const BRACK: i64 = 57;
const BRACE: i64 = 1197;
const ANGLED: i64 = 25137;

const PARENTH_2: i64 = 1;
const BRACK_2: i64 = 2;
const BRACE_2: i64 = 3;
const ANGLED_2: i64 = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\nPuzzle 10:\n");

    let mut lines: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut lines)?;


    let mut stack = Vec::new();
    let mut error_total: i64 = 0;
    for line in &lines {
        for ch in line.chars() {
            match ch {
                ')' => {
                    match stack.last() {
                        Some(&val) => if val == '(' {
                            stack.pop();
                        } else {
                            error_total += PARENTH;
                            break;
                        },
                        None => {
                            ()//error_total += PARENTH;
                        }
                    }
                },
                ']' => {
                    match stack.last() {
                        Some(&val) => if val == '[' {
                            stack.pop();
                        } else {
                            error_total += BRACK;
                            break;
                        },
                        None => ()//error_total += BRACK
                    }
                },
                '}' => {
                    match stack.last() {
                        Some(&val) => if val == '{' {
                            stack.pop();
                        } else {
                            error_total += BRACE;
                            break;
                        },
                        None => ()//error_total += BRACE
                    }
                },
                '>' => {
                    match stack.last() {
                        Some(&val) => if val == '<' {
                            stack.pop();
                        } else {
                            error_total += ANGLED;
                            break;
                        },
                        None => ()//error_total += ANGLED
                    }
                },
                _ => stack.push(ch)
            }
        }
        let len = stack.len();
        for _ in 0..len {
            stack.pop();
        }
        //println!("{:?}", stack);
    }

    
    println!("Part 1: {}", error_total);

    let mut errored;
    let mut error_totals: Vec<i64> = Vec::new();
    for line in &lines {
        errored = false;
        for ch in line.chars() {
            match ch {
                ')' => {
                    match stack.last() {
                        Some(&val) => if val == '(' {
                            stack.pop();
                        } else {
                            errored = true;
                        },
                        None => {
                            ()//error_total += PARENTH;
                        }
                    }
                },
                ']' => {
                    match stack.last() {
                        Some(&val) => if val == '[' {
                            stack.pop();
                        } else {
                            errored = true;
                        },
                        None => ()//error_total += BRACK
                    }
                },
                '}' => {
                    match stack.last() {
                        Some(&val) => if val == '{' {
                            stack.pop();
                        } else {
                            errored = true;
                        },
                        None => ()//error_total += BRACE
                    }
                },
                '>' => {
                    match stack.last() {
                        Some(&val) => if val == '<' {
                            stack.pop();
                        } else {
                            errored = true;
                        },
                        None => ()//error_total += ANGLED
                    }
                },
                _ => stack.push(ch)
            }
        }
        if errored {
            let len = stack.len();
            for _ in 0..len {
                stack.pop();
            }
        } else {
            error_total = 0;
            for x in (0..stack.len()).rev() {
                error_total = error_total * 5 + match stack[x] {
                    '(' => PARENTH_2,
                    '[' => BRACK_2,
                    '{' => BRACE_2,
                    '<' => ANGLED_2,
                    _ => 0
                };
                println!("Character: {}, Value: {}", stack[x], error_total);
            }
            error_totals.push(error_total);

            println!("{:?} = {}", stack, error_total);
            let len = stack.len();
            for _ in 0..len {
                stack.pop();
            }
        }
    }

    error_totals.sort();

    println!("Part 2: {}", error_totals[error_totals.len() / 2]);
    Ok(())
}

fn load_file_into_vec(path_str: &str, lines: &mut Vec<String>) -> Result<(), std::io::Error> {
    let path = Path::new(path_str);
    
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents).expect("Can't read file.");

    println!("Importing data into vec...");
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    println!("Importing done!\n");
    Ok(())
}