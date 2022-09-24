use crate::utils;
use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 36
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=36
pub fn solve() {
    let org = 1000000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 0;

    for num in 1..n + 1 {
        if utils::sequence::is_palindromic(format!("{:b}", num).as_str())
            && utils::sequence::is_palindromic(num.to_string().as_str())
        {
            answer += num;
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
