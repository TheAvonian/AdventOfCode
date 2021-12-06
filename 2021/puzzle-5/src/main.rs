//https://adventofcode.com/2021/day/5

use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    println!("\n\nPuzzle 5: \n");

    let mut lines: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut lines)?;

    println!("\nGenerating map...");
    let mut map = Grid::new();
    println!("Map generated!\n");
    println!("Drawing lines on map...");
    for line in lines {
        map.draw(&Line::from(line));
    }
    println!("Done drawing!\n");
    println!("Counting overlaps...");
    let mut total = 0;
    for row in 0..1000 {
        for col in 0..1000 {
            if map.grid[row * 1000 + col] >= 2 {
                total += 1;
            }
        }
    }
    println!("Counting complete!\n\nTotal overlaps: {}", total);
    Ok(())
}
struct Point(usize, usize);
struct Line {
    p1: Point,
    p2: Point
}
impl Line {
    fn from(line: String) -> Self {
        let sections = line.split(' ').collect::<Vec<&str>>();
        let point1 = sections[0].split(',').collect::<Vec<&str>>();
        let point2 = sections[2].split(',').collect::<Vec<&str>>();

        Self { p1: Point(point1[0].parse::<usize>().expect("Bad parse"), point1[1].parse::<usize>().expect("Bad parse")), p2: Point(point2[0].parse::<usize>().expect("Bad parse"), point2[1].parse::<usize>().expect("Bad parse")) }
    }
}
struct Grid {
    grid: Box<[i32]>,
}

impl Grid {
    fn new() -> Self {
        Self { grid: [0; 1000000].to_vec().into_boxed_slice() }
    }
    fn draw(&mut self, line: &Line) {
        if line.p1.0 == line.p2.0 || line.p1.1 == line.p2.1 || line.p1.0 - line.p2.0 == line.p1.1 - line.p2.1 {
            let start_row = line.p1.0.min(line.p2.0);
            let end_row = line.p1.0.max(line.p2.0);
            let start_col = line.p1.1.min(line.p2.1);
            let end_col = line.p1.1.max(line.p2.1);
            for row in start_row..=end_row {
                // fix for 45 degree by separating
                for col in start_col..=end_col {
                    self.grid[row * 1000 + col] += 1;
                }
            }
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