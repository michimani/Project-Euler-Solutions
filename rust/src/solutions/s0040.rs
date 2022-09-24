use proconio::input;
use std::time::Instant;

const TEN: usize = 10;

/// Solution for Project Euler problem 40
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=40
pub fn solve() {
    let org = 1000000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 1;

    let mut num_str = "".to_string();
    let mut i = 1;
    while num_str.len() <= n {
        num_str = format!("{}{}", num_str, i);
        i += 1
    }

    let digits = n.to_string().len();
    for d in 0..digits {
        let co = num_str.chars().nth(TEN.pow(d as u32) - 1);
        if co != None {
            answer *= co.unwrap() as usize - 48;
        }
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
