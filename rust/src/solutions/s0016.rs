use proconio::input;
use std::time::Instant;

// Solution for Project Euler problem 16
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=16
pub fn solve() {
    let org = 1000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut digits: Vec<i32> = Vec::new();
    digits.push(1);
    for _ in 0..n {
        let digits_tmp = digits.to_vec();
        let mut add = 0;
        for (i, d) in digits_tmp.iter().enumerate() {
            let mut new_d = *d * 2 + add;
            if new_d >= 10 {
                add = 1;
                new_d = new_d - 10;
                if digits.len() == i + 1 {
                    digits.push(1);
                }
            } else {
                add = 0;
            }
            digits[i] = new_d;
        }
    }

    let mut answer = 0;
    for d in digits {
        answer += d;
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
