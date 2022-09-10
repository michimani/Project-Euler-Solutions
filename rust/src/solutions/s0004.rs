use proconio::input;
use std::time::Instant;

// Solution for Project Euler problem 4
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=4
pub fn solve() {
    let org = 3;
    println!("(original: {})", org);
    input! {
      n: u32,
    }

    let start = Instant::now();

    let ten: usize = 10;
    let mut answer = 0;

    for a in (ten.pow(n - 1))..(ten.pow(n) + 1) {
        for b in (ten.pow(n - 1))..(ten.pow(n) + 1) {
            let times = a * b;
            let is_p = is_palindromic(times);
            if is_p && times > answer {
                answer = times;
            }
        }
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "\nIt took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn is_palindromic(num: usize) -> bool {
    let num_str = num.to_string();
    let mut chars: Vec<char> = Vec::new();

    for i in (0..num_str.len()).rev() {
        match num_str.chars().nth(i) {
            Some(c) => chars.push(c),
            _ => {}
        }
    }

    let reversed = String::from_iter(chars.into_iter());

    return num_str == reversed;
}

#[test]
fn test_is_palindromic() {
    assert_eq!(is_palindromic(1234), false);
    assert_eq!(is_palindromic(1221), true);
    assert_eq!(is_palindromic(121), true);
    assert_eq!(is_palindromic(11), true);
    assert_eq!(is_palindromic(1), true);
}
