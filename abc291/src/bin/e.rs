#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input! {
        N: usize, M: usize,
        XY: [(Usize1, Usize1); M],
    };

    let mut graph = vec![vec![]; N];
    let mut indegree = vec![0; N];
    for (u, v) in XY {
        graph[u].push(v);
        indegree[v] += 1;
    }
    let mut start = vec![];
    for u in 0..N {
        if indegree[u] == 0 {
            start.push(u);
        }
    }
    if start.len() != 1 {
        println!("No");
        return;
    }
    let mut u = start[0];
    let mut ans = vec![None; N];
    let mut order = 1;
    loop {
        ans[u] = Some(order);
        order += 1;
        let mut new_u = vec![];
        for &v in &graph[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                new_u.push(v);
            }
        }
        if new_u.len() != 1 {
            break;
        }
        u = new_u[0];
    }

    let ans = ans.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    if ans.len() != N {
        println!("No");
        return;
    }

    println!("Yes");
    println!("{}", ans.iter().join(" "));
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
