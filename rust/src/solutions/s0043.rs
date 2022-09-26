use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 43
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=43
pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;

    let nums =
        utils::sequence::generate_pandigital_numbers(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec());
    let primes = utils::prime::generate_prime_numbers(17);

    for n in nums {
        let mut n_str = n.to_string();
        if n_str.len() < 10 {
            n_str = format!("0{n_str}");
        }

        let mut has_property = true;
        for i in 2..8 + 1 {
            let sub_n = &n_str[i - 1..i + 2].parse().unwrap();
            if sub_n % primes[i - 2] != 0 {
                has_property = false;
                break;
            }
        }

        if has_property {
            answer += n;
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
