use std::collections::HashMap;

// Solution for Project Euler problem 5
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=5
pub fn solve() {
  let mut answer = 1;

  let mut primes: Vec<usize> = Vec::new();
  let mut primes_count_map: HashMap<usize, usize> = HashMap::new();
  for i in 2..21 {
    let mut is_prime = true;
    for p in (&primes).into_iter() {
      if i % p == 0 {
        is_prime = false;
        break;
      }
    }

    if is_prime {
      primes.push(i);
      primes_count_map.insert(i, 1);
    } else {
      for p in (&primes).into_iter() {
        let mut divide_count = 0;
        let mut for_divide = i;
        while for_divide % p == 0 {
          for_divide = for_divide / p;
          divide_count = divide_count + 1;
        }

        if &divide_count > &primes_count_map[p] {
          primes_count_map.insert(*p, divide_count);
        }
      }
    }
  }

  for (p, c) in primes_count_map.into_iter() {
    println!("{}, {}", p, c);
    for _ in 0..c {
      answer = answer * p;
    }
  }

  println!("answer is {}", answer);
}
