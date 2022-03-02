use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\nPuzzle 10:\n");

    let mut lines: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut lines)?;

    let mut numbers: Vec<Vec<i32>> = Vec::new();
    for x in 0..lines.len() {
        numbers.push(Vec::new());
        for cha in lines[x].chars() {
            numbers[x].push(cha.to_string().parse::<i32>()?);
        }
    }

    // Has all numbers
    let mut sum_of_risk = 0;

    let mut cur;
    for y in 0..numbers.len() {
        for x in 0..numbers[y].len() {
            cur = 0;
            // check above, check right
            // check down, check left
            match numbers.get(((y as i32)-1) as usize) {
                Some(val) => {
                    if numbers[y][x] < val[x] {
                        cur += 1;
                    }
                },
                None => cur += 1
            }
            match numbers[y].get(((x as i32)-1) as usize) {
                Some(&val) => {
                    if numbers[y][x] < val {
                        cur += 1;
                    }
                },
                None => cur += 1
            }
            match numbers[y].get(((x as i32)+1) as usize) {
                Some(&val) => {
                    if numbers[y][x] < val {
                        cur += 1;
                    }
                },
                None => cur += 1
            }
            match numbers.get(((y as i32)+1) as usize) {
                Some(val) => {
                    if numbers[y][x] < val[x] {
                        cur += 1;
                    }
                },
                None => cur += 1
            }
            
            if cur == 4 {
                //println!("{}", numbers[y][x]);
                sum_of_risk += numbers[y][x] + 1;
            }
        }
    }
    // 1 + height

    println!("Part 1: {}", sum_of_risk);
    let mut all_spots: Vec<Vec<Spot>> = Vec::new();
    for y in 0..numbers.len() {
        all_spots.push(Vec::new());
        for x in 0..numbers[y].len() {
            all_spots[y].push(Spot { 
                is_basin: false, 
                is_nine: numbers[y][x] == 9 
            });
        }
    }
    //println!("Done");
    let mut basins: Vec<i32> = Vec::new();
    for y in 0..all_spots.len() {
        for x in 0..all_spots[y].len() {
            if all_spots[y][x].is_basin || all_spots[y][x].is_nine {
                continue;
            }
            let current_basin = backtrack(&(y as i32), &(x as i32), &mut all_spots);
            //println!("{}", current_basin);
            basins.push(current_basin);
        }
    }
    basins.sort();
    basins.reverse();
    println!("Part 2: {}", basins[0] * basins[1] * basins[2]);
    Ok(())
}

fn backtrack(y: &i32, x: &i32, map: &mut Vec<Vec<Spot>>) -> i32 {
    if !(*y < map.len() as i32 && *y >= 0 && *x >= 0 && *x < map[*y as usize].len() as i32) {
        return 0;
    }

    if map[*y as usize][*x as usize].is_basin || map[*y as usize][*x as usize].is_nine {
        return 0;
    }

    map[*y as usize][*x as usize].is_basin = true;
    return 1 + 
    backtrack(&(*y - 1), x, map) +
    backtrack(&(*y + 1), x, map) +
    backtrack(y, &(*x - 1), map) +
    backtrack(y, &(*x + 1), map);
}

struct Spot {
    is_basin: bool,
    is_nine: bool
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