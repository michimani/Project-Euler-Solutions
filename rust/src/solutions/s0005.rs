use crate::utils;
use proconio::input;
use std::collections::HashMap;
use std::time::Instant;

/// Solution for Project Euler problem 5
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=5
pub fn solve() {
    let org: usize = 20;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 1;

    let mut primes: Vec<usize> = Vec::new();
    let mut primes_count_map: HashMap<usize, usize> = HashMap::new();
    for i in 2..(n + 1) {
        if utils::prime::is_prime_number(i) {
            primes.push(i);
            primes_count_map.insert(i, 1);
        } else {
            for p in (&primes).into_iter() {
                let mut divide_count = 0;
                let mut for_divide = i;
                while for_divide % p == 0 {
                    for_divide = for_divide / p;
                    divide_count = divide_count + 1;
                }

                if &divide_count > &primes_count_map[p] {
                    primes_count_map.insert(*p, divide_count);
                }
            }
        }
    }

    for (p, c) in primes_count_map.into_iter() {
        for _ in 0..c {
            answer = answer * p;
        }
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "\nIt took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
