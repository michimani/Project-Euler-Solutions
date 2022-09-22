use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 1
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=1
pub fn solve() {
    let org = 1000;
    println!("(original: {})", org);
    input! {
      limit: usize,
    }

    let start = Instant::now();

    let mut sum = 0;
    let mut num = 1;
    while num < limit {
        if num % 3 == 0 || num % 5 == 0 {
            sum = sum + num;
        }

        num = num + 1;
    }

    println!("answer is {}", sum);

    let end = start.elapsed();
    println!(
        "\nIt took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
