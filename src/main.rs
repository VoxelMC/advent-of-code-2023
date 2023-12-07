// use std::array;

// mod problems;
mod problems;

fn main() {
    println!("Running Advent of Code 2023!");
    let p_one_result = problems::one::exec("./inputs/1.txt");
    println!("Problem 1: {}", p_one_result);
    let top = String::from("┃");
    let middle = String::from("┠─ λ");
    let bottom = String::from("┖─ λ");
    println!("{}\n{}\n{}", top, middle, bottom);
}

// transition from green to red in colour output
// get numerical iterator from 1 to 50
// chose colours based fraction of iter / 50 * 255
//
//
// fn print_results(problems: array::IntoIter<i32, 50>) {
//     let top = String::from("┃");
//     let middle = String::from("┠");
//     let bottom = String::from("┖");
//     for problem in problems {
//         println!("Problem {}", problem );
//     }
// }
