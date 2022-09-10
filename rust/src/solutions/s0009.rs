use proconio::input;
use std::time::Instant;

// Solution for Project Euler problem 9
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=9
pub fn solve() {
    let org = 1000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 0;

    for a in 1..(n / 3) {
        for b in (a + 1)..(n / 2) {
            let c = n - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                answer = a * b * c;
                break;
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
