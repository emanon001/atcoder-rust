pub fn zalgo(s: &[char]) -> Vec<usize> {
    if s.len() == 0 {
        panic!("string lenght is 0");
    }
    let len = s.len();
    let mut res = vec![0; len];
    res[0] = len;
    let mut i = 1;
    let mut j = 0;
    while i < len {
        while i + j < len && s[j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < len && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zalgo() {
        assert_eq!(zalgo(&chars("ababa")), vec![5, 0, 3, 0, 1]);
        assert_eq!(zalgo(&chars("x")), vec![1]);
    }

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect::<Vec<_>>()
    }
}
