#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn compress_list<T: Copy + std::cmp::PartialEq>(list: Vec<T>) -> Vec<(T, usize)> {
    let mut res = Vec::new();
    if list.is_empty() {
        return res;
    }
    let mut cur_v = list[0];
    let mut count = 1;
    for v in list.into_iter().skip(1) {
        if v == cur_v {
            count += 1;
        } else {
            res.push((cur_v, count));
            count = 1;
        }
        cur_v = v;
    }
    res.push((cur_v, count));
    res
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        _N: usize, K: usize,
        S: Chars,
    };

    let mut compressed = compress_list(S);
    let mut one_count = 0;
    for i in 0..compressed.len() {
        if compressed[i].0 == '1' {
            one_count += 1;
            if one_count == K {
                compressed.swap(i - 1, i);
                break;
            }
        }
    }
    let ans = compressed
        .into_iter()
        .map(|(ch, count)| ch.to_string().repeat(count))
        .join("");
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
