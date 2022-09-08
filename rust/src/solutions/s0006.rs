use proconio::input;

// Solution for Project Euler problem 6
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=6
pub fn solve() {
  input! {
    n: usize,
  }

  let mut answer = 0;

  for i in 1..(n + 1) {
    for j in 1..(n + 1) {
      if i != j {
        answer = answer + (i * j)
      }
    }
  }

  println!("answer is {}", answer);
}
