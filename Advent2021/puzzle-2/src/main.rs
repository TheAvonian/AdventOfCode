use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

enum Direction {
    None,
    Forward,
    Down,
    Up,
}

struct Command {
    direction: Direction,
    value: i32
}

impl Command {
    fn from(line: &String) -> Self {
        let com_line: Vec<&str> = line.split(' ').collect();
        let direction = match com_line[0] {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => Direction::None
        };
        // this is bad lol
        Self { direction, value: com_line[1].parse::<i32>().unwrap() }
    }
}

#[derive(Default)]
struct SubmarinePart1 {
    horizontal: i32,
    depth: i32,
}

#[derive(Default)]
struct SubmarinePart2 {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

trait TakesCommand {
    fn give_command(&mut self, command: &Command);
}

impl SubmarinePart1 {
    fn get_result(&self) -> i32 {
        return self.horizontal * self.depth;
    }
}

impl SubmarinePart2 {
    fn get_result(&self) -> i32 {
        return self.horizontal * self.depth;
    }
}

impl TakesCommand for SubmarinePart1 {
    fn give_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => {
                self.horizontal += command.value;
            },
            Direction::Down => {
                self.depth += command.value;
            },
            Direction::Up => {
                self.depth -= command.value;
            },
            _ => ()
        }
    }
}

impl TakesCommand for SubmarinePart2 {
    fn give_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => {
                self.horizontal += command.value;
                self.depth += self.aim * command.value;
            },
            Direction::Down => {
                self.aim += command.value;
            },
            Direction::Up => {
                self.aim -= command.value;
            },
            _ => ()
        }
    }
}

fn main() -> Result<(), std::io::Error> {

    println!("\n\nPuzzle 2:\n");

    let mut lines: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut lines)?;

    println!("Giving commands to submarine part 1...");
    let mut submarine = SubmarinePart1::default();
    for line in &lines {
        submarine.give_command(&Command::from(line));
    }
    println!("Done!\n\nSubmarine part 1 final position: {}\n", submarine.get_result());

    println!("Giving commands to submarine part 2...");
    let mut submarine = SubmarinePart2::default();
    for line in &lines {
        submarine.give_command(&Command::from(line));
    }
    println!("Done!\n\nSubmarine part 2 final position: {}", submarine.get_result());

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