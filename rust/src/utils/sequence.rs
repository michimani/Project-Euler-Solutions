/// Check the string is palindromic.
///
/// # Example
/// ```
/// assert_eq!(true, is_palindromic("1221"));
/// assert_eq!(true, is_palindromic("32123"));
/// assert_eq!(false, is_palindromic("123421"));
/// ```
pub fn is_palindromic(num_str: &str) -> bool {
    let mid = num_str.len() / 2;
    let before_mid = &num_str[0..mid];
    let after_mid = &num_str[mid..num_str.len()];

    let mut is_palindromic = true;
    for i in 0..before_mid.len() {
        if before_mid[i..i + 1] != after_mid[after_mid.len() - i - 1..after_mid.len() - i] {
            is_palindromic = false;
            break;
        }
    }

    return is_palindromic;
}

#[test]
fn test_is_palindromic() {
    assert_eq!(true, is_palindromic("100101001"));
    assert_eq!(true, is_palindromic("1001001001"));
    assert_eq!(false, is_palindromic("1001001000"));
}

/// Check the number has 0 at any of digits.
///
/// # Example
/// ```
/// assert_eq!(true, has_zero("100"));
/// assert_eq!(true, has_zero("101"));
/// assert_eq!(false, has_zero("123"));
/// assert_eq!(false, has_zero("1"));
/// ```
pub fn has_zero(num_str: &str) -> bool {
    if num_str.find("0") == None {
        return false;
    }

    return true;
}

#[test]
fn test_has_zero() {
    assert_eq!(true, has_zero("100"));
    assert_eq!(true, has_zero("101"));
    assert_eq!(false, has_zero("123"));
    assert_eq!(false, has_zero("1"));
}

/// Check the number has some duplicated numbers.
///
/// # Example
/// ```
/// assert_eq!(false, has_duplicated_numbers("8"));
/// assert_eq!(true, has_duplicated_numbers("88"));
/// assert_eq!(true, has_duplicated_numbers("808"));
/// assert_eq!(false, has_duplicated_numbers("1234"));
/// ```
pub fn has_duplicated_numbers(num_str: &str) -> bool {
    if num_str.len() < 2 {
        return false;
    }

    for (i, c) in num_str.chars().enumerate() {
        if num_str[0..i].find(c) != None
            || (i + 1 <= num_str.len() && num_str[i + 1..num_str.len()].find(c) != None)
        {
            return true;
        }
    }

    return false;
}

#[test]
fn test_has_duplicated_numbers() {
    assert_eq!(false, has_duplicated_numbers("8"));
    assert_eq!(true, has_duplicated_numbers("88"));
    assert_eq!(true, has_duplicated_numbers("808"));
    assert_eq!(false, has_duplicated_numbers("1234"));
}

/// Check the number is n-digit pandigital.
///
/// # Example
/// ```
/// assert_eq!(true, is_pandigital("32415", &[1, 2, 3, 4, 5].to_vec()));
/// assert_eq!(false, is_pandigital("112345", &[1, 2, 3, 4, 5].to_vec()));
/// assert_eq!(false, is_pandigital("32415", &[5, 6, 7, 8, 9].to_vec()));
/// ```
pub fn is_pandigital(num_str: &str, digits: &Vec<i32>) -> bool {
    if has_duplicated_numbers(num_str) {
        return false;
    }

    if num_str.len() != digits.len() {
        return false;
    }

    let mut cnt = 0;
    for d in digits {
        for ns in num_str.chars() {
            if ns as i32 - 48 - d == 0 {
                cnt += 1;
            }
        }
    }

    return cnt == digits.len();
}

#[test]
fn test_is_pandigital() {
    assert_eq!(true, is_pandigital("32415", &[1, 2, 3, 4, 5].to_vec()));
    assert_eq!(false, is_pandigital("112345", &[1, 2, 3, 4, 5].to_vec()));
    assert_eq!(false, is_pandigital("32415", &[5, 6, 7, 8, 9].to_vec()));
}
