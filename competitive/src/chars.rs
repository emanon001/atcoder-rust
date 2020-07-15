use cargo_snippet::snippet;

#[snippet("ascii_chars")]
pub fn make_ascii_lowercase_chars() -> Vec<char> {
    let base = 0x61;
    make_chars(base..base + 26)
}

#[snippet("ascii_chars")]
pub fn make_ascii_uppercase_chars() -> Vec<char> {
    let base = 0x41;
    make_chars(base..base + 26)
}

#[snippet("ascii_chars")]
fn make_chars(range: std::ops::Range<u8>) -> Vec<char> {
    range.into_iter().map(char::from).collect::<Vec<_>>()
}

#[snippet("ascii_chars")]
pub fn rotate_ascii_lowercase_char(ch: char, n: isize) -> char {
    assert!(ch.is_ascii_lowercase());
    rotate_char(ch, n, 0x61, 26)
}

#[snippet("ascii_chars")]
pub fn rotate_ascii_uppercase_char(ch: char, n: isize) -> char {
    assert!(ch.is_ascii_uppercase());
    rotate_char(ch, n, 0x41, 26)
}

#[snippet("ascii_chars")]
fn rotate_char(ch: char, n: isize, base: u8, m: u8) -> char {
    let m = m as isize;
    let ch_pos = ch as u8 - base;
    let offset = if n >= 0 {
        (ch_pos as isize + n) % m
    } else {
        (m - (ch_pos as isize - n.abs()).abs() % m) % m
    } as u8;
    char::from(base + offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_ascii_lowercase_chars() {
        assert_eq!(
            make_ascii_lowercase_chars(),
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
            ]
        )
    }

    #[test]
    fn test_make_ascii_uppercase_chars() {
        assert_eq!(
            make_ascii_uppercase_chars(),
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
            ]
        )
    }

    #[test]
    fn test_rotate_ascii_lowercase_char() {
        let ch = 'a';
        // right
        assert_eq!(rotate_ascii_lowercase_char(ch, 0), 'a');
        assert_eq!(rotate_ascii_lowercase_char(ch, 1), 'b');
        assert_eq!(rotate_ascii_lowercase_char(ch, 25), 'z');
        assert_eq!(rotate_ascii_lowercase_char(ch, 26), 'a');
        assert_eq!(rotate_ascii_lowercase_char(ch, 27), 'b');
        assert_eq!(rotate_ascii_lowercase_char(ch, 51), 'z');
        assert_eq!(rotate_ascii_lowercase_char(ch, 52), 'a');
        // left
        assert_eq!(rotate_ascii_lowercase_char(ch, -1), 'z');
        assert_eq!(rotate_ascii_lowercase_char(ch, -25), 'b');
        assert_eq!(rotate_ascii_lowercase_char(ch, -26), 'a');
        assert_eq!(rotate_ascii_lowercase_char(ch, -27), 'z');
        assert_eq!(rotate_ascii_lowercase_char(ch, -51), 'b');
        assert_eq!(rotate_ascii_lowercase_char(ch, -52), 'a');
    }

    #[test]
    fn test_rotate_ascii_uppercase_char() {
        let ch = 'A';
        // right
        assert_eq!(rotate_ascii_uppercase_char(ch, 0), 'A');
        assert_eq!(rotate_ascii_uppercase_char(ch, 1), 'B');
        assert_eq!(rotate_ascii_uppercase_char(ch, 25), 'Z');
        assert_eq!(rotate_ascii_uppercase_char(ch, 26), 'A');
        assert_eq!(rotate_ascii_uppercase_char(ch, 27), 'B');
        assert_eq!(rotate_ascii_uppercase_char(ch, 51), 'Z');
        assert_eq!(rotate_ascii_uppercase_char(ch, 52), 'A');
        // left
        assert_eq!(rotate_ascii_uppercase_char(ch, -1), 'Z');
        assert_eq!(rotate_ascii_uppercase_char(ch, -25), 'B');
        assert_eq!(rotate_ascii_uppercase_char(ch, -26), 'A');
        assert_eq!(rotate_ascii_uppercase_char(ch, -27), 'Z');
        assert_eq!(rotate_ascii_uppercase_char(ch, -51), 'B');
        assert_eq!(rotate_ascii_uppercase_char(ch, -52), 'A');
    }
}
