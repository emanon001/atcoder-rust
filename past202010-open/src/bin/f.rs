#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, K: usize,
        S: [String; N],
    };

    let counts = S.into_iter().counts();
    let mut count_to_words = BTreeMap::new();
    for (s, c) in counts {
        count_to_words.entry(c).or_insert(vec![]).push(s);
    }
    let mut count = 0;
    for (_, words) in count_to_words.into_iter().rev() {
        let c = words.len();
        count += c;
        if c > 1 && count >= K {
            println!("AMBIGUOUS");
            return;
        }
        if count >= K {
            println!("{}", words[0]);
            return;
        }
    }
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
