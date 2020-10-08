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
        av: [usize; 2.pow(n as u32)]
    };

    let mut res = vec![0; av.len()];
    let av = av.into_iter().enumerate().collect::<Vec<_>>();
    let mut winners = av;
    let mut c = 1;
    while winners.len() >= 2 {
        let mut new_winners = Vec::new();
        for chunk in winners.chunks(2) {
            let max = chunk.into_iter().max_by_key(|x| x.1).unwrap();
            for (i, a) in chunk {
                if i == &max.0 {
                    new_winners.push((*i, *a));
                }
                res[*i] = c;
            }
        }
        winners = new_winners;
        c += 1;
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
