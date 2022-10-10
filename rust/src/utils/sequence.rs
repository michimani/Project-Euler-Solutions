/// Used to compute powers of 10.
const TEN: usize = 10;

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

/// Generates pandigital numbers with the specified digits.
///
/// # Example
/// ```
/// assert_eq!([12, 21].to_vec(), generate_pandigital_numbers(&[1, 2].to_vec()));
/// assert_eq!([123, 132, 213, 231, 312, 321].to_vec(), generate_pandigital_numbers(&[1, 2, 3].to_vec()));
/// ```
pub fn generate_pandigital_numbers(digits: &Vec<usize>) -> Vec<usize> {
    let mut nums = Vec::new();

    fn gen(
        list: &mut Vec<usize>,
        base: usize,
        digits: &Vec<usize>,
        digit_number: usize,
        exclude_indices: &Vec<usize>,
    ) {
        if digit_number == 0 {
            list.push(base);
            return;
        }

        for (i, v) in digits.iter().enumerate() {
            if exclude_indices.contains(&i) {
                continue;
            }

            let new_base = base + v * TEN.pow(digit_number as u32 - 1);
            let mut exclude_clone = exclude_indices.clone();
            exclude_clone.push(i);
            gen(list, new_base, digits, digit_number - 1, &exclude_clone);
        }
    }

    gen(&mut nums, 0, digits, digits.len(), &Vec::new());

    return nums;
}

/// Two numbers have the same digits (difference order is ok).
///
/// # Example
/// ```
/// assert_eq!(false, has_same_digits(122, 12));
/// assert_eq!(true, has_same_digits(123, 312));
/// assert_eq!(true, has_same_digits(12234, 21234));
/// assert_eq!(false, has_same_digits(100099, 200198));
/// assert_eq!(true, has_same_digits(125874, 251748));
/// ```
pub fn has_same_digits(n: u64, m: u64) -> bool {
    if n == m {
        return true;
    }

    let mut n_str: Vec<char> = n.to_string().chars().collect();
    let mut m_str: Vec<char> = m.to_string().chars().collect();

    if n_str.len() != m_str.len() {
        return false;
    }

    n_str.sort_by(|c1, c2| c1.cmp(c2));
    m_str.sort_by(|c1, c2| c1.cmp(c2));

    for (i, c) in n_str.iter().enumerate() {
        if m_str[i] != *c {
            return false;
        }
    }

    return true;
}

pub fn reverse(n: u64) -> u64 {
    let mut reversed = "".to_string();
    for c in n.to_string().chars() {
        reversed = format!("{}{}", c as u64 - 48, reversed)
    }

    // reversed is clearly a number
    return reversed.parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindromic() {
        assert_eq!(true, is_palindromic("100101001"));
        assert_eq!(true, is_palindromic("1001001001"));
        assert_eq!(false, is_palindromic("1001001000"));
    }

    #[test]
    fn test_has_zero() {
        assert_eq!(true, has_zero("100"));
        assert_eq!(true, has_zero("101"));
        assert_eq!(false, has_zero("123"));
        assert_eq!(false, has_zero("1"));
    }

    #[test]
    fn test_has_duplicated_numbers() {
        assert_eq!(false, has_duplicated_numbers("8"));
        assert_eq!(true, has_duplicated_numbers("88"));
        assert_eq!(true, has_duplicated_numbers("808"));
        assert_eq!(false, has_duplicated_numbers("1234"));
    }

    #[test]
    fn test_is_pandigital() {
        assert_eq!(true, is_pandigital("32415", &[1, 2, 3, 4, 5].to_vec()));
        assert_eq!(false, is_pandigital("112345", &[1, 2, 3, 4, 5].to_vec()));
        assert_eq!(false, is_pandigital("32415", &[5, 6, 7, 8, 9].to_vec()));
    }

    #[test]
    fn test_generate_pandigital_numbers() {
        assert_eq!(
            [12, 21].to_vec(),
            generate_pandigital_numbers(&[1, 2].to_vec())
        );
        assert_eq!(
            [123, 132, 213, 231, 312, 321].to_vec(),
            generate_pandigital_numbers(&[1, 2, 3].to_vec())
        );
    }

    #[test]
    fn test_has_same_digits() {
        assert_eq!(false, has_same_digits(122, 12));
        assert_eq!(true, has_same_digits(123, 312));
        assert_eq!(true, has_same_digits(12234, 21234));
        assert_eq!(false, has_same_digits(100099, 200198));
        assert_eq!(true, has_same_digits(125874, 251748));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(321, reverse(123));
        assert_eq!(321, reverse(1230));
        assert_eq!(1001, reverse(1001));
    }
}
