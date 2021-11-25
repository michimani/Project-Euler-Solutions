use proconio::input;

// Solution for Project Euler problem 1
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=1
pub fn solve() {
  input!{
    limit: usize,
  }

  let mut sum = 0;
  let mut num = 1;
  while num < limit {
    if num % 3 == 0 || num % 5 == 0 {
      sum = sum + num;
    }

    num = num + 1;
  }

  println!("answer is {}", sum);
}