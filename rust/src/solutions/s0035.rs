use crate::utils;
use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 35
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=35
pub fn solve() {
    let org = 1000000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 0;

    let primes = utils::prime::generate_prime_numbers(n as i32);
    for p in &primes {
        let ten: i32 = 10;
        let s_len = p.to_string().len();

        let mut is_circular = true;
        let mut p_tmp = *p;
        for _ in 1..s_len {
            let n_str = p_tmp.to_string();
            let largest_digit = (n_str.chars().nth(0).unwrap() as i32) - 48;
            if n_str.len() < s_len {
                p_tmp = p_tmp * 10;
            } else {
                p_tmp = (p_tmp - (largest_digit * ten.pow(s_len as u32 - 1))) * 10 + largest_digit;
            }
            if !primes.contains(&p_tmp) {
                is_circular = false;
                break;
            }
        }

        if is_circular {
            answer += 1;
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
