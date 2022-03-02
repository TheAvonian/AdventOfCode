use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\nPuzzle 11:\n");

    let mut lines: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut lines)?;
    
    let mut map: Vec<Vec<Octopus>> = Vec::new();
    for x in 0..lines.len() {
        map.push(Vec::new());
        for cha in lines[x].chars() {
            map[x].push(Octopus {
                value: cha.to_string().parse::<i32>()?,
                flashed: false
            });
        }
    }

    let days = 100;
    let mut flashes = 0;
    for day in 1.. { 
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                if !map[row][col].flashed {
                    add_score_at(row, col, &mut map);
                }
            }
        }
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                
                if map[row][col].flashed {
                    flashes += 1;
                    map[row][col].flashed = false;
                }
            }
        }
        println!("After step {}:", day);
        let mut total_0 = 0;
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                print!("{} ", map[row][col].value);
                if map[row][col].value == 0 {
                    total_0 += 1;
                }
            }
            println!();
        }
        println!();
        if total_0 == 100 {
            break;
        }
    }

    println!("Flashes Part 1: {}", flashes);
    Ok(())
}

fn add_score_at(y: usize, x: usize, map: &mut Vec<Vec<Octopus>>) {
    //println!("{},{}",y,x);
    map[y][x].add_score();
    if map[y][x].flashed {
        for yy in (y as i32)-1..=(y as i32)+1 {
            for xx in (x as i32)-1..=(x as i32)+1 {
                if xx == x as i32 && yy == y as i32 {
                    continue;
                }
                if yy < 0 || yy >= map.len() as i32 || xx < 0 || xx >= map[yy as usize].len() as i32 {
                    continue;
                }
                if !map[yy as usize][xx as usize].flashed {
                    add_score_at(yy as usize, xx as usize, map);
                }
            }
        }
    }
}

struct Octopus {
    value: i32,
    flashed: bool
}

impl Octopus {
    fn add_score(&mut self) {
        if !self.flashed {
            self.value += 1;
        }
        
        if self.value > 9 {
            self.flashed = true;
            self.value = 0;
        }
    }
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