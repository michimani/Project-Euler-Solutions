/// Check the number is triangle number.
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
