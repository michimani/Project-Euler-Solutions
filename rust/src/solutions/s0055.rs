use std::time::Instant;

use crate::utils::big::BigNumber;
use crate::utils::sequence;

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
    fn add_reversed(n: BigNumber) -> (BigNumber, bool) {
        let rev = BigNumber::reverse(&n);
        return (n.add(&rev), true);
    }

    const LIMIT: u64 = 50;

    let mut num = BigNumber::new(&n.to_string()).unwrap();
    let mut count: u64 = 1;
    let mut is_lychrel = true;
    while count < LIMIT {
        let succeeded;
        (num, succeeded) = add_reversed(num);
        if !succeeded {
            break;
        }

        if sequence::is_palindromic(&num.to_string()) {
            is_lychrel = false;
            break;
        }
        count += 1;
    }

    return is_lychrel;
}
