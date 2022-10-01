use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 49
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=49
pub fn solve() {
    let start = Instant::now();

    let primes = utils::prime::generate_prime_numbers(9999);
    let check_primes = primes.iter().filter(|x| **x >= 1000).collect::<Vec<_>>();
    let mut pair_list: Vec<Vec<&usize>> = Vec::new();

    for (i, p) in check_primes.iter().enumerate() {
        // p is a number clearly
        let digits = utils::big::to_big(&p.to_string()).unwrap();
        let rotations = utils::sequence::generate_pandigital_numbers(&digits);
        let mut pair = Vec::new();
        pair.push(*p);
        for j in i + 1..check_primes.len() {
            if rotations.contains(check_primes[j]) {
                pair.push(check_primes[j])
            }
        }

        if pair.len() == 3 {
            if pair[2] - pair[1] == pair[1] - pair[0] {
                pair_list.push(pair);
            }
        }
    }

    for pair in pair_list {
        print!("answer is ");
        for p in pair {
            print!("{p}")
        }
        println!("");
    }

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
