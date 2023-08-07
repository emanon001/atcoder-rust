#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
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

fn solve() {
    input! {
        n: usize,
        av: [i64; n]
    };

    let mut list = compress_list(av.iter().copied().sorted().collect::<Vec<_>>());
    list.reverse();
    for k in 0..n {
        let res = if k >= list.len() { 0 } else { list[k].1 };
        println!("{}", res);
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
