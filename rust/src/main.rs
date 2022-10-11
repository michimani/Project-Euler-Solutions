mod solutions;
mod utils;
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
        15 => solutions::s0015::solve(),
        16 => solutions::s0016::solve(),
        31 => solutions::s0031::solve(),
        32 => solutions::s0032::solve(),
        33 => solutions::s0033::solve(),
        34 => solutions::s0034::solve(),
        35 => solutions::s0035::solve(),
        36 => solutions::s0036::solve(),
        37 => solutions::s0037::solve(),
        38 => solutions::s0038::solve(),
        39 => solutions::s0039::solve(),
        40 => solutions::s0040::solve(),
        41 => solutions::s0041::solve(),
        42 => solutions::s0042::solve(),
        43 => solutions::s0043::solve(),
        44 => solutions::s0044::solve(),
        45 => solutions::s0045::solve(),
        46 => solutions::s0046::solve(),
        47 => solutions::s0047::solve(),
        48 => solutions::s0048::solve(),
        49 => solutions::s0049::solve(),
        50 => solutions::s0050::solve(),
        52 => solutions::s0052::solve(),
        53 => solutions::s0053::solve(),
        55 => solutions::s0055::solve(),
        56 => solutions::s0056::solve(),

        _ => println!("solution {} is not found.", no),
    }
}
