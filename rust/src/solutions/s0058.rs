use proconio::input;
use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 58
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=58
pub fn solve() {
    let org = 10;
    println!("(original: {})", org);
    input! {
      n: u64,
    }

    let start = Instant::now();
    let mut corners = 1;
    let mut corner_primes = 0;
    let mut layer: u64 = 1;
    let mut last = 0;

    let answer = loop {
        let from = last + 1;
        let to = (2 * layer - 1).pow(2);
        for (i, n_in_layer) in (from..=to).into_iter().enumerate() {
            if layer > 1 && (i + 1) as u64 % ((layer - 1) * 2) == 0 {
                corners += 1;
                if utils::prime::is_prime_number(n_in_layer as usize) {
                    corner_primes += 1;
                }
            }
            last = n_in_layer;
        }

        let mut rate: f64 = 0.0;
        if layer > 1 {
            rate = ((corner_primes as f64) / (corners as f64)) * 100.0;
        }
        if rate > 0.0 && rate < n as f64 {
            break 2 * layer - 1;
        }
        layer += 1;
    };

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
