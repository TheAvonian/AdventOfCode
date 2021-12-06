//https://adventofcode.com/2021/day/3
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    // Take gamma rate (most common) and epsilon rate (least common) in binary, 
    // multiply together = result
    println!("\n\nPuzzle 3:\n");

    let mut lines: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut lines)?;

    println!("Converting data...");
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    let mut ones;
    let mut zeroes;

    for char_ind in 0..lines[0].len() {
        ones = 0;
        zeroes = 0;
        for lin in 0..lines.len() {
            match lines[lin].as_bytes()[char_ind] as char {
                '1' => ones += 1,
                '0' => zeroes += 1,
                _ => ()
            };
        }
        if ones > zeroes {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
    }
    let gamma_rate = match i32::from_str_radix(&gamma_rate, 2) {
        Ok(val) => val,
        Err(_) => panic!("Parse error: {}", gamma_rate)
    };
    let epsilon_rate = match i32::from_str_radix(&epsilon_rate, 2) {
        Ok(val) => val,
        Err(_) => panic!("Parse error: {}", epsilon_rate)
    };
    println!("Conversion complete!\n");
    println!("Output is {}",epsilon_rate * gamma_rate);
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