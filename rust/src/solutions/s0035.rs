use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 35
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=35
pub fn solve() {
    let org = 1000000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();

    let mut answer = 0;

    let primes = primes(n);
    for p in &primes {
        let rotations = rotations(*p);
        let mut is_circular = true;
        for r in rotations {
            if !primes.contains(&r) {
                is_circular = false;
                break;
            }
        }
        if is_circular {
            answer += 1;
        }
    }

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

/// Returns primes below limit.
fn primes(limit: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();

    for num in 2..limit + 1 {
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
        }
    }

    return primes;
}

#[test]
fn test_primes() {
    assert_eq!([2, 3, 5, 7].to_vec(), primes(10));
}

/// Returns rotations of the number.
///
/// # Example
/// ```
/// assert_eq!([1].to_vec(), rotations(1));
/// assert_eq!([123, 231, 312].to_vec(), rotations(123));
/// assert_eq!([9089, 899, 8990, 9908].to_vec(), rotations(9089));
/// ```
fn rotations(n: usize) -> Vec<usize> {
    let mut n_str = n.to_string();
    let s_len = n_str.len();

    let mut rotations: Vec<usize> = Vec::new();
    rotations.push(n);
    for _ in 1..s_len {
        n_str = format!("{}{}", &n_str[1..s_len], &n_str[0..1]);
        rotations.push(n_str.parse().unwrap())
    }

    return rotations;
}

#[test]
fn test_rotations() {
    assert_eq!([1].to_vec(), rotations(1));
    assert_eq!([123, 231, 312].to_vec(), rotations(123));
    assert_eq!([9089, 899, 8990, 9908].to_vec(), rotations(9089));
}
