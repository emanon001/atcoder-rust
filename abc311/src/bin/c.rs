#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        n: usize,
        a: [Usize1; n]
    };

    let mut visited = HashSet::new();
    for u in 0..n {
        if visited.contains(&u) {
            continue;
        }
        let mut path = dfs(u, &a, &mut visited);
        let u = path.pop_back().unwrap();
        let mut ans = VecDeque::new();
        for v in path.into_iter().rev() {
            ans.push_front(v);
            if u == v {
                break;
            }
        }
        println!("{}", ans.len());
        println!("{}", ans.into_iter().join(" "));
        return;
    }
}

fn dfs(u: usize, edges: &Vec<usize>, visited: &mut HashSet<usize>) -> VecDeque<usize> {
    if visited.contains(&u) {
        return vec![u + 1].into();
    }
    visited.insert(u);
    let mut path = dfs(edges[u], edges, visited);
    path.push_front(u + 1);
    path
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
