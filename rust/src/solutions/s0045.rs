use proconio::input;
use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 45
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=45
pub fn solve() {
    let org = 286;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut t = get_triangle_number(n);
    let mut d = n + 1;

    let answer = loop {
        if !utils::natural::is_pentagonal_number(t as i64) {
            t += d;
            d += 1;
            continue;
        }

        if !utils::natural::is_hexagonal_number(t as i64) {
            t += d;
            d += 1;
            continue;
        }

        break t;
    };

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn get_triangle_number(n: usize) -> usize {
    return n * (n + 1) / 2;
}

#[test]
fn test_get_triangle_number() {
    assert_eq!(6, get_triangle_number(3));
    assert_eq!(15, get_triangle_number(5));
}
