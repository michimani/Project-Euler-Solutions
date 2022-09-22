use proconio::input;
use std::time::Instant;

/// Solution for Project Euler problem 31
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=31
pub fn solve() {
    let org = 200;
    println!("(original: {})", org);
    input! {
      n: i32,
    }

    let start = Instant::now();

    let mut answer = 0;

    answer += ways_1(n);
    answer += ways_2(n);
    answer += ways_3(n);
    answer += ways_4(n);
    answer += ways_5(n);
    answer += ways_6(n);
    answer += ways_7(n);
    answer += ways_8(n);

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

const P1: i32 = 1;
const P2: i32 = 2;
const P5: i32 = 5;
const P10: i32 = 10;
const P20: i32 = 20;
const P50: i32 = 50;
const P100: i32 = 100; // £1
const P200: i32 = 200; // £2

const COINS: [i32; 8] = [P1, P2, P5, P10, P20, P50, P100, P200];

fn ways_1(total: i32) -> i32 {
    let mut ways = 0;

    for c in COINS {
        if total % c == 0 {
            ways += 1;
        }
    }

    return ways;
}

fn ways_2(total: i32) -> i32 {
    let mut ways = 0;

    for (i1, c1) in COINS.iter().enumerate() {
        for (i2, c2) in COINS.iter().enumerate() {
            if i1 <= i2 {
                continue;
            }

            let total_tmp = total - c1 - c2;
            let mut c1_cnt = 0;
            loop {
                let remain = total_tmp - c1 * c1_cnt;
                if remain < 0 {
                    break;
                }

                if remain == 0 || remain % c2 == 0 {
                    ways += 1;
                }

                c1_cnt += 1;
            }
        }
    }

    return ways;
}

fn ways_3(total: i32) -> i32 {
    let mut ways = 0;

    for (i1, c1) in COINS.iter().enumerate() {
        for (i2, c2) in COINS.iter().enumerate() {
            if i1 <= i2 {
                continue;
            }

            for (i3, c3) in COINS.iter().enumerate() {
                if i2 <= i3 {
                    continue;
                }

                let total_tmp = total - c1 - c2 - c3;
                let mut c1_cnt = 0;
                loop {
                    let remain = total_tmp - c1 * c1_cnt;
                    if remain < 0 {
                        break;
                    }

                    if remain == 0 {
                        ways += 1;
                        break;
                    }

                    let mut c2_cnt = 0;
                    loop {
                        let remain = remain - c2 * c2_cnt;
                        if remain < 0 {
                            break;
                        }

                        if remain == 0 || remain % c3 == 0 {
                            ways += 1;
                        }

                        c2_cnt += 1;
                    }

                    c1_cnt += 1;
                }
            }
        }
    }

    return ways;
}

fn ways_4(total: i32) -> i32 {
    let mut ways = 0;

    for (i1, c1) in COINS.iter().enumerate() {
        for (i2, c2) in COINS.iter().enumerate() {
            if i1 <= i2 {
                continue;
            }

            for (i3, c3) in COINS.iter().enumerate() {
                if i2 <= i3 {
                    continue;
                }

                for (i4, c4) in COINS.iter().enumerate() {
                    if i3 <= i4 {
                        continue;
                    }

                    let total_tmp = total - c1 - c2 - c3 - c4;
                    let mut c1_cnt = 0;
                    loop {
                        let remain = total_tmp - c1 * c1_cnt;
                        if remain < 0 {
                            break;
                        }

                        if remain == 0 {
                            ways += 1;
                            break;
                        }

                        let mut c2_cnt = 0;
                        loop {
                            let remain = remain - c2 * c2_cnt;
                            if remain < 0 {
                                break;
                            }

                            if remain == 0 {
                                ways += 1;
                                break;
                            }

                            let mut c3_cnt = 0;
                            loop {
                                let remain = remain - c3 * c3_cnt;
                                if remain < 0 {
                                    break;
                                }

                                if remain == 0 || remain % c4 == 0 {
                                    ways += 1;
                                }

                                c3_cnt += 1;
                            }

                            c2_cnt += 1;
                        }

                        c1_cnt += 1;
                    }
                }
            }
        }
    }

    return ways;
}

fn ways_5(total: i32) -> i32 {
    let mut ways = 0;

    for (i1, c1) in COINS.iter().enumerate() {
        for (i2, c2) in COINS.iter().enumerate() {
            if i1 <= i2 {
                continue;
            }

            for (i3, c3) in COINS.iter().enumerate() {
                if i2 <= i3 {
                    continue;
                }

                for (i4, c4) in COINS.iter().enumerate() {
                    if i3 <= i4 {
                        continue;
                    }

                    for (i5, c5) in COINS.iter().enumerate() {
                        if i4 <= i5 {
                            continue;
                        }

                        let total_tmp = total - c1 - c2 - c3 - c4 - c5;
                        let mut c1_cnt = 0;
                        loop {
                            let remain = total_tmp - c1 * c1_cnt;
                            if remain < 0 {
                                break;
                            }

                            if remain == 0 {
                                ways += 1;
                                break;
                            }

                            let mut c2_cnt = 0;
                            loop {
                                let remain = remain - c2 * c2_cnt;
                                if remain < 0 {
                                    break;
                                }

                                if remain == 0 {
                                    ways += 1;
                                    break;
                                }

                                let mut c3_cnt = 0;
                                loop {
                                    let remain = remain - c3 * c3_cnt;
                                    if remain < 0 {
                                        break;
                                    }

                                    if remain == 0 {
                                        ways += 1;
                                        break;
                                    }

                                    let mut c4_cnt = 0;
                                    loop {
                                        let remain = remain - c4 * c4_cnt;
                                        if remain < 0 {
                                            break;
                                        }

                                        if remain == 0 || remain % c5 == 0 {
                                            ways += 1;
                                        }

                                        c4_cnt += 1;
                                    }

                                    c3_cnt += 1;
                                }

                                c2_cnt += 1;
                            }

                            c1_cnt += 1;
                        }
                    }
                }
            }
        }
    }

    return ways;
}

fn ways_6(total: i32) -> i32 {
    let mut ways = 0;

    for (i1, c1) in COINS.iter().enumerate() {
        for (i2, c2) in COINS.iter().enumerate() {
            if i1 <= i2 {
                continue;
            }

            for (i3, c3) in COINS.iter().enumerate() {
                if i2 <= i3 {
                    continue;
                }

                for (i4, c4) in COINS.iter().enumerate() {
                    if i3 <= i4 {
                        continue;
                    }

                    for (i5, c5) in COINS.iter().enumerate() {
                        if i4 <= i5 {
                            continue;
                        }

                        for (i6, c6) in COINS.iter().enumerate() {
                            if i5 <= i6 {
                                continue;
                            }

                            let total_tmp = total - c1 - c2 - c3 - c4 - c5 - c6;
                            let mut c1_cnt = 0;
                            loop {
                                let remain = total_tmp - c1 * c1_cnt;
                                if remain < 0 {
                                    break;
                                }

                                if remain == 0 {
                                    ways += 1;
                                    break;
                                }

                                let mut c2_cnt = 0;
                                loop {
                                    let remain = remain - c2 * c2_cnt;
                                    if remain < 0 {
                                        break;
                                    }

                                    if remain == 0 {
                                        ways += 1;
                                        break;
                                    }

                                    let mut c3_cnt = 0;
                                    loop {
                                        let remain = remain - c3 * c3_cnt;
                                        if remain < 0 {
                                            break;
                                        }

                                        if remain == 0 {
                                            ways += 1;
                                            break;
                                        }

                                        let mut c4_cnt = 0;
                                        loop {
                                            let remain = remain - c4 * c4_cnt;
                                            if remain < 0 {
                                                break;
                                            }

                                            if remain == 0 {
                                                ways += 1;
                                                break;
                                            }

                                            let mut c5_cnt = 0;
                                            loop {
                                                let remain = remain - c5 * c5_cnt;
                                                if remain < 0 {
                                                    break;
                                                }

                                                if remain == 0 || remain % c6 == 0 {
                                                    ways += 1;
                                                }

                                                c5_cnt += 1;
                                            }

                                            c4_cnt += 1;
                                        }

                                        c3_cnt += 1;
                                    }

                                    c2_cnt += 1;
                                }

                                c1_cnt += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    return ways;
}

fn ways_7(total: i32) -> i32 {
    let mut ways = 0;

    for (i1, c1) in COINS.iter().enumerate() {
        for (i2, c2) in COINS.iter().enumerate() {
            if i1 <= i2 {
                continue;
            }

            for (i3, c3) in COINS.iter().enumerate() {
                if i2 <= i3 {
                    continue;
                }

                for (i4, c4) in COINS.iter().enumerate() {
                    if i3 <= i4 {
                        continue;
                    }

                    for (i5, c5) in COINS.iter().enumerate() {
                        if i4 <= i5 {
                            continue;
                        }

                        for (i6, c6) in COINS.iter().enumerate() {
                            if i5 <= i6 {
                                continue;
                            }

                            for (i7, c7) in COINS.iter().enumerate() {
                                if i6 <= i7 {
                                    continue;
                                }

                                let total_tmp = total - c1 - c2 - c3 - c4 - c5 - c6 - c7;
                                let mut c1_cnt = 0;
                                loop {
                                    let remain = total_tmp - c1 * c1_cnt;
                                    if remain < 0 {
                                        break;
                                    }

                                    if remain == 0 {
                                        ways += 1;
                                        break;
                                    }

                                    let mut c2_cnt = 0;
                                    loop {
                                        let remain = remain - c2 * c2_cnt;
                                        if remain < 0 {
                                            break;
                                        }

                                        if remain == 0 {
                                            ways += 1;
                                            break;
                                        }

                                        let mut c3_cnt = 0;
                                        loop {
                                            let remain = remain - c3 * c3_cnt;
                                            if remain < 0 {
                                                break;
                                            }

                                            if remain == 0 {
                                                ways += 1;
                                                break;
                                            }

                                            let mut c4_cnt = 0;
                                            loop {
                                                let remain = remain - c4 * c4_cnt;
                                                if remain < 0 {
                                                    break;
                                                }

                                                if remain == 0 {
                                                    ways += 1;
                                                    break;
                                                }

                                                let mut c5_cnt = 0;
                                                loop {
                                                    let remain = remain - c5 * c5_cnt;
                                                    if remain < 0 {
                                                        break;
                                                    }

                                                    if remain == 0 {
                                                        ways += 1;
                                                        break;
                                                    }

                                                    let mut c6_cnt = 0;
                                                    loop {
                                                        let remain = remain - c6 * c6_cnt;
                                                        if remain < 0 {
                                                            break;
                                                        }

                                                        if remain == 0 || remain % c7 == 0 {
                                                            ways += 1;
                                                        }

                                                        c6_cnt += 1;
                                                    }

                                                    c5_cnt += 1;
                                                }

                                                c4_cnt += 1;
                                            }

                                            c3_cnt += 1;
                                        }

                                        c2_cnt += 1;
                                    }

                                    c1_cnt += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return ways;
}

fn ways_8(total: i32) -> i32 {
    let mut ways = 0;

    for (i1, c1) in COINS.iter().enumerate() {
        for (i2, c2) in COINS.iter().enumerate() {
            if i1 <= i2 {
                continue;
            }

            for (i3, c3) in COINS.iter().enumerate() {
                if i2 <= i3 {
                    continue;
                }

                for (i4, c4) in COINS.iter().enumerate() {
                    if i3 <= i4 {
                        continue;
                    }

                    for (i5, c5) in COINS.iter().enumerate() {
                        if i4 <= i5 {
                            continue;
                        }

                        for (i6, c6) in COINS.iter().enumerate() {
                            if i5 <= i6 {
                                continue;
                            }

                            for (i7, c7) in COINS.iter().enumerate() {
                                if i6 <= i7 {
                                    continue;
                                }

                                for (i8, c8) in COINS.iter().enumerate() {
                                    if i7 <= i8 {
                                        continue;
                                    }

                                    let total_tmp = total - c1 - c2 - c3 - c4 - c5 - c6 - c7 - c8;
                                    let mut c1_cnt = 0;
                                    loop {
                                        let remain = total_tmp - c1 * c1_cnt;
                                        if remain < 0 {
                                            break;
                                        }

                                        let mut c2_cnt = 0;
                                        loop {
                                            let remain = remain - c2 * c2_cnt;
                                            if remain < 0 {
                                                break;
                                            }

                                            let mut c3_cnt = 0;
                                            loop {
                                                let remain = remain - c3 * c3_cnt;
                                                if remain < 0 {
                                                    break;
                                                }

                                                let mut c4_cnt = 0;
                                                loop {
                                                    let remain = remain - c4 * c4_cnt;
                                                    if remain < 0 {
                                                        break;
                                                    }

                                                    let mut c5_cnt = 0;
                                                    loop {
                                                        let remain = remain - c5 * c5_cnt;
                                                        if remain < 0 {
                                                            break;
                                                        }

                                                        let mut c6_cnt = 0;
                                                        loop {
                                                            let remain = remain - c6 * c6_cnt;
                                                            if remain < 0 {
                                                                break;
                                                            }

                                                            let mut c7_cnt = 0;
                                                            loop {
                                                                let remain = remain - c7 * c7_cnt;
                                                                if remain < 0 {
                                                                    break;
                                                                }

                                                                if remain % c8 == 0 {
                                                                    ways += 1;
                                                                }

                                                                c7_cnt += 1;
                                                            }

                                                            c6_cnt += 1;
                                                        }

                                                        c5_cnt += 1;
                                                    }

                                                    c4_cnt += 1;
                                                }

                                                c3_cnt += 1;
                                            }

                                            c2_cnt += 1;
                                        }

                                        c1_cnt += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return ways;
}

#[test]
fn test_ways_n() {
    assert_eq!(0, ways_8(200));
    assert_eq!(8, ways_1(200));
}
