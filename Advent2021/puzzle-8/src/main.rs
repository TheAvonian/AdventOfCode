use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\nPuzzle 8:\n");

    let mut lines: Vec<String> = Vec::new();
    load_file_into_vec("./src/input.txt", &mut lines)?;
    
    // unique nums: 7, 4, 3, 2
    let mut count = 0;
    for line in &lines {
        let vals = line.split('|');
        let right = vals.collect::<Vec<&str>>()[1].split(' ');
        let right =  right.collect::<Vec<&str>>();
        for val in right {
            match val.len() {
                7 | 4 | 3 | 2 => count += 1,
                _ => ()
            }
        }
    }
    println!("{}", count);

    
    let mut total_sum = 0;
    for line in &lines {
        let vals = line.split(" | ").map(|x| x.to_string()).collect::<Vec<String>>();

        let left_side = vals[0].to_string();
        let right_side = vals[1].split(' ').collect::<Vec<&str>>();
        
        let pattern = NumberPattern::from(left_side);
        let mut sum = "".to_string();
        for unknown in right_side {
            sum.push(pattern.to_number(unknown.to_string())); 
        }
        total_sum += sum.parse::<i32>()?;
        pattern.print_pattern();
        println!();
    }
    println!("Total value: {}", total_sum);
    Ok(())
}

struct NumberPattern {
    top: char,
    top_right: char,
    middle: char,
    top_left: char,
    bottom_right: char,
    bottom: char,
    bottom_left: char
}

