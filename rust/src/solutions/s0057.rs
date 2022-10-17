use proconio::input;
use std::time::Instant;

use crate::utils::big::BigNumber;

/// Solution for Project Euler problem 57
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=57
pub fn solve() {
    let org = 1000;
    println!("(original: {})", org);
    input! {
      n: u64,
    }

    let start = Instant::now();

    let mut answer = 0;
    let mut prev1 = BigNumber::from_vec(vec![1]);
    let mut prev2 = BigNumber::from_vec(vec![0]);
    let bn0 = BigNumber::from_vec(vec![0]);
    let bn2 = BigNumber::from_vec(vec![2]);

    for _ in 1..=n {
        let mut nume = prev1.add(&bn0);
        let deno = (prev1.mul(&bn2)).add(&prev2);
        nume = nume.add(&deno);

        if nume.len() > deno.len() {
            answer += 1;
        }

        prev2 = prev1;
        prev1 = deno;
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
