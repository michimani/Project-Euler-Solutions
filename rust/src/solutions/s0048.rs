use proconio::input;
use std::time::Instant;

use crate::utils::big::BigNumber;

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
    let bn0 = BigNumber::from_vec(vec![0]);
    let mut answer = BigNumber::from_vec(vec![0]);
    for num in 1..=n {
        // num is a number clearly
        let bn_base = BigNumber::new(&num.to_string()).unwrap();
        let mut bn_base_tmp = bn_base.add(&bn0);
        for _ in 0..num - 1 {
            bn_base_tmp = bn_base_tmp.mul(&bn_base);
            if digit_limit > 0 && bn_base_tmp.len() > digit_limit {
                bn_base_tmp = bn_base_tmp.part(0, 10).unwrap();
            }
        }
        answer = answer.add(&bn_base_tmp);
        if digit_limit > 0 && answer.len() > digit_limit {
            answer = answer.part(0, 10).unwrap();
        }
    }

    println!("answer is {}", answer.to_string());

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