impl NumberPattern {
    fn print_pattern(&self) {
        println!(" {}{}{}{} ", self.top, self.top, self.top, self.top);
        println!("{}    {}", self.top_left, self.top_right);
        println!("{}    {}", self.top_left, self.top_right);
        println!(" {}{}{}{} ", self.middle, self.middle, self.middle, self.middle);
        println!("{}    {}", self.bottom_left, self.bottom_right);
        println!("{}    {}", self.bottom_left, self.bottom_right);
        println!(" {}{}{}{} ", self.bottom, self.bottom, self.bottom, self.bottom);
    }
    fn to_number(&self, unknown: String) -> char {
        match unknown.len() {
            7 => {
                return '8';
            },
            3 => {
                return '7';
            },
            4 => {
                return '4';
            },
            2 => {
                return '1';
            },
            _ => ()
        };
        // 1, 4, 7, 8 done
        if unknown.contains(self.top) && unknown.contains(self.top_left) && unknown.contains(self.top_right) && unknown.contains(self.bottom_right) && unknown.contains(self.bottom_left) && unknown.contains(self.bottom) && !unknown.contains(self.middle) {
            return '0';
        }
        // 0, 1, 4, 7, 8 done
        if unknown.contains(self.top) && !unknown.contains(self.top_left) && unknown.contains(self.top_right) && !unknown.contains(self.bottom_right) && unknown.contains(self.bottom_left) && unknown.contains(self.bottom) && unknown.contains(self.middle) {
            return '2';
        }
        if unknown.contains(self.top) && !unknown.contains(self.top_left) && unknown.contains(self.top_right) && unknown.contains(self.bottom_right) && !unknown.contains(self.bottom_left) && unknown.contains(self.bottom) && unknown.contains(self.middle) {
            return '3';
        }
        if unknown.contains(self.top) && unknown.contains(self.top_left) && !unknown.contains(self.top_right) && unknown.contains(self.bottom_right) && !unknown.contains(self.bottom_left) && unknown.contains(self.bottom) && unknown.contains(self.middle) {
            return '5';
        }
        // 0, 1, 2, 3, 4, 5, 7, 8 done
        if unknown.contains(self.top) && unknown.contains(self.top_left) && !unknown.contains(self.top_right) && unknown.contains(self.bottom_right) && unknown.contains(self.bottom_left) && unknown.contains(self.bottom) && unknown.contains(self.middle) {
            return '6';
        }
        if unknown.contains(self.top) && unknown.contains(self.top_left) && unknown.contains(self.top_right) && unknown.contains(self.bottom_right) && !unknown.contains(self.bottom_left) && unknown.contains(self.bottom) && unknown.contains(self.middle) {
            return '9';
        }
        return '0';
    }
    fn from(line: String) -> Self {
        let mut vals = line.split(' ').collect::<Vec<&str>>();
        let mut eight = "";
        let mut seven = "";
        let mut four = "";
        let mut one = "";
        let mut indices = Vec::new();
        for (i,val) in vals.iter().enumerate() {
            match val.len() {
                7 => {
                    eight = *val;
                    indices.push(i);
                },
                3 => {
                    seven = *val;
                    indices.push(i);
                },
                4 => {
                    four = *val;
                    indices.push(i);
                },
                2 => {
                    one = *val;
                    indices.push(i);
                },
                _ => ()
            };
        }
        let right_side = one.to_string();
        let mut top = ' ';
        for x in seven.chars() {
            if !one.contains(x) {
                top = x;
            }
        }
        for x in (0..indices.len()).rev() {
            vals.remove(indices[x]);
            indices.remove(x);
        }
        let mut two_five_six = Vec::new();
        for (i,x) in vals.iter().enumerate() {
            if !x.contains(right_side.as_bytes()[0] as char) || !x.contains(right_side.as_bytes()[1] as char) {
                two_five_six.push(*x);
                indices.push(i);
                //println!("YOYO");
            }
        }
        for x in (0..indices.len()).rev() {
            vals.remove(indices[x]);
            indices.remove(x);
        }
        let mut bottom = ' ';
        for (i,x) in vals.iter().enumerate() {
            if x.contains(four.as_bytes()[0] as char) && x.contains(four.as_bytes()[1] as char) && x.contains(four.as_bytes()[2] as char) && x.contains(four.as_bytes()[3] as char) && x.to_string() != eight.to_string() {
                for cha in x.chars() {
                    if !four.contains(cha) && cha != top {
                        bottom = cha;
                        indices.push(i);
                        break;
                    }
                }
            }
        }
        vals.remove(indices[0]);
        indices.remove(0);
        // 9, 8, 7, 4, 1 done ind
        // 2, 5, 6 together
        // 0, 3 hanging

        let mut six = "";
        // top, bottom done
        for (i,x) in two_five_six.iter().enumerate() {
            if x.len() == 6 {
                indices.push(i);
                six = *x;
            }
        }
        // errors here
        two_five_six.remove(indices[0]);
        indices.remove(0);

        let mut zero = "";
        for (i, x) in vals.iter().enumerate() {
            if x.len() == 6 {
                zero = *x;
                //println!("AH");
            }

            indices.push(i);
        }
        vals.remove(indices[0]);
        indices.remove(0);
        //errors here

        // 0, 1, 3, 4, 6, 7, 8, 9 found
        
        // only two and five
        let mut five = "";
        for (i,x) in two_five_six.iter().enumerate() {
            let mut sum = 0;
            for cha in x.chars() {
                if four.contains(cha) {
                    sum+=1;
                }
            }
            if sum == 3 {
                five = *x;
            }
            indices.push(i);
        }
        two_five_six.remove(indices[0]);
        indices.remove(0);
        two_five_six.remove(indices[0]);
        indices.remove(0);


        // all numbers found

        //bottom left
        let mut bottom_left = ' ';
        for x in six.chars() {
            if !five.contains(x) {
                bottom_left = x;
                break;
            }
        }

        //top right
        let mut top_right = ' ';
        for x in eight.chars() {
            if !six.contains(x) {
                top_right = x;
                break;
            }
        }
        // top, bottom, bottom left, top right

        let mut bottom_right = ' ';
        for x in one.chars() {
            if x != top_right {
                bottom_right = x;
            }
        }

        // top left
        let mut top_left = ' ';
        for x in zero.chars() {
            if x != top && x != bottom && x != bottom_left && x != bottom_right && x != top_right {
                top_left = x
            }
        }
        let mut middle = ' ';
        for x in four.chars() {
            if x != bottom_right && x != top_right && x != top_left {
                middle = x;
            }
        }
        Self { top, bottom, bottom_left, top_right, bottom_right, top_left, middle }
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