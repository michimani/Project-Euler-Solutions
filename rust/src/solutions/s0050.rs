use proconio::input;
use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 50
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=50
pub fn solve() {
    let org = 1000000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let primes = utils::prime::generate_prime_numbers(n);
    println!("There are {} primes.", primes.len());

    let mut answer = 0;
    let mut answer_count = 0;

    for (i, p) in primes.iter().enumerate() {
        for s in 0..i {
            let mut sum = 0;
            let mut count = 0;
            let mut j = s;
            while sum < *p {
                if j == i {
                    break;
                }

                sum += primes[j];
                count += 1;
                j += 1;
            }

            if sum == *p && count > answer_count {
                answer = *p;
                answer_count = count;
            }
        }
    }

    println!("answer is {} (terms: {})", answer, answer_count);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
