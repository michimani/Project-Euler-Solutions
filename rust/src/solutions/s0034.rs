use std::time::Instant;

/// Solution for Project Euler problem 34
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=34
pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;

    // init
    let factorials = factorial_list();
    let limit = factorials[9];
    for i in 3..limit + 1 {
        let ds = digits(i);
        let mut s = 0;
        for d in ds {
            s += factorials[d as usize];
        }

        if s == i {
            answer += i;
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

/// Returns list of factorial numbers of 1 to 9.
fn factorial_list() -> [i32; 10] {
    let mut list: [i32; 10] = [1; 10];

    let mut p: i32 = 1;
    for i in 1..10 {
        p *= i;
        list[(i) as usize] = p;
    }

    return list;
}

/// Returns digits of the number.
///
/// # Example
/// ```
/// assert_eq!([1].to_vec(), digits(1));
/// assert_eq!([1, 2, 3].to_vec(), digits(123));
/// ```
fn digits(n: i32) -> Vec<i32> {
    let mut ds = Vec::new();
    for c in n.to_string().chars() {
        ds.push((c as usize - 48) as i32)
    }

    return ds;
}

#[test]
fn test_digits() {
    assert_eq!([1].to_vec(), digits(1));
    assert_eq!([1, 2, 3].to_vec(), digits(123));
}
