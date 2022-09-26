use std::fs::File;
use std::io::Read;
use std::time::Instant;

use crate::utils;

/// Solution for Project Euler problem 42
///
/// Copyright michimani All rights reserved.
///
/// https://projecteuler.net/problem=42
pub fn solve() {
    let start = Instant::now();

    let mut answer = 0;

    let words = get_words();
    for w in words {
        let mut sum = 0;
        for c in w.chars() {
            sum += utils::charactor::alphabet_to_index(c);
        }

        if utils::natural::is_triangle_number(sum as i64) {
            answer += 1;
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

const WORDS_FILE: &str = "data/p042_words.txt";

fn get_words() -> Vec<String> {
    match File::open(WORDS_FILE) {
        Ok(mut r) => {
            let mut content = String::new();
            match r.read_to_string(&mut content) {
                Ok(_) => {
                    let mut words = Vec::new();

                    for s in content.split(",") {
                        let str = &s[1..s.to_string().len() - 1];
                        words.push(str.to_string());
                    }

                    return words;
                }
                Err(e) => {
                    println!("failed to read file. err={}", e);
                    return Vec::new();
                }
            };
        }
        Err(e) => {
            println!("file not found. err={}", e);
            return Vec::new();
        }
    };
}

#[test]
fn test_get_words() {
    let words = get_words();
    assert_eq!("A", words[0]);
    assert_eq!("ABOUT", words[3]);
}
