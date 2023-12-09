use std::path::Path;
use crate::problems::util;

pub fn exec(path: &str) -> String {
    let input_path = Path::new(path);
    let input_file = util::read_lines(input_path).expect("Unable to open file");
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
        // For Problem 2, try current_line.split(char::is_numeric).collect::<Vec<&str>>();
        for char in current_line.chars().rev() {
            if let Ok(_) = char.to_string().parse::<i32>() {
                current_string.push(char);
                break;
            }
        }

        sum += current_string.parse::<i32>().unwrap();
    }

    format!("{}", sum)
}

