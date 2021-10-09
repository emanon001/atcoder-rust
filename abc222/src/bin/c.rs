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
        n: usize, m: usize,
        table: [Chars; 2 * n]
    };

    let mut v = (0..n * 2).map(|i| (0, i)).collect::<Vec<_>>();
    for j in 0..m {
        let mut new_v = Vec::new();
        for p in 0..n {
            let (aw, ai) = v[p * 2];
            let (bw, bi) = v[p * 2 + 1];
            match (table[ai][j], table[bi][j]) {
                ('G', 'C') => {
                    new_v.push((aw + 1, ai));
                    new_v.push((bw, bi));
                }
                ('G', 'P') => {
                    new_v.push((aw, ai));
                    new_v.push((bw + 1, bi));
                }
                ('C', 'P') => {
                    new_v.push((aw + 1, ai));
                    new_v.push((bw, bi));
                }
                ('C', 'G') => {
                    new_v.push((aw, ai));
                    new_v.push((bw + 1, bi));
                }
                ('P', 'G') => {
                    new_v.push((aw + 1, ai));
                    new_v.push((bw, bi));
                }
                ('P', 'C') => {
                    new_v.push((aw, ai));
                    new_v.push((bw + 1, bi));
                }
                _ => {
                    new_v.push((aw, ai));
                    new_v.push((bw, bi));
                }
            }
        }
        v = new_v;
        v.sort_by_key(|&(w, i)| (-w, i));
    }
    let mut res = vec![0; n * 2];
    for j in 0..n * 2 {
        let (_, i) = v[j];
        res[j] = i + 1;
    }
    for x in res {
        println!("{}", x);
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
