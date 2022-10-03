/// Returns index of alphabet.

/// a is 1, b is 2 ... z is 26 (ignore upper case/lower case).
/// if it is not alphabet, return c.
///
/// # Example
pub fn alphabet_to_index(c: char) -> usize {
    let u = c as usize;

    // a to z
    if u >= 97 && u <= 122 {
        return u - 96;
    }

    // A to Z
    if u >= 65 && u < 90 {
        return u - 64;
    }

    // else
    return u;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet_to_index() {
        assert_eq!(11, alphabet_to_index('K'));
        assert_eq!(25, alphabet_to_index('y'));
        assert_eq!(33, alphabet_to_index('!'));
    }
}
