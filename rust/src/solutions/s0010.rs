use proconio::input;
use std::time::Instant;

// Solution for Project Euler problem 10
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=10
pub fn solve() {
  let org = 2000000;
  println!("(original: {})", org);
  input! {
    n: usize,
  }

  let start = Instant::now();

  let mut answer: usize = 0;
  let mut primes: Vec<usize> = Vec::new();

  for num in 2..n + 1 {
    let mut is_prime = true;
    let to_div = ((num as f64).powf(0.5)) as usize;

    for p in &primes {
      if *p > to_div {
        break;
      }
      if num % *p == 0 {
        is_prime = false;
        break;
      }
    }

    if is_prime {
      primes.push(num);
      answer += num;
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
