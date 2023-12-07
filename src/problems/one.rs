use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn exec(path: &str) -> i32 {
    let input_path = Path::new(path);
    let input_file = read_lines(input_path).expect("Unable to open file");
    let mut sum = 0;

    for line in input_file {
        let current_line = line.as_ref().unwrap();
        let mut current_string = String::from("");

        for char in current_line.chars() {
            if let Ok(_parsed) = char.to_string().parse::<i32>() {
                current_string.push(char);
                break;
            }
        }

        for char in current_line.chars().rev() {
            if let Ok(_) = char.to_string().parse::<i32>() {
                current_string.push(char);
                break;
            }
        }

        sum += current_string.parse::<i32>().unwrap();
    }

    println!("Problem 1: {}", sum);
    sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
