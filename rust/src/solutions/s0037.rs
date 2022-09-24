use crate::utils;
use std::time::Instant;

const TEN: i32 = 10;

/// Solution for Project Euler problem 37
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=37
pub fn solve() {
    let primes_limit = 1000000; // Took me a few tries to find it....
    let start = Instant::now();

    let mut answer = 0;
    let limit = 11;
    let mut truncatable_count = 0;

    let primes = utils::prime::generate_prime_numbers(primes_limit);
    for p in &primes {
        if *p < 10 {
            continue;
        }

        if is_truncatable(*p, &primes) {
            println!("{p}");
            answer += p;
            truncatable_count += 1;
        }

        if truncatable_count >= limit {
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

fn is_truncatable(num: usize, primes: &Vec<usize>) -> bool {
    let mut num_tmp = num;

    // right to left
    let is_truncatable = loop {
        num_tmp = num_tmp / 10;

        if num_tmp <= 0 {
            break true;
        }

        if !primes.contains(&num_tmp) {
            break false;
        }
    };

    if !is_truncatable {
        return false;
    }

    // left to right
    num_tmp = num;
    return loop {
        let digits = num_tmp.to_string().len();
        num_tmp = num_tmp
            - (num_tmp / TEN.pow(digits as u32 - 1) as usize) * TEN.pow(digits as u32 - 1) as usize;

        if num_tmp <= 0 {
            break true;
        }

        if !primes.contains(&num_tmp) {
            break false;
        }
    };
}

#[test]
fn test_is_truncatable() {
    assert_eq!(true, is_truncatable(23, &[2, 3].to_vec()))
}
