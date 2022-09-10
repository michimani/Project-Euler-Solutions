use proconio::input;
use std::time::Instant;

// Solution for Project Euler problem 7
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=7
pub fn solve() {
  let org = 10001;
  println!("(original: {})", org);
  input! {
    n: usize,
  }

  let start = Instant::now();

  let mut primes: Vec<usize> = Vec::new();
  let mut num = 2;

  let answer = loop {
    let mut is_prime = true;
    for p in primes.iter() {
      if num % p == 0 {
        is_prime = false;
        break;
      }
    }

    if is_prime {
      primes.push(num);
    }

    if primes.len() == n {
      break num;
    }

    if num % 2 == 0 {
      num += 1;
    } else {
      num += 2;
    }
  };

  println!("answer is {}", answer);

  let end = start.elapsed();
  println!(
    "\nIt took {}.{:03} seconds.",
    end.as_secs(),
    end.subsec_nanos() / 1_000_000
  );
}
