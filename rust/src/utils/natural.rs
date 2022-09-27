/// Check the number is a triangle number.
///
/// Example of triangle numbers: 1, 3, 6, 10, 15...
pub fn is_triangle_number(n: i64) -> bool {
    let mut check = n;
    let mut minus = 1;
    return loop {
        if check == 0 {
            break true;
        } else if check < 0 {
            break false;
        }
        check -= minus;
        minus += 1;
    };
}

#[test]
fn test_is_triangle_number() {
    assert_eq!(true, is_triangle_number(55));
    assert_eq!(false, is_triangle_number(8));
}

/// Check the number is a pentagonal number.
///
/// Example of pentagonal numbers: 1, 5, 12, 22, 35...
pub fn is_pentagonal_number(n: i64) -> bool {
    let mut check = n;
    let mut count = 0;
    return loop {
        if check == 0 {
            break true;
        }

        if check < 0 {
            break false;
        }

        check = check - 3 * count - 1;
        count += 1;
    };
}

#[test]
fn test_is_pentagonal_number() {
    assert_eq!(true, is_pentagonal_number(40755));
    assert_eq!(false, is_pentagonal_number(9999));
}
