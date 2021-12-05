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
    println!("Importing done!\n\nGetting output from file...");

    let mut total_increasing = -1;

    let mut last_val: i32 = std::i32::MIN;

    for num in &numbers {
        if num > &last_val {
            total_increasing+=1;
        }
        last_val = *num;
    }
    println!("Output finished!\n\nOutput: {}", total_increasing);
}
