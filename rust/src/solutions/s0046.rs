use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 46
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=46
pub fn solve() {
    let start = Instant::now();

    let mut last_index: i64 = -1;
    let mut loop_cnt = 0;
    let mut last_largest = 0;
    let mut found = false;

    let answer = loop {
        let limit = (10 as usize).pow(loop_cnt + 3);
        let odds = (3..limit).filter(|x| *x % 2 != 0).collect::<Vec<usize>>();
        let mut odd_composites = Vec::new();
        for (i1, o1) in odds.iter().enumerate() {
            for i2 in i1..odds.len() {
                if i1 as i64 <= last_index && i2 as i64 <= last_index {
                    continue;
                }

                let o2 = odds[i2];

                let oc = o1 * o2;
                if last_largest > oc {
                    continue;
                }
                odd_composites.push(oc);
            }
        }

        last_index = odds.len() as i64 - 1;

        odd_composites.sort_by(|a, b| a.cmp(b));

        let mut ans = 0;
        for oc in &odd_composites {
            if !is_goldbach_number(*oc) {
                ans = *oc;
                found = true;
                break;
            }
            last_largest = *oc;
        }

        if found {
            break ans;
        }

        loop_cnt += 1;
    };

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn is_goldbach_number(n: usize) -> bool {
    let mut sqrt: usize = 1;
    return loop {
        let minus = 2 * sqrt.pow(2);
        if n <= minus {
            break false;
        }
        let remain = n - minus;

        if utils::prime::is_prime_number(remain) {
            break true;
        }

        sqrt += 1;
    };
}

#[test]
fn test_is_goldbach_number() {
    assert_eq!(true, is_goldbach_number(9));
    assert_eq!(true, is_goldbach_number(77));
    assert_eq!(true, is_goldbach_number(121));
}
