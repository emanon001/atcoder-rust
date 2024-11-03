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
        N: usize,
        A: [usize; N],
    };

    let mut prev_pos_map = HashMap::new();
    let mut ans = vec![];
    for (i, &a) in A.iter().enumerate() {
        let prev_pos = if let Some(pos) = prev_pos_map.get(&a) {
            (pos + 1) as isize
        } else {
            -1
        };
        ans.push(prev_pos);
        prev_pos_map.insert(a, i);
    }
    println!("{}", ans.iter().join(" "));
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
