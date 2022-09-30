use proconio::input;
use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 47
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=47
pub fn solve() {
    let org = 4;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let primes_limit = (10 as usize).pow(n as u32 + 2);
    let primes = utils::prime::generate_prime_numbers(primes_limit);
    let limit = match primes.last() {
        Some(p) => p,
        None => &(0),
    };

    let mut answer = 0;
    let mut num: usize = 1;
    let mut count = 0;

    while num <= *limit {
        if has_distinct_primes(num, &primes, n) {
            count += 1;
        } else {
            count = 0;
        }

        if count == n {
            answer = num - n + 1;
            break;
        }

        num += 1;
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn has_distinct_primes(num: usize, primes: &Vec<usize>, factors_count: usize) -> bool {
    let mut num_tmp = num;
    let mut div_count = 0;
    for p in primes {
        let mut divided = false;
        while num_tmp % p == 0 {
            num_tmp = num_tmp / p;
            divided = true;
        }

        if divided {
            div_count += 1;
        }

        if div_count == factors_count || num_tmp < *p {
            break;
        }
    }

    return div_count == factors_count && num_tmp == 1;
}

#[test]
fn test_has_distinct_primes() {
    let primes = utils::prime::generate_prime_numbers(100);

    assert_eq!(false, has_distinct_primes(13, &primes, 2));
    assert_eq!(true, has_distinct_primes(14, &primes, 2));
    assert_eq!(true, has_distinct_primes(15, &primes, 2));
    assert_eq!(false, has_distinct_primes(643, &primes, 3));
    assert_eq!(true, has_distinct_primes(644, &primes, 3));
    assert_eq!(true, has_distinct_primes(645, &primes, 3));
    assert_eq!(true, has_distinct_primes(646, &primes, 3));
    assert_eq!(false, has_distinct_primes(647, &primes, 3));
}
