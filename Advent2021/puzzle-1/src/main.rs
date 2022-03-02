use std::error::Error;
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;


fn main() {
    println!("\n\nPuzzle 1:\n");

    let path = Path::new("./src/input.txt");
    
    let file = match fs::File::open(path) { Ok(val) => val, Err(error) => panic!("Error: {}", error) };
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents).expect("Can't read file.");

    let mut numbers: Vec<i32> = Vec::new();

    println!("Importing data into vec...");
    for line in contents.lines() {
        
        match line.parse::<i32>() {
            Ok(number) => numbers.push(number),
            Err(err) => panic!("Error parsing: {}", err)
        };
    }
    println!("Importing done!\n\nGetting part 1 output from file...");

    let mut total_increasing = -1;

    let mut last_val: i32 = std::i32::MIN;

    for num in &numbers {
        if num > &last_val {
            total_increasing+=1;
        }
        last_val = *num;
    }

    println!("Output part 1 finished!\n\nOutput p1: {}\n\n", total_increasing);

    println!("Getting part 2 output from file...");

    total_increasing = -1;

    last_val = std::i32::MIN;
    for num in 0..numbers.len() - 2 {
        total_increasing += match numbers[num] + numbers[num+1] + numbers[num+2] {
            val if val > last_val => 1,
            _ => 0
        };
        last_val = numbers[num] + numbers[num+1] + numbers[num + 2];
    }

    println!("Output part 2 finished!\n\nOutput p2: {}\n\n", total_increasing);

    println!("{}", -30 % 26);
}
