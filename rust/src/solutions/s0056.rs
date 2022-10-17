use std::time::Instant;

use crate::utils::big::BigNumber;

/// Solution for Project Euler problem 56
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=56
pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;

    for a in 1..100 {
        for b in 1..100 {
            if a % 10 == 0 {
                // probably smaller
                continue;
            }

            // a is clearly a number
            let mut big_a = BigNumber::new(&a.to_string()).unwrap();
            let big_a_times = BigNumber::new(&a.to_string()).unwrap();
            for _ in 0..b {
                big_a = big_a.mul(&big_a_times);
            }

            let s = big_a.sum_og_digits();
            if s > answer {
                answer = s;
            }
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
