#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        sv: [Chars; n]
    };

    if sv.len() != sv.iter().unique().count() {
        println!("No");
        return;
    }

    let res = if sv.into_iter().all(|s| {
        let c1 = s[0];
        let c2 = s[1];
        ['H', 'D', 'C', 'S'].contains(&c1)
            && [
                'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
            ]
            .contains(&c2)
    }) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", res);
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
