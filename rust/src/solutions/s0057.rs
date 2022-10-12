use proconio::input;
use std::time::Instant;

use crate::utils;

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
    let mut prev1 = vec![1];
    let mut prev2 = vec![0];

    for _ in 1..=n {
        let mut nume = utils::big::sum_of(&prev1, &vec![0]);
        let deno = utils::big::sum_of(&utils::big::product_of(&prev1, &vec![2]), &prev2);
        nume = utils::big::sum_of(&nume, &deno);

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
