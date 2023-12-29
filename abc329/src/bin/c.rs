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

fn solve() {
    input_interactive! {
        _: usize,
        s: Chars
    };

    let mut count_map = HashMap::new();
    for (ch, count) in compress_list(s) {
        let count = count.max(*count_map.get(&ch).unwrap_or(&0));
        count_map.insert(ch, count);
    }
    let ans: usize = count_map.values().sum();
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
