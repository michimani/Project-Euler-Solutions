use std::time::Instant;

/// Solution for Project Euler problem 53
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=53
pub fn solve() {
    let start = Instant::now();

    let limit = 1000000;
    let mut answer = 0;
    for n in 1..=100 {
        let mid_even = n % 2 == 0;
        let to = n / 2;
        for r in 0..=to {
            let c = limited_combinations(n, r, limit);
            if c > limit {
                answer += 2;
                if mid_even && r == to {
                    answer -= 1;
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

fn limited_combinations(n: u64, r: u64, limit: u64) -> u64 {
    if n < r {
        return 0;
    }

    let mut divs: Vec<u64> = (1..=r).collect();
    let mut divs2: Vec<u64> = (1..=(n - r)).collect();
    divs.append(&mut divs2);

    let mut used: Vec<usize> = Vec::new();

    let mut c = 1;
    for ni in 1..=n {
        c *= ni;

        for (i, d) in divs.iter().enumerate() {
            if used.contains(&i) {
                continue;
            }

            if c % d == 0 {
                c /= d;
                used.push(i);
            }
        }

        if c >= limit {
            let mut c_tmp = c;
            for (i, d) in divs.iter().enumerate() {
                if used.contains(&i) {
                    continue;
                }
                c_tmp /= d;
            }

            if c_tmp > limit {
                return c;
            }

            if c_tmp == 0 {
                c_tmp = 1;
            }

            for nti in ni..=n {
                c_tmp *= nti;
                if c_tmp > limit {
                    return c;
                }
            }
        }
    }

    return c;
}
