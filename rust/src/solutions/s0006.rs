use proconio::input;
use std::time::Instant;

// Solution for Project Euler problem 6
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=6
pub fn solve() {
    let org: usize = 100;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 0;

    for i in 1..(n + 1) {
        for j in 1..(n + 1) {
            if i != j {
                answer = answer + (i * j)
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
