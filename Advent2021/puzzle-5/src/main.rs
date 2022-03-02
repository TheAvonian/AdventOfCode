//https://adventofcode.com/2021/day/5
#![allow(unused_variables)]
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

const SIZE: usize = 1000;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
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
    for row in 0..SIZE {
        for col in 0..SIZE {
            if map.grid[row * SIZE + col] >= 2 {
                total += 1;
            }
            
            //print!("{} ", match map.grid[col * SIZE + row] {
            //    0 => ".".to_string(),
            //    val @ _ => format!("{}",val)
            //});
        }
        //println!();
    }
    println!("Counting complete!\n\nTotal overlaps: {}", total);
    Ok(())
}
struct Point(i32, i32);
struct Line {
    p1: Point,
    p2: Point
}
impl Line {
    fn from(line: String) -> Self {
        let sections = line.split(' ').collect::<Vec<&str>>();
        let point1 = sections[0].split(',').collect::<Vec<&str>>();
        let point2 = sections[2].split(',').collect::<Vec<&str>>();

        Self { p1: Point(point1[0].parse::<i32>().expect("Bad parse"), point1[1].parse::<i32>().expect("Bad parse")), p2: Point(point2[0].parse::<i32>().expect("Bad parse"), point2[1].parse::<i32>().expect("Bad parse")) }
    }
}
struct Grid {
    grid: Box<[i32]>,
}

impl Grid {
    fn new() -> Self {
        Self { grid: [0; SIZE * SIZE].to_vec().into_boxed_slice() }
    }
    fn draw(&mut self, line: &Line) {
        if line.p1.0 != line.p2.0 && line.p1.1 != line.p2.1 {
            let forwards_diagonal = (line.p1.0 - line.p2.0) * (line.p1.1 - line.p2.1) > 0;

            let x_start = line.p1.0.min(line.p2.0);
            let y_start = match forwards_diagonal {
                true => line.p1.1.min(line.p2.1),
                false => line.p1.1.max(line.p2.1)
            };

            for i in 0..=(line.p1.0 - line.p2.0).abs() {
                if forwards_diagonal{
                    self.grid[((x_start + i) * SIZE as i32 + y_start + i) as usize] += 1;
                } else {
                    self.grid[((x_start + i) * SIZE as i32 + y_start - i) as usize] += 1;
                }
            }
        }
        


        /*if (line.p1.0 as i32 - line.p2.0 as i32).abs() == (line.p1.1 as i32 - line.p2.1 as i32).abs() || (line.p1.0 as i32 - line.p2.1 as i32).abs() == (line.p1.1 as i32 - line.p2.0 as i32).abs() {
            // diagonals
            let slope_row = (line.p2.0 as i32 - line.p1.0 as i32).signum();
            let slope_col = (line.p2.1 as i32 - line.p1.1 as i32).signum();
            let mut cur_row = line.p1.0 as i32;
            let mut cur_col = line.p1.1 as i32;
            while slope_row * cur_row <= line.p2.0 as i32 && slope_col * cur_col <= line.p2.1 as i32 {
                self.grid[(cur_row * SIZE as i32 + cur_col) as usize] += 1;
                println!("{}", cur_row);
                cur_row += slope_row;
                cur_col += slope_col;
            }

            return;
        }*/


        if line.p1.0 == line.p2.0 || line.p1.1 == line.p2.1  {
            let start_row = line.p1.0.min(line.p2.0);
            let end_row = line.p1.0.max(line.p2.0);
            let start_col = line.p1.1.min(line.p2.1);
            let end_col = line.p1.1.max(line.p2.1);
            for row in start_row..=end_row {
                // horizontal and vertical works
                for col in start_col..=end_col {
                    self.grid[(row * SIZE as i32 + col) as usize] += 1;
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