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
