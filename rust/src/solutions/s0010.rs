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

  let mut primes: Vec<usize> = Vec::new();
  let mut num = 2;
  let mut answer = 0;
  loop {
    if num > n {
      break;
    }

    let mut is_prime = true;
    for p in primes.iter() {
      if num % p == 0 {
        is_prime = false;
        break;
      }
    }

    if is_prime {
      primes.push(num);
      answer += num;
    }

    if num % 2 == 0 {
      num += 1;
    } else {
      num += 2;
    }
  }

  println!("answer is {}", answer);
}
