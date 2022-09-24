use crate::utils;
use std::time::Instant;

const TEN: usize = 10;

/// Solution for Project Euler problem 38
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=38
pub fn solve() {
    let start = Instant::now();

    let mut stop = false;
    let mut digits = 1;
    let mut answer_tmp = 0;
    let answer = loop {
        let base = 9 * TEN.pow(digits - 1);
        for i in 1..TEN.pow(digits - 1) {
            let num = base + i;
            let mut num_str = num.to_string();
            if utils::sequence::has_zero(&num_str)
                || utils::sequence::has_duplicated_numbers(&num_str)
            {
                continue;
            }

            if num_str.len() >= 9 {
                stop = true;
                break;
            }

            let mut times = 2;
            while num_str.len() < 9 {
                let num_str_tmp = (num * times).to_string();
                num_str = format!("{}{}", num_str, num_str_tmp);
                times += 1;
            }

            if utils::sequence::is_pandigital(&num_str, &[1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec())
                && answer_tmp < num_str.parse().unwrap()
            {
                answer_tmp = num_str.parse().unwrap();
            }
        }

        if stop {
            break answer_tmp;
        }

        digits += 1;
    };

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
