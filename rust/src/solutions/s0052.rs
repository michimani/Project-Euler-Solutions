use proconio::input;
use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 52
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=52
pub fn solve() {
    let org = 6;
    println!("(original: {})", org);
    input! {
      n: u64,
    }

    let start = Instant::now();
    let mut num = 1;

    let answer = loop {
        let mut ok = true;
        for x in 2..=n {
            let xnum = num * x;
            if !utils::sequence::has_same_digits(num, xnum) {
                ok = false;
                break;
            }
        }

        if ok {
            break num;
        }

        num += 1;
    };

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
