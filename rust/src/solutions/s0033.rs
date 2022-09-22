use std::time::Instant;

// Solution for Project Euler problem 33
// Copyright michimani All rights reserved.
//
// https://projecteuler.net/problem=33
pub fn solve() {
    let start = Instant::now();

    let mut ans_x = 1;
    let mut ans_y = 1;

    // x/y
    for x in 10..100 {
        for y in 10..100 {
            if x >= y {
                continue;
            }

            let (is_curious, _, _) = curious_fraction(x, y);

            if is_curious {
                ans_x *= x;
                ans_y *= y;
            }
        }
    }

    let gcd = gcd(ans_x, ans_y);
    let answer = ans_y / gcd;

    println!("answer is {}", answer);

    let end = start.elapsed();
    println!(
        "It took {}.{:03} seconds.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn curious_fraction(x: i32, y: i32) -> (bool, i32, i32) {
    let mut x_str = x.to_string();
    let mut y_str = y.to_string();

    let mut has_same_digit = false;
    let mut same_digit = ' ';
    for c in x_str.chars() {
        if y_str.find(c) != None && c != '0' {
            has_same_digit = true;
            same_digit = c;
            break;
        }
    }

    if !has_same_digit {
        return (false, x, y);
    }

    x_str.retain(|c| c != same_digit);
    y_str.retain(|c| c != same_digit);

    if x_str.len() == 0 {
        x_str = same_digit.to_string();
    }
    if y_str.len() == 0 {
        y_str = same_digit.to_string();
    }

    let xd = x_str.parse().unwrap();
    let yd = y_str.parse().unwrap();

    if xd == 0 || yd == 0 {
        return (false, x, y);
    }

    if x as f32 / y as f32 == xd as f32 / yd as f32 {
        return (true, xd, yd);
    }

    return (false, x, y);
}

#[test]
fn test_curios_fraction() {
    assert_eq!((true, 4, 8), curious_fraction(49, 98));
    assert_eq!((false, 48, 88), curious_fraction(48, 88));
}

// Euclid
// m > n
fn gcd(m: i32, n: i32) -> i32 {
    let mut mm = m;
    let mut nn = n;

    return loop {
        let r = mm % nn;
        if r == 0 {
            break nn;
        }

        mm = nn;
        nn = r;
    };
}

#[test]
fn test_gcd() {
    assert_eq!(4, gcd(12, 8))
}
