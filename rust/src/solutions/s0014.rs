use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 14
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=14
pub fn solve() {
    let org = 1000000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 0;
    let mut answers_count = 0;

    for i in 1..n + 1 {
        let mut res = i;
        let mut count = 1;
        while res > 1 {
            if res % 2 == 0 {
                res = res / 2;
            } else {
                res = 3 * res + 1;
            }

            count += 1;
        }

        if count > answers_count {
            answers_count = count;
            answer = i;
        }
    }

    println!("answer is {} (with {} terms)", answer, answers_count);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
