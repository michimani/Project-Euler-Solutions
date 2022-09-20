mod solutions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let solution_no_str = &args[1];
    let solution_no: usize = solution_no_str.parse().unwrap();
    solve(solution_no);
}

fn solve(no: usize) {
    match no {
        1 => solutions::s0001::solve(),
        2 => solutions::s0002::solve(),
        3 => solutions::s0003::solve(),
        4 => solutions::s0004::solve(),
        5 => solutions::s0005::solve(),
        6 => solutions::s0006::solve(),
        7 => solutions::s0007::solve(),
        8 => solutions::s0008::solve(),
        9 => solutions::s0009::solve(),
        10 => solutions::s0010::solve(),
        12 => solutions::s0012::solve(),
        13 => solutions::s0013::solve(),
        14 => solutions::s0014::solve(),

        _ => println!("solution {} is not found.", no),
    }
}
