use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 44
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=44
pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;
    let mut answer_pj = 0;
    let mut answer_pk = 0;
    let mut found = false;
    let mut loop_cnt = 0;
    let mut p_nums = Vec::new();
    const PER_COUNT: usize = 500;

    while !found {
        let mut p_nums_tmp = gen_pentagonal_numbers(PER_COUNT * loop_cnt + 1, PER_COUNT);
        p_nums.append(&mut p_nums_tmp);

        for (j, pj) in p_nums.iter().enumerate() {
            for k in j + 1..p_nums.len() {
                // already checked
                if j <= PER_COUNT * loop_cnt && k <= PER_COUNT * loop_cnt {
                    continue;
                }

                let pk = p_nums[k];

                if !utils::natural::is_pentagonal_number((pj + pk) as i64) {
                    continue;
                }

                let diff = pk - pj;
                if !utils::natural::is_pentagonal_number(diff as i64) {
                    continue;
                }

                answer = diff;
                answer_pj = *pj;
                answer_pk = pk;
                found = true;
                break;
            }

            if found {
                break;
            }
        }
        if found {
            break;
        }

        loop_cnt += 1;
    }

    println!("answer is {} ({}, {})", answer, answer_pj, answer_pk);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

/// Generate pentagonal numbers.
///
/// start: index of pentagonal number.
fn gen_pentagonal_numbers(start: usize, count: usize) -> Vec<usize> {
    let mut nums = Vec::new();

    for i in start..start + count + 1 {
        nums.push((i * (3 * i - 1)) / 2);
    }

    return nums;
}
