use crate::utils;
use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 4
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=4
pub fn solve() {
    let org = 3;
    println!("(original: {})", org);
    input! {
      n: u32,
    }

    let start = Instant::now();

    let ten: usize = 10;
    let mut answer = 0;

    for a in (ten.pow(n - 1))..(ten.pow(n) + 1) {
        for b in (ten.pow(n - 1))..(ten.pow(n) + 1) {
            let times = a * b;
            let is_p = utils::sequence::is_palindromic(&times.to_string());
            if is_p && times > answer {
                answer = times;
            }
        }
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "\nIt took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
