use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\nPuzzle 7:\n");

    let mut line: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut line)?;

    let list: Vec<i32> = line[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    //let list = [16,1,2,0,4,2,7,1,2,14];
    let mut max = list[0];
    for x in &list {
        if x > &max {
            max = *x;
        }
    }
    println!("Max number: {}\n", max);
    println!("First number: {}", list[0]);
    println!("Last number: {}", list[list.len()-1]);
    let mut lowest_move = -1;
    let mut total_fuel = std::i32::MAX;
    let mut sum;
    for horizontal_pos in 0..=max as i32 {
        sum = 0;
        for x in &list {
            sum += (*x - horizontal_pos).abs();
        }
        if sum < total_fuel {
            total_fuel = sum;
            lowest_move = horizontal_pos;
        }
    }
    println!("Lowest fuel needed: {}", total_fuel);
    println!("Lowest fuel use position: {}\n\nPart 2:\n", lowest_move);

    let mut lowest_move = -1;
    let mut total_fuel = std::i32::MAX;
    for horizontal_pos in 0..=max as i32 {
        sum = 0;
        for x in &list {
            for l in 1..=(*x - horizontal_pos).abs() {
                sum += l;
            }
        }
        if sum < total_fuel {
            total_fuel = sum;
            lowest_move = horizontal_pos;
            println!("Current fuel: {}", total_fuel);
            println!("Current horizontal pos: {}", lowest_move);
        }
        
    }
    println!("Lowest fuel needed: {}", total_fuel);
    println!("Lowest fuel use position: {}", lowest_move);

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