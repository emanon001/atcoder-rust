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
        n: usize, k: usize,
    };
    let mut que = BinaryHeap::new();
    let mut c = 0;
    let v = (3, 1, 1, 1);
    que.push(std::cmp::Reverse(v));
    let mut used = HashSet::new();
    used.insert(v);
    while let Some(std::cmp::Reverse((x, i, j, k2))) = que.pop() {
        c += 1;
        if c == k {
            println!("{} {} {}", i, j, k2);
            return;
        }
        // i + 1, j, k
        let v = (x + 1, i + 1, j, k2);
        if i + 1 <= n && !used.contains(&v) {
            used.insert(v);
            que.push(std::cmp::Reverse(v));
        }
        // i, j + 1, k
        let v = (x + 1, i, j + 1, k2);
        if j + 1 <= n && !used.contains(&v) {
            used.insert(v);
            que.push(std::cmp::Reverse(v));
        }
        // i, j, k + 1
        let v = (x + 1, i, j, k2 + 1);
        if k2 + 1 <= n && !used.contains(&v) {
            used.insert(v);
            que.push(std::cmp::Reverse(v));
        }
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
