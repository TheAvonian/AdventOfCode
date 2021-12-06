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

    println!("Converting data for part 1...");
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
    println!("Output for part 1 is {}\n",epsilon_rate * gamma_rate);
    drop(epsilon_rate);
    drop(gamma_rate);

    let mut lines_left: Vec<String> = lines.to_vec();
    
    let mut cur_ind = 0;
    let mut remove_char;

    println!("Converting for Oxygen...");
    while lines_left.len() > 1 {
        ones = 0;
        zeroes = 0;

        for lin in 0..lines_left.len() {
            match lines_left[lin].as_bytes()[cur_ind] as char {
                '1' => ones += 1,
                '0' => zeroes += 1,
                _ => ()
            };
        }

        remove_char = match ones >= zeroes {
            true => "0".to_string(),
            false => "1".to_string()
        };

        for x in (0..lines_left.len()).rev() {
            if lines_left[x].as_bytes()[cur_ind] == remove_char.as_bytes()[0] {
                lines_left.remove(x);
            }
        }
        cur_ind += 1;
    }

    let oxygen_gen_rating = lines_left[0].to_string();

   // lines_left.remove(0);
    println!("Converting done!\n\nConverting for Carbon Dioxide...");
    cur_ind = 0;
    lines_left = lines.to_vec();
    while lines_left.len() > 1 {
        ones = 0;
        zeroes = 0;

        for lin in 0..lines_left.len() {
            match lines_left[lin].as_bytes()[cur_ind] as char {
                '1' => ones += 1,
                '0' => zeroes += 1,
                _ => ()
            };
        }

        remove_char = match ones >= zeroes {
            true => "1".to_string(),
            false => "0".to_string()
        };

        for x in (0..lines_left.len()).rev() {
            if lines_left[x].as_bytes()[cur_ind] == remove_char.as_bytes()[0] {
                lines_left.remove(x);
            }
        }
        cur_ind += 1;
    }
    let co2_scrub_rating = lines_left[0].to_string();
    println!("Conversion done!");

    let oxygen_gen_rating = match i32::from_str_radix(&oxygen_gen_rating, 2) {
        Ok(val) => val,
        Err(_) => panic!("Parse error: {}", oxygen_gen_rating)
    };
    let co2_scrub_rating = match i32::from_str_radix(&co2_scrub_rating, 2) {
        Ok(val) => val,
        Err(_) => panic!("Parse error: {}", co2_scrub_rating)
    };

    println!("\nOutput for part 2 is {} * {} = {}\n",oxygen_gen_rating, co2_scrub_rating, oxygen_gen_rating * co2_scrub_rating);

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