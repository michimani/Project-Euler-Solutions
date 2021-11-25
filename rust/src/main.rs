mod solutions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let solution_no_str = &args[1];
    let solution_no: usize = solution_no_str.parse().unwrap();
    solve(solution_no);
}

fn solve(no: usize) {
    if no == 1 {
        solutions::s0001::solve();
    } else {
        println!("solution {} is not found.", no)
    }
}