use proconio::input;
use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 60
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=60
pub fn solve() {
    let org = 10000;
    println!(
        "(original: {} (This value is sufficient to find the answer))",
        org
    );
    input! {
      n: usize,
    }

    let start = Instant::now();

    let primes = utils::prime::generate_prime_numbers(n);
    let gen_end = start.elapsed();
    println!(
        "It took {}.{:03} seconds to generate {} primes",
        gen_end.as_secs(),
        gen_end.subsec_nanos() / 1_000_000,
        n
    );

    let mut answer_candidates = Vec::new();
    let ilimit = primes.len();
    for p1i in 1..ilimit {
        let p1 = primes[p1i];
        for p2i in p1i + 1..ilimit {
            let p2 = primes[p2i];

            if !utils::prime::is_prime_number(format!("{p1}{p2}").parse::<usize>().unwrap())
                || !utils::prime::is_prime_number(format!("{p2}{p1}").parse::<usize>().unwrap())
            {
                continue;
            }

            for p3i in p2i + 1..ilimit {
                let p3 = primes[p3i];

                if !utils::prime::is_prime_number(format!("{p1}{p3}").parse::<usize>().unwrap())
                    || !utils::prime::is_prime_number(format!("{p3}{p1}").parse::<usize>().unwrap())
                    || !utils::prime::is_prime_number(format!("{p2}{p3}").parse::<usize>().unwrap())
                    || !utils::prime::is_prime_number(format!("{p3}{p2}").parse::<usize>().unwrap())
                {
                    continue;
                }

                for p4i in p3i + 1..ilimit {
                    let p4 = primes[p4i];

                    if !utils::prime::is_prime_number(format!("{p1}{p4}").parse::<usize>().unwrap())
                        || !utils::prime::is_prime_number(
                            format!("{p4}{p1}").parse::<usize>().unwrap(),
                        )
                        || !utils::prime::is_prime_number(
                            format!("{p2}{p4}").parse::<usize>().unwrap(),
                        )
                        || !utils::prime::is_prime_number(
                            format!("{p4}{p2}").parse::<usize>().unwrap(),
                        )
                        || !utils::prime::is_prime_number(
                            format!("{p3}{p4}").parse::<usize>().unwrap(),
                        )
                        || !utils::prime::is_prime_number(
                            format!("{p4}{p3}").parse::<usize>().unwrap(),
                        )
                    {
                        continue;
                    }

                    for p5i in p4i + 1..ilimit {
                        let p5 = primes[p5i];

                        if !utils::prime::is_prime_number(
                            format!("{p1}{p5}").parse::<usize>().unwrap(),
                        ) || !utils::prime::is_prime_number(
                            format!("{p5}{p1}").parse::<usize>().unwrap(),
                        ) || !utils::prime::is_prime_number(
                            format!("{p2}{p5}").parse::<usize>().unwrap(),
                        ) || !utils::prime::is_prime_number(
                            format!("{p5}{p2}").parse::<usize>().unwrap(),
                        ) || !utils::prime::is_prime_number(
                            format!("{p3}{p5}").parse::<usize>().unwrap(),
                        ) || !utils::prime::is_prime_number(
                            format!("{p5}{p3}").parse::<usize>().unwrap(),
                        ) || !utils::prime::is_prime_number(
                            format!("{p4}{p5}").parse::<usize>().unwrap(),
                        ) || !utils::prime::is_prime_number(
                            format!("{p5}{p4}").parse::<usize>().unwrap(),
                        ) {
                            continue;
                        }

                        println!(
                            "{}, {}, {}, {}, {}: {}",
                            p1,
                            p2,
                            p3,
                            p4,
                            p5,
                            (p1 + p2 + p3 + p4 + p5)
                        );
                        answer_candidates.push(p1 + p2 + p3 + p4 + p5);
                    }
                }
            }
        }
    }

    println!("answer candidates are {:?}", answer_candidates);
    let mut answer = 0;
    for ac in answer_candidates {
        if answer == 0 {
            answer = ac;
        } else if ac < answer {
            answer = ac;
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
