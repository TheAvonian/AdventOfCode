//https://adventofcode.com/2021/day/4
#![allow(dead_code, unused_assignments)]
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("\n\nPuzzle 4:\n");

    let mut lines: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut lines)?;


    println!("\nCollecting draws...");
    let draws = lines[0].split(',').map(|x| x.parse::<i32>().expect("Parse fail")).collect::<Vec<i32>>();
    lines.remove(0);
    lines.remove(0);

    println!("Collected!\n");
    println!("Creating boards...");
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut count = 0;
    for i in 0..lines.len()-4 {
        if count > 0 {
            count-=1;
            continue;
        }
        let cur_strings = &[&lines[i], &lines[i+1], &lines[i+2], &lines[i+3], &lines[i+4]];
        let bor: BingoBoard = BingoBoard::from(cur_strings);
        boards.push(bor);
        count = 5;
    }
    println!("Boards: {}\nCreated!\n", boards.len());
    println!("Playing game...");
    let mut first_score = 0;
    let mut score = 0;
    let mut first = true;
    for draw in &draws {
        for x in 0..boards.len() {
            //println!("{}",x);
            boards[x].set_active(*draw);
            if !boards[x].has_bingo && boards[x].is_bingo() {
                //println!("{}: {}", x, i);
                if first {
                    first_score = boards[x].get_score(*draw);
                    first = false;
                } else {
                    score = boards[x].get_score(*draw);
                }
                boards[x].has_bingo = true;
            }
        }
    }
    println!("Finished!\n\nOutput score is: {}\n\n", first_score);

    println!("Resetting boards...");
    /*for x in 0..boards.len() {
        if !boards[x].has_bingo {
            println!("{}", x);
            for row in 0..boards[x].grid.len() {
                for col in 0..boards[x].grid[row].len() {
                    print!(" {}:{} ", boards[x].grid[row][col].0,boards[x].grid[row][col].1);
                }
                println!();
            }
            println!();
        }
    }*/
    println!("Finished!\n\nOutput score is: {}\n\n{}", score, draws.len());
    Ok(())
}

/**
 * `i32`: value
 * `bool`: active
 */
#[derive(Clone, Copy)]
struct Tile(i32, bool);
struct BingoBoard {
    has_bingo: bool,
    grid: [[Tile; 5]; 5],
}

impl BingoBoard {
    fn get_score(&self, num: i32) -> i32 {
        let mut sum = 0;
        for row in self.grid {
            for col in row {
                if !col.1 {
                    sum += col.0;
                }
            }
        }
        //println!("{}\n", sum * num);
        sum * num
    }

    fn set_active(&mut self, num: i32) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x].0 == num {
                    self.grid[y][x].1 = true;
                    return;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        for row in self.grid {
            if row[0].1 && row[1].1 && row[2].1 && row[3].1 && row[4].1 {
                //println!("Horizontal Bingo");
                return true;
            }
        }
        for x in 0..5 {
            if self.grid[0][x].1 && self.grid[1][x].1 && self.grid[2][x].1 && self.grid[3][x].1 && self.grid[4][x].1 {
                //println!("Vertical Bingo");
                return true;
            } 
        }
        return false;
    }

    fn get(&self, row: usize, col: usize) -> Option<&Tile> {
        match self.grid.get(row) {
            Some(res) => match res.get(col) {
                Some(val) => Some(val),
                None => None
            },
            None => None
        }
    }

    fn from(vals: &[&String; 5]) -> Self {
        let mut grid: [[Tile; 5]; 5] = [[Tile(0,false); 5]; 5];
        let mut row_i = 0;
        let mut col_i = 0;
        for col in vals {
            col_i = 0;
            for val in col.split(' ').collect::<Vec<&str>>() {
                if val.is_empty() {
                    continue;
                }
                let v = val.parse::<i32>().expect("Parse Err");
                grid[row_i][col_i] = Tile(v, false);
                col_i+=1;
            }
            row_i+=1;
        }
        Self { has_bingo: false, grid }
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