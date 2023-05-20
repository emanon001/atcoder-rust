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
        n: usize, q: usize,
    };

    let mut vs = vec![HashSet::new(); n];
    let mut undirected = n;
    for _ in 0..q {
        input! {
            kind: usize,
        }
        match kind {
            1 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                }
                if vs[u].is_empty() {
                    undirected -= 1;
                }
                vs[u].insert(v);
                if vs[v].is_empty() {
                    undirected -= 1;
                }
                vs[v].insert(u);
                println!("{}", undirected);
            }
            2 => {
                input! {
                    v: Usize1,
                }
                for u in vs[v].clone() {
                    vs[u].remove(&v);
                    if vs[u].is_empty() {
                        undirected += 1;
                    }
                }
                if !vs[v].is_empty() {
                    undirected += 1;
                }
                vs[v] = HashSet::new();
                println!("{}", undirected);
            }
            _ => unreachable!(),
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
