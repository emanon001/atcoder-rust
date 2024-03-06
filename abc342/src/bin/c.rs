#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn make_ascii_lowercase_chars() -> Vec<char> {
    let base = 0x61;
    make_chars(base..base + 26)
}
fn make_chars(range: std::ops::Range<u8>) -> Vec<char> {
    range.into_iter().map(char::from).collect::<Vec<_>>()
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        _: usize,
        S: Chars,
        Q: usize,
        CD: [(char, char); Q],
    };

    let mut char_map = make_ascii_lowercase_chars()
        .into_iter()
        .map(|ch| (ch, ch))
        .collect::<HashMap<_, _>>();
    for (c, d) in CD {
        for ch in make_ascii_lowercase_chars() {
            if char_map[&ch] == c {
                char_map.insert(ch, d);
            }
        }
    }
    let ans = S.into_iter().map(|ch| char_map[&ch]).collect::<String>();
    println!("{}", ans);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
