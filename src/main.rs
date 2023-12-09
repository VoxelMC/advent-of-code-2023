mod problems;

fn main() {
    println!("▲ Running Advent of Code 2023!");

    let mut problems: Vec<String> = Vec::new();
    problems.push(problems::one::exec("./inputs/1.txt"));
    problems.push(problems::two::exec("./inputs/1.txt"));
    // problems.push(problems::two::exec("./inputs/debug_2.txt"));
    print_results(problems);
}

// transition from green to red in colour output
// get numerical iterator from 1 to 50
// chose colours based fraction of iter / 50 * 255
fn print_results(problems: Vec<String>) {
    let top = String::from("┃");
    let middle = String::from("┠─ λ");
    let bottom = String::from("┖─ λ");

    println!("{}", top);

    for (i, prob_result) in problems.iter().enumerate() {
        let index = i + 1;
        let prefix = if index != problems.len() { middle.clone() } else { bottom.clone() };
        println!("{} Problem {}: {}", prefix, index, prob_result );
    }
}
