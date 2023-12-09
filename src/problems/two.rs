use std::path::Path;
use regex::{Regex, Match};
use crate::problems::util;

pub fn exec(path: &str) -> String {
    let input_path = Path::new(path);
    let input_file = util::read_lines(input_path).expect("Unable to open file");
    let mut sum: i32 = 0;
    let numbers = [("one"), ("two"), ("three"), ("four"), ("five"), "six", "seven", "eight", "nine"];
    let numbers_regex = Regex::new(r"\d|on|tw|thr|four|fiv|six|seven|igh|ine");
    let mut test: Vec<i32> = vec![];

    for line in input_file {
        let current_line = line.as_ref().unwrap();
        
        // println!("{}", current_line);
        let current_line_matches = numbers_regex.clone().unwrap().find_iter(current_line).collect::<Vec<Match>>();
        // println!("{:#?}", current_line_matches);

        let first_number = if current_line_matches.get(0).unwrap().as_str().len() == 1 { 
            current_line_matches.get(0).unwrap().as_str().parse::<i32>().unwrap()
        } else {
            numbers.iter().position(|&x| x == current_line_matches.get(0).unwrap().as_str()).unwrap() as i32
        };

        let second_number = if current_line_matches.get(current_line_matches.len() - 1).unwrap().as_str().len() == 1 { 
            current_line_matches.get(current_line_matches.len() - 1).unwrap().as_str().parse::<i32>().unwrap()
        } else {
            numbers.iter().position(|&x| x == current_line_matches.get(current_line_matches.len() - 1).unwrap().as_str()).unwrap() as i32
        };

        println!("{}: {}{}", current_line, first_number, second_number);

        // println!("{}", sum);
        sum += format!("{}{}", first_number, second_number).parse::<i32>().unwrap();
        test.push(format!("{}{}", first_number, second_number).parse::<i32>().unwrap());
        // test.push(sum);
        // println!("{}", sum);
    }
    // println!("{:#?}", test);
    println!("itersum {:#?}", test.iter().sum::<i32>());

    format!("{}", sum)
}

