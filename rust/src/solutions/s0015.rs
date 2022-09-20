use proconio::input;
use std::time::Instant;

// Solution for Project Euler problem 15
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=15
//
// NOTE: (n+m)!/n!m!
//       If n = m, this is the same of 2nCn
pub fn solve() {
    let org = 20;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 1;
    let mut div = 1;
    for i in 1..n + 1 {
        answer = answer * (i + n);
        div = div * i;
        if answer % div == 0 {
            answer = answer / div;
            div = 1;
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
