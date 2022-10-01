use std::result::Result;

pub fn sum_of(na: &Vec<usize>, nb: &Vec<usize>) -> Vec<usize> {
    let mut ans = na.clone();
    let mut dc = nb.len();
    if &nb.len() > &ans.len() {
        dc = ans.len();
        ans = nb.clone();
    }

    let mut add_next = 0;
    for i in 0..dc {
        let mut t = na[i] + nb[i] + add_next;
        add_next = 0;
        if t > 9 {
            add_next = t / 10;
            t = t - (add_next * 10);
        }

        ans[i] = t;
    }

    if add_next > 0 {
        if ans.len() == dc {
            ans.push(add_next);
        } else {
            ans[dc] = ans[dc] + add_next;
        }
    }

    return ans;
}

#[test]
fn test_sum_of() {
    assert_eq!([4, 1].to_vec(), sum_of(&[0, 1].to_vec(), &[4].to_vec()));
    assert_eq!(
        [1, 3, 2, 1].to_vec(),
        sum_of(&[0, 1, 0, 1].to_vec(), &[1, 2, 2].to_vec())
    );
    assert_eq!(
        [1, 3, 2, 1].to_vec(),
        sum_of(&[1, 2].to_vec(), &[0, 1, 2, 1].to_vec())
    );
    assert_eq!(
        [1, 3, 2, 1].to_vec(),
        sum_of(&[0, 1, 2, 1].to_vec(), &[1, 2].to_vec())
    );
}

pub fn product_of(na: &Vec<usize>, nb: &Vec<usize>) -> Vec<usize> {
    let mut ans = Vec::new();

    for (i, a) in na.iter().enumerate() {
        let mut ans_tmp = Vec::new();
        for _ in 0..i {
            ans_tmp.push(0);
        }

        let mut add_next = 0;
        for b in nb {
            let mut t = a * b + add_next;
            add_next = 0;
            if t > 9 {
                add_next = t / 10;
                t = t - (add_next * 10);
            }

            ans_tmp.push(t);
        }

        if add_next > 0 {
            ans_tmp.push(add_next);
        }

        ans = sum_of(&ans, &ans_tmp);
    }

    return ans;
}

#[test]
fn test_product_of() {
    assert_eq!(
        [1, 2, 1].to_vec(),
        product_of(&[1, 1].to_vec(), &[1, 1].to_vec())
    );
    assert_eq!(
        [0, 5, 8, 5, 5, 6, 7, 9, 0, 2, 3, 9, 1, 2, 1].to_vec(),
        product_of(
            &[5, 6, 7, 8, 9].to_vec(),
            &[0, 9, 8, 7, 6, 5, 4, 3, 2, 1].to_vec()
        )
    );
}

pub fn to_string(big: &Vec<usize>) -> String {
    let mut s = String::new();

    let mut tmp = big.clone();
    tmp.reverse();
    for t in tmp {
        s = format!("{}{}", s, t);
    }

    return s;
}

#[test]
fn test_to_string() {
    assert_eq!("12345", to_string(&[5, 4, 3, 2, 1].to_vec()))
}

pub fn to_big(num_str: &str) -> Result<Vec<usize>, &str> {
    let mut big = Vec::new();

    for dc in num_str.chars() {
        let d = dc as i32 - 48;
        if d < 0 || d > 10 {
            return Err("parse error");
        }
        big.push(d as usize);
    }

    big.reverse();
    return Ok(big);
}

#[test]
fn test_to_big() {
    let r1 = to_big("12345");
    assert_eq!(Ok([5, 4, 3, 2, 1].to_vec()), r1);
    let r2 = to_big("12345abc");
    assert!(r2.is_err());
}
