use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 12
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=12
pub fn solve() {
    let org = 500;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut num = 1;
    let mut answer: usize = 0;

    loop {
        answer = answer + num;

        let dc = divisors_count(answer);

        if dc > n {
            break;
        }

        num += 1;
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "\nIt took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn divisors_count(num: usize) -> usize {
    let mut count = 1;
    if num != 1 {
        count += 1;
    }

    for d in 2..((num as f64).powf(0.5) as usize + 1) {
        if num % d == 0 {
            count += 1;

            let a = num / d;
            if a != d {
                count += 1;
            }
        }
    }

    return count;
}

#[test]
fn test_divisors_count() {
    assert_eq!(1, divisors_count(1));
    assert_eq!(2, divisors_count(3));
    assert_eq!(4, divisors_count(21));
    assert_eq!(6, divisors_count(28));
}
