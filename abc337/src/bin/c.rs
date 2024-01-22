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
        A: [isize; N]
    };

    let mut pos_map = HashMap::new();
    for (i, a) in A.into_iter().enumerate() {
        pos_map.insert(a, i + 1);
    }
    let mut ans = vec![];
    let mut pos = pos_map[&-1];
    loop {
        ans.push(pos);
        if let Some(&next_pos) = pos_map.get(&(pos as isize)) {
            pos = next_pos;
        } else {
            break;
        }
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
