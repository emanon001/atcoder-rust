use proconio::input;
use proconio::marker::Chars;

pub fn make_ascii_lowercase_chars() -> Vec<char> {
    let base = 0x61;
    make_chars(base..base + 26)
}
pub fn make_ascii_uppercase_chars() -> Vec<char> {
    let base = 0x41;
    make_chars(base..base + 26)
}
fn make_chars(range: std::ops::Range<u8>) -> Vec<char> {
    range.into_iter().map(char::from).collect::<Vec<_>>()
}
pub fn rotate_ascii_lowercase_char(ch: char, n: isize) -> char {
    assert!(ch.is_ascii_lowercase());
    rotate_char(ch, n, 0x61, 26)
}
pub fn rotate_ascii_uppercase_char(ch: char, n: isize) -> char {
    assert!(ch.is_ascii_uppercase());
    rotate_char(ch, n, 0x41, 26)
}
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

fn main() {
    input! {
        mut s: Chars,
        k: usize
    };

    let mut rest = k;
    for i in 0..s.len() {
        if rest == 0 {
            break;
        }
        let ch = s[i];
        let pos = ch as usize - 0x61;
        if pos == 0 {
            continue;
        }
        let c = 26 - pos;
        if c <= rest {
            s[i] = 'a';
            rest -= c;
        }
    }
    if rest > 0 {
        let i = s.len() - 1;
        s[i] = rotate_ascii_lowercase_char(s[i], rest as isize);
    }
    let res = s.into_iter().collect::<String>();
    println!("{}", res);
}
