use crate::utils;
use std::time::Instant;

/// Solution for Project Euler problem 32
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=32
static TEN: u32 = 10;

pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;
    let mut checked: Vec<u32> = Vec::new();

    // multiplicand digits loop: 1 to 8
    for m1d in 1..9 {
        // multiplier digits loop: 1 to 9 - m1
        for m2d in 1..9 - m1d + 1 {
            if (TEN.pow(m1d) - 1) * (TEN.pow(m2d) - 1) < TEN.pow(9 - m1d - m2d)
                || TEN.pow(m1d - 1) * TEN.pow(m2d - 1) >= TEN.pow(9 - m1d - m2d)
            {
                continue;
            }

            for m1 in TEN.pow(m1d - 1)..TEN.pow(m1d) {
                if utils::sequence::has_zero(m1.to_string().as_str()) {
                    continue;
                }

                let m2l = gen_multiplier_list(m2d, m1);
                for m2 in m2l {
                    let (p, b) = is_mul_mul(m1, m2);
                    if b {
                        if !checked.contains(&p) {
                            answer += p;
                            checked.push(p);
                        }
                    }
                }
            }
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

// gen_multiplier_list generates multiplier
fn gen_multiplier_list(digit: u32, exclude: u32) -> Vec<u32> {
    let mut ml = Vec::new();

    for i in TEN.pow(digit as u32 - 1)..TEN.pow(digit as u32) {
        // exclude number contains 0
        if utils::sequence::has_zero(i.to_string().as_str()) {
            continue;
        }

        let mut is_multiplier = true;
        for ec in exclude.to_string().chars() {
            if i.to_string().find(ec) != None {
                is_multiplier = false;
                break;
            }
        }

        if !is_multiplier {
            continue;
        }

        is_multiplier = !utils::sequence::has_duplicated_numbers(&i.to_string());
        if is_multiplier {
            ml.push(i);
        }
    }

    return ml;
}

#[test]
fn test_gen_multiplier_list() {
    let l1 = gen_multiplier_list(1, 123);
    assert_eq!([4, 5, 6, 7, 8, 9].to_vec(), l1);

    let l2 = gen_multiplier_list(2, 1234567);
    assert_eq!([89, 98].to_vec(), l2);
}

// is_mul_mul calculate a * b,
// and check they are multiplicand/multiplier/product
// can be written as a 1 through 9 pandigital.
fn is_mul_mul(a: u32, b: u32) -> (u32, bool) {
    let p = a * b;

    let check_str = format!("{}{}{}", a, b, p);

    return (
        p,
        utils::sequence::is_pandigital(&check_str, &[1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec()),
    );
}

#[test]
fn test_is_mul_mul() {
    let (t1, b1) = is_mul_mul(39, 186);
    assert_eq!(7254, t1);
    assert!(b1);

    let (t2, b2) = is_mul_mul(10, 23);
    assert_eq!(230, t2);
    assert!(!b2);

    let (t3, b3) = is_mul_mul(888, 0);
    assert_eq!(0, t3);
    assert!(!b3);
}
