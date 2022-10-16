use std::ops::{Add, Mul};
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

pub fn to_string(big: &Vec<usize>) -> String {
    let mut s = String::new();

    let mut tmp = big.clone();
    tmp.reverse();
    for t in tmp {
        s = format!("{}{}", s, t);
    }

    return s;
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

#[derive(Debug, PartialEq)]
pub struct BigNumber(Vec<u64>);

impl BigNumber {
    pub fn new(num_str: &str) -> Result<Self, &str> {
        let mut big = Vec::new();

        for dc in num_str.chars() {
            let d = dc as i32 - 48;
            if d < 0 || d > 10 {
                return Err("parse error");
            }
            big.push(d as u64);
        }

        big.reverse();
        return Ok(Self(big));
    }

    pub fn from_vec(nv: Vec<u64>) -> Self {
        return Self(nv);
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        let len = self.0.len();
        for i in 1..=len {
            let d = self.0[len - i];
            s = format!("{}{}", s, d);
        }

        return s;
    }
}

impl Add for BigNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut ans = self.0.clone();
        let mut digit = other.0.len();
        if other.0.len() > ans.len() {
            digit = ans.len();
            ans = other.0.clone();
        }

        let mut add_next = 0;
        for i in 0..digit {
            let mut t = self.0[i] + other.0[i] + add_next;
            add_next = 0;
            if t > 9 {
                add_next = t / 10;
                t = t - (add_next * 10);
            }

            ans[i] = t;
        }

        if add_next > 0 {
            if ans.len() == digit {
                ans.push(add_next);
            } else {
                ans[digit] = ans[digit] + add_next;
            }
        }

        return Self(ans);
    }
}

impl Mul for BigNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut ans = Self::new("0").unwrap();

        for (i, a) in self.0.iter().enumerate() {
            let mut ans_tmp = Vec::new();
            for _ in 0..i {
                ans_tmp.push(0);
            }

            let mut add_next = 0;
            for b in &other.0 {
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

            ans = ans + Self::from_vec(ans_tmp);
        }

        return ans;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

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

    #[test]
    fn test_to_string() {
        assert_eq!("12345", to_string(&[5, 4, 3, 2, 1].to_vec()))
    }

    #[test]
    fn test_to_big() {
        let r1 = to_big("12345");
        assert_eq!(Ok([5, 4, 3, 2, 1].to_vec()), r1);
        let r2 = to_big("12345abc");
        assert!(r2.is_err());
    }

    #[test]
    fn test_big_number_new() {
        let bn1 = BigNumber::new("123");
        assert_eq!(Ok(BigNumber::from_vec(vec![3, 2, 1])), bn1);
        let bn2 = BigNumber::new("123abc");
        assert!(bn2.is_err());
    }

    #[test]
    fn test_big_number_from_vec() {
        let bn1 = BigNumber::from_vec(vec![1, 2, 3, 4]);
        assert_eq!("4321", bn1.to_string());
    }

    #[test]
    fn test_big_number_to_string() {
        let bn = BigNumber::new("12345").unwrap();
        assert_eq!("12345", bn.to_string());
    }

    #[test]
    fn test_big_number_add() {
        let bn1 = BigNumber::new("123").unwrap();
        let bn2 = BigNumber::new("456").unwrap();
        let bn3 = bn1 + bn2;
        assert_eq!("579", bn3.to_string());
    }

    #[test]
    fn test_big_number_mul() {
        let bn1 = BigNumber::new("123").unwrap();
        let bn2 = BigNumber::new("456").unwrap();
        let bn3 = bn1 * bn2;
        assert_eq!("56088", bn3.to_string());
    }
}
