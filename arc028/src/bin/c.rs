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
        pv: [usize; n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    let mut indegree = vec![0; n];
    for (u, v) in pv.into_iter().enumerate() {
        let u = u + 1;
        graph[u].push(v);
        graph[v].push(u);
        indegree[u] += 1;
        indegree[v] += 1;
    }

    let mut res = vec![0; n];
    let mut que = VecDeque::new();
    for i in 0..n {
        if indegree[i] == 1 {
            que.push_back((i, i));
            res[i] = n - 1;
        }
    }
    let mut childs = vec![0; n];
    while let Some((u, parent)) = que.pop_front() {
        for &v in &graph[u] {
            if v == parent {
                continue;
            }
            if indegree[v] == 1 {
                continue;
            }
            indegree[v] -= 1;
            res[v] = res[v].max(childs[u] + 1);
            if indegree[v] > 0 {
                childs[v] += childs[u] + 1;
            }
            if indegree[v] == 1 {
                res[v] = res[v].max(n - childs[v] - 1);
                que.push_back((v, u));
            }
        }
    }
    for i in 0..n {
        println!("{}", res[i]);
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
