use crate::utils;
use std::time::Instant;

/// Solution for Project Euler problem 41
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=41
pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;

    let mut found = false;
    for limit in (2..10).rev() {
        let mut digits = Vec::new();
        for i in 1..limit + 1 {
            digits.push(i);
        }

        let mut p_nums = utils::sequence::generate_pandigital_numbers(&digits);
        p_nums.sort_by(|a, b| b.cmp(a));
        for pn in p_nums {
            if utils::prime::is_prime_number(pn) {
                answer = pn;
                found = true;
                break;
            }
        }

        if found {
            break;
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
