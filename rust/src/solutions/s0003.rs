use proconio::input;
use std::collections::HashMap;

// Solution for Project Euler problem 3
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=3
pub fn solve() {
  // default: n = 600851475143
  input! {
    n: usize,
  }

  let mut div_n = n;
  let mut div = 1; // divisor
  let mut primes: HashMap<usize, bool> = HashMap::new(); // prime numbers
  let mut answer_tmp = 0;
  let mut loop_count = 0;

  let answer = loop {
    div = div + 1;
    loop_count = loop_count + 1;
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

    if div_n <= 1 || div_n < div {
      break answer_tmp;
    }

    if div % 2 != 0 {
      div = div + 2;
    }
  };

  println!("answer is {}", answer);
  println!("loop count is {}", loop_count);
}
