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
        S: Chars,
    };

    let ch_to_count = S.into_iter().counts();
    let mut count_to_ch_count = HashMap::new();
    for (_, count) in ch_to_count {
        *count_to_ch_count.entry(count).or_insert(0) += 1;
    }
    for (_, ch_count) in count_to_ch_count {
        if ch_count != 0 && ch_count != 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
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
