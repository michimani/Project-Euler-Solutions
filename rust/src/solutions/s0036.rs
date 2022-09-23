use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 36
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=36
pub fn solve() {
    let org = 1000000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 0;

    for num in 1..n + 1 {
        if is_palindromic(format!("{:b}", num).as_str()) && is_palindromic(num.to_string().as_str())
        {
            answer += num;
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

/// Check the string is palindromic.
fn is_palindromic(num_str: &str) -> bool {
    let mid = num_str.len() / 2;
    let before_mid = &num_str[0..mid];
    let after_mid = &num_str[mid..num_str.len()];

    let mut is_palindromic = true;
    for i in 0..before_mid.len() {
        if before_mid[i..i + 1] != after_mid[after_mid.len() - i - 1..after_mid.len() - i] {
            is_palindromic = false;
            break;
        }
    }

    return is_palindromic;
}

#[test]
fn test_is_palindromic() {
    assert_eq!(true, is_palindromic("100101001"));
    assert_eq!(true, is_palindromic("1001001001"));
    assert_eq!(false, is_palindromic("1001001000"));
}
