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
        n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n],
    };

    let mut a = a;
    for _ in 0..4 {
        if check(&a, &b, n) {
            println!("Yes");
            return;
        }
        a = rotate90(a, n);
    }
    println!("No");
}

fn rotate90(a: Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut ans = Vec::new();
    for i in 0..n {
        let mut row = VecDeque::new();
        for j in 0..n {
            row.push_front(a[j][i]);
        }
        ans.push(row.into_iter().collect::<Vec<_>>())
    }
    ans
}

fn check(a: &[Vec<usize>], b: &[Vec<usize>], n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 && b[i][j] == 0 {
                return false;
            }
        }
    }
    true
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
