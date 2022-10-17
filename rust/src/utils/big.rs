use std::result::Result;

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

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut s = String::new();

        let len = self.0.len();
        for i in 1..=len {
            let d = self.0[len - i];
            s = format!("{}{}", s, d);
        }

        return s;
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn part(&self, from: usize, to: usize) -> Result<Self, &str> {
        if to < from || to - 1 > self.len() {
            return Err("invalid index");
        }

        return Ok(Self::from_vec(self.0[from..to].to_vec()));
    }

    pub fn add(&self, other: &Self) -> Self {
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

    pub fn mul(&self, other: &Self) -> Self {
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

            ans = ans.add(&Self::from_vec(ans_tmp));
        }

        return ans;
    }

    #[allow(dead_code)]
    pub fn to_reverse(&mut self) {
        self.0.reverse();
    }

    pub fn reverse(target: &Self) -> Self {
        let mut rev = target.0.clone();
        rev.reverse();
        return Self(rev);
    }

    pub fn sum_og_digits(&self) -> u64 {
        let mut sum = 0;

        for d in &self.0 {
            sum += *d
        }

        return sum;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

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
        let bn3 = bn1.add(&bn2);
        assert_eq!("579", bn3.to_string());
    }

    #[test]
    fn test_big_number_mul() {
        let bn1 = BigNumber::new("123").unwrap();
        let bn2 = BigNumber::new("456").unwrap();
        let bn3 = bn1.mul(&bn2);
        assert_eq!("56088", bn3.to_string());
    }

    #[test]
    fn test_big_number_part() {
        let bn = BigNumber::new("987654321").unwrap();
        assert_eq!("54321", bn.part(0, 5).unwrap().to_string());
        assert_eq!("76", bn.part(5, 7).unwrap().to_string());
        assert!(bn.part(0, 100).is_err())
    }

    #[test]
    fn test_big_number_to_reverse() {
        let mut bn = BigNumber::new("12345").unwrap();
        assert_eq!("12345", bn.to_string());
        bn.to_reverse();
        assert_eq!("54321", bn.to_string());
    }

    #[test]
    fn test_big_number_reverse() {
        let bn = BigNumber::new("12345").unwrap();
        let bn_rev = BigNumber::reverse(&bn);
        assert_eq!("12345", bn.to_string());
        assert_eq!("54321", bn_rev.to_string());
    }

    #[test]
    fn test_big_number_sum_of_digits() {
        let bn = BigNumber::new("1234567890").unwrap();
        assert_eq!(45, bn.sum_og_digits());
    }
}
