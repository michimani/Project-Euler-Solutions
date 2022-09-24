/// Returns the number is prime number.
///
/// # Example
/// ```
/// assert_eq!(false, is_prime_number(1));
/// assert_eq!(true, is_prime_number(2));
/// assert_eq!(false, is_prime_number(100));
/// assert_eq!(true, is_prime_number(104729));
/// ```
pub fn is_prime_number(num: i32) -> bool {
    if num < 2 {
        return false;
    }

    let to_div = ((num as f64).powf(0.5)) as i32;

    for p in 2..num {
        if p > to_div {
            break;
        }
        if num % p == 0 {
            return false;
        }
    }

    return true;
}

#[test]
fn test_is_prime_number() {
    assert_eq!(false, is_prime_number(1));
    assert_eq!(true, is_prime_number(2));
    assert_eq!(false, is_prime_number(100));
    assert_eq!(true, is_prime_number(104729));
}

/// Generates prime numbers until limit.
///
/// # Example
/// ```
/// assert_eq!([2, 3, 5].to_vec(), generate_prime_numbers(5));
/// assert_eq!(
///     [2, 3, 5, 7, 11, 13, 17, 19].to_vec(),
///     generate_prime_numbers(20)
/// );
/// ```
pub fn generate_prime_numbers(limit: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();

    for num in 2..limit + 1 {
        let mut is_prime = true;
        let to_div = ((num as f64).powf(0.5)) as usize;

        for p in &primes {
            if *p > to_div {
                break;
            }
            if num % *p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(num);
        }
    }

    return primes;
}

#[test]
fn test_generate_prime_numbers() {
    assert_eq!([2, 3, 5].to_vec(), generate_prime_numbers(5));
    assert_eq!(
        [2, 3, 5, 7, 11, 13, 17, 19].to_vec(),
        generate_prime_numbers(20)
    );
}
