use cargo_snippet::snippet;

#[snippet("radix_conversion")]
pub fn convert_radix(value: &str, from_radix: u32, to_radix: u64) -> Vec<u64> {
    let radix10 = u64::from_str_radix(value, from_radix).expect("valid radix");
    convert_radix_from10(radix10, to_radix)
}

#[snippet("radix_conversion")]
pub fn convert_radix_from10(radix10: u64, to_radix: u64) -> Vec<u64> {
    let mut x = radix10;
    let mut ans = Vec::new();
    loop {
        ans.push(x % to_radix);
        x /= to_radix;
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
    fn test_convert_radix() {
        // 2 -> 10
        assert_eq!(convert_radix("1110", 2, 10), vec![1, 4]);
        // 10 -> 2
        assert_eq!(convert_radix("14", 10, 2), vec![1, 1, 1, 0]);
        // 2 -> 8
        assert_eq!(convert_radix("1110", 2, 8), vec![1, 6]);
        // 8 -> 2
        assert_eq!(convert_radix("16", 8, 2), vec![1, 1, 1, 0]);
    }

    #[test]
    fn test_convert_radix_to_radix2() {
        assert_eq!(convert_radix_from10(0, 2), vec![0]);
        assert_eq!(convert_radix_from10(1, 2), vec![1]);
        assert_eq!(convert_radix_from10(2, 2), vec![1, 0]);
        assert_eq!(convert_radix_from10(3, 2), vec![1, 1]);
    }

    #[test]
    fn test_convert_radix_to_radix3() {
        assert_eq!(convert_radix_from10(0, 3), vec![0]);
        assert_eq!(convert_radix_from10(1, 3), vec![1]);
        assert_eq!(convert_radix_from10(2, 3), vec![2]);
        assert_eq!(convert_radix_from10(3, 3), vec![1, 0]);
        assert_eq!(convert_radix_from10(4, 3), vec![1, 1]);
        assert_eq!(convert_radix_from10(5, 3), vec![1, 2]);
        assert_eq!(convert_radix_from10(6, 3), vec![2, 0]);
        assert_eq!(convert_radix_from10(7, 3), vec![2, 1]);
        assert_eq!(convert_radix_from10(8, 3), vec![2, 2]);
    }

    #[test]
    fn test_convert_radix_to_radix10() {
        assert_eq!(convert_radix_from10(0, 10), vec![0]);
        assert_eq!(convert_radix_from10(1, 10), vec![1]);
        assert_eq!(convert_radix_from10(10, 10), vec![1, 0]);
        assert_eq!(convert_radix_from10(19, 10), vec![1, 9]);
    }
}
