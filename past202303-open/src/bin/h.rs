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
        N: usize,
        mut A: [i64; N],
    };

    A.sort();
    let mut list = compress_list(A);
    let mut ans = 0;
    while !list.is_empty() {
        let mut prev = -1;
        let mut count = 0;
        let mut next_list = Vec::new();
        for (a, c) in list
            .into_iter()
            .chain(std::iter::once((i64::max_value(), 1)))
        {
            if a == prev + 1 {
                count += 1;
            } else {
                ans += count % 3;
                count = 1;
            }
            prev = a;
            if c > 1 {
                next_list.push((a, c - 1));
            }
        }
        list = next_list;
    }
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
