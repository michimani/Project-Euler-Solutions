use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 55
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=55
pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;
    for n in 1..=10000 {
        if is_lychrel(n) {
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

fn is_lychrel(n: u64) -> bool {
    fn add_reversed(n: Vec<usize>) -> (Vec<usize>, bool) {
        let mut rev = n.clone();
        rev.reverse();
        return (utils::big::sum_of(&n, &rev), true);
    }

    const LIMIT: u64 = 50;

    let mut num = utils::big::to_big(&n.to_string()).unwrap();
    let mut count: u64 = 1;
    let mut is_lychrel = true;
    while count < LIMIT {
        let succeeded;
        (num, succeeded) = add_reversed(num);
        if !succeeded {
            break;
        }

        if utils::sequence::is_palindromic(&utils::big::to_string(&num)) {
            is_lychrel = false;
            break;
        }
        count += 1;
    }

    return is_lychrel;
}
