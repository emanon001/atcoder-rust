#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        n: usize, _m: usize,
        p: [(i64, [usize]); n]
    };

    let p = p
        .into_iter()
        .map(|(p_i, f)| (p_i, f.into_iter().collect::<HashSet<_>>()))
        .collect::<Vec<_>>();

    for i in 0..n {
        for j in 0..n {
            let (p1, f1) = &p[i];
            let (p2, f2) = &p[j];
            if p1 >= p2 && f1.difference(f2).count() == 0 && (p1 > p2 || f2.len() > f1.len()) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
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
