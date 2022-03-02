use std::{fs};
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\nPuzzle 6:\n");

    let mut line: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut line)?;

    let list: Vec<i32> = line[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut all_schools: Vec<School> = Vec::new();
    all_schools.push(School { amount: 0, time: 0 });
    all_schools.push(School { amount: 0, time: 1 });
    all_schools.push(School { amount: 0, time: 2 });
    all_schools.push(School { amount: 0, time: 3 });
    all_schools.push(School { amount: 0, time: 4 });
    all_schools.push(School { amount: 0, time: 5 });
    all_schools.push(School { amount: 0, time: 6 });
    all_schools.push(School { amount: 0, time: 7 });
    all_schools.push(School { amount: 0, time: 8 });

    //let mut tot = 0;
    for fish in &list {
        for school in &mut all_schools {
            if *fish == school.time {
                school.amount+=1;
                break;
            }
        }
        //tot+=1;
    }
    for _ in 1..=256 {
        // have time be static
        // amounts get swapped down
        pass_time(&mut all_schools);
    }

    let mut sum = 0;

    for school in &all_schools {
        sum += school.amount;
    }
    println!("Total fish after 256 days: {}", sum);
    //generate_fish(&mut list, 10);
    //println!("Total other fish: {}", list.len());
    Ok(())
}

struct School {
    amount: i64,
    time: i32
}

fn pass_time(list: &mut Vec<School>) {
    // 8 -> 7 -> 6 -> 5 -> 4 -> 3 -> 2 -> 1 -> 0
    // ^<-<-<-<-<+<-<-<-<-<-<-<-<-<-<-<-<-<-<<<|
    
    let new_fish = list[0].amount;
    for x in 1..list.len() {
        list[x-1].amount = list[x].amount;
    }
    list[8].amount = new_fish;
    list[6].amount += new_fish;
}

/*fn generate_fish(list: &mut Vec<i32>, days: i32) -> usize {
    let mut time = Instant::now();
    for day in 1..=days {
        for lantern_fish in 0..list.len() {
            if list[lantern_fish] == 0 {
                list[lantern_fish] = 6;
                list.push(8);
            } else {
                list[lantern_fish] -= 1;
            }
        }
        if day % 10 == 0 {
            println!("Time since last 10: {}ms", time.elapsed().as_millis());
            println!("Day {}: {}", day, list.len());
            time = Instant::now();
        }
    }
    return list.len();
}*/

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