use proconio::input;
use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 48
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=48
pub fn solve() {
    let org = 1000;
    let org_limit = 10;
    println!(
        "(original: {}, {} (digit limit. 0 means unlimited))",
        org, org_limit
    );
    input! {
      n: usize,
      digit_limit: usize
    }

    let start = Instant::now();
    let mut ans_big = Vec::new();
    for num in 1..=n {
        // num is a number clearly
        let num_big_base = utils::big::to_big(&num.to_string()).unwrap();
        let mut num_big_tmp = num_big_base.clone();
        for _ in 0..num - 1 {
            num_big_tmp = utils::big::product_of(&num_big_tmp, &num_big_base);
            if digit_limit > 0 && num_big_tmp.len() > digit_limit {
                num_big_tmp = num_big_tmp[0..10].to_owned();
            }
        }
        ans_big = utils::big::sum_of(&ans_big, &num_big_tmp);
        if digit_limit > 0 && ans_big.len() > digit_limit {
            ans_big = ans_big[0..10].to_owned();
        }
    }

    let answer = utils::big::to_string(&ans_big);

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
