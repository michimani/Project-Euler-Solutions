use proconio::input;

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

  let mut answer: usize = 0;
  for num in 2..n + 1 {
    if num > 5 && num % 5 == 0 {
      continue;
    }

    let mut is_prime = true;
    for d in 2..(((num as f64).powf(0.5)) as usize + 1) {
      if num % d == 0 {
        is_prime = false;
        break;
      }
    }

    if is_prime {
      answer += num;
    }
  }

  println!("answer is {}", answer);
}
