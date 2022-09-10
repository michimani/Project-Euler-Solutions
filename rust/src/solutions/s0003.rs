use proconio::input;
use std::collections::HashMap;
use std::time::Instant;

// Solution for Project Euler problem 3
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=3
pub fn solve() {
  let org: usize = 600851475143;
  println!("(original: {})", org);
  input! {
    n: usize,
  }

  let start = Instant::now();

  let mut div_n = n;
  let mut div = 1; // divisor
  let mut primes: HashMap<usize, bool> = HashMap::new(); // prime numbers
  let mut answer_tmp = 0;

  let answer = loop {
    div = div + 1;
    if div > 2 && div % 2 == 0 {
      continue;
    }

    let mut is_prime = true;
    for p in primes.keys() {
      if div % p == 0 {
        is_prime = false;
        break;
      }
    }

    if !is_prime {
      continue;
    }

    primes.insert(div, true);

    if div_n % div == 0 {
      div_n = div_n / div;
      answer_tmp = div;
    }

    if div_n < div {
      break answer_tmp;
    }

    if div % 2 != 0 {
      div = div + 2;
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
