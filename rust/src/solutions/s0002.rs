use proconio::input;
use std::time::Instant;

// Solution for Project Euler problem 2
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=2
pub fn solve() {
    let org = 4000000;
    println!("(original: {})", org);
    input! {
      limit: usize,
    }

    let start = Instant::now();

    let mut base1 = 1;
    let mut base2 = 2;
    let mut answer = 2;

    answer = loop {
        let add = base1 + base2;

        if add % 2 == 0 {
            answer += add;
        }

        base1 = base2;
        base2 = add;

        if base2 > limit {
            break answer;
        }
    };

    println!("answer is {}", answer);
    let end = start.elapsed();
    println!(
        "\nIt took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
