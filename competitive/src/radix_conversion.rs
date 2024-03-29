use cargo_snippet::snippet;

#[snippet("radix_conversion")]
pub fn convert_radix(radix10: u64, base: u64) -> Vec<u64> {
    let mut x = radix10;
    let mut ans = Vec::new();
    loop {
        ans.push(x % base);
        x /= base;
        if x == 0 {
            break;
        }
    }
    ans.reverse();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_radix_2() {
        assert_eq!(convert_radix(0, 2), vec![0]);
        assert_eq!(convert_radix(1, 2), vec![1]);
        assert_eq!(convert_radix(2, 2), vec![1, 0]);
        assert_eq!(convert_radix(3, 2), vec![1, 1]);
    }

    #[test]
    fn test_convert_radix_3() {
        assert_eq!(convert_radix(0, 3), vec![0]);
        assert_eq!(convert_radix(1, 3), vec![1]);
        assert_eq!(convert_radix(2, 3), vec![2]);
        assert_eq!(convert_radix(3, 3), vec![1, 0]);
        assert_eq!(convert_radix(4, 3), vec![1, 1]);
        assert_eq!(convert_radix(5, 3), vec![1, 2]);
        assert_eq!(convert_radix(6, 3), vec![2, 0]);
        assert_eq!(convert_radix(7, 3), vec![2, 1]);
        assert_eq!(convert_radix(8, 3), vec![2, 2]);
    }

    #[test]
    fn test_convert_radix_10() {
        assert_eq!(convert_radix(0, 10), vec![0]);
        assert_eq!(convert_radix(1, 10), vec![1]);
        assert_eq!(convert_radix(10, 10), vec![1, 0]);
        assert_eq!(convert_radix(19, 10), vec![1, 9]);
    }
}
