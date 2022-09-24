use proconio::input;
use std::collections::HashMap;
use std::time::Instant;

/// Solution for Project Euler problem 39
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=39
pub fn solve() {
    let org = 1000;
    println!("(original: {})", org);
    input! {
      n: usize,
    }

    let start = Instant::now();
    let mut p_solutions_map: HashMap<usize, usize> = HashMap::new();

    let mut answer = 0;

    for a in 1..n / 3 {
        for b in a + 1..n / 2 {
            for c in b + 1..n / 2 + 1 {
                let p = a + b + c;
                if a * a + b * b == c * c {
                    let mut cnt = 1;
                    let before = &p_solutions_map.get(&p);
                    if *before != None {
                        cnt += before.unwrap();
                    }

                    p_solutions_map.insert(p, cnt);
                }
            }
        }
    }

    let mut cnt_check = 0;
    for (p, cnt) in p_solutions_map {
        if cnt_check < cnt {
            answer = p;
            cnt_check = cnt;
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
