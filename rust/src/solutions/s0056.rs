use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 56
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=56
pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;

    for a in 1..100 {
        for b in 1..100 {
            if a % 10 == 0 {
                // probably smaller
                continue;
            }

            // a is clearly a number
            let mut big_a = utils::big::to_big(&a.to_string()).unwrap();
            let big_a_times = utils::big::to_big(&a.to_string()).unwrap();
            for _ in 0..b {
                big_a = utils::big::product_of(&big_a, &big_a_times);
            }

            let s = sum_of_digits(big_a);
            if s > answer {
                answer = s;
            }
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

fn sum_of_digits(digits: Vec<usize>) -> u64 {
    let mut sum = 0;

    for d in digits {
        sum += d as u64
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_digits() {
        assert_eq!(1, sum_of_digits(vec![1, 0, 0, 0, 0, 0, 0]));
        assert_eq!(55, sum_of_digits(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }
}
