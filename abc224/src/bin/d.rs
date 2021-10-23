#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

const INF: usize = 1_usize << 30;

fn solve2(
    u: usize,
    graph: Vec<Vec<usize>>,
    state: Vec<usize>,
    mut dp: HashMap<Vec<usize>, usize>,
) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(std::cmp::Reverse((0, u, state)));
    let mut res = INF;
    while let Some(std::cmp::Reverse((cost, u, state))) = heap.pop() {
        let ok = state.iter().take(8).enumerate().all(|(x, i)| x == *i);
        if ok {
            chmin!(res, cost);
            continue;
        }
        for &v in &graph[u] {
            let mut new_state = state.clone();
            new_state[u] = new_state[v];
            new_state[v] = INF;
            if cost + 1 < *dp.get(&new_state).unwrap_or(&INF) {
                dp.insert(new_state.clone(), cost + 1);
                heap.push(std::cmp::Reverse((cost + 1, v, new_state.clone())));
            }
        }
    }
    res
}

fn solve() {
    input! {
        m: usize,
        edges: [(Usize1, Usize1); m],
        pv: [Usize1; 8]
    };

    let mut u = 0;
    for i in 0..9 {
        if !pv.contains(&i) {
            u = i;
            break;
        }
    }
    let mut graph = vec![Vec::new(); 9];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut state = vec![INF; 9];
    for i in 0..8 {
        state[pv[i]] = i;
    }
    state[u] = INF;
    let mut dp = HashMap::new();
    dp.insert(state.clone(), INF);
    let res = solve2(u, graph, state, dp);
    if res == INF {
        println!("-1");
    } else {
        println!("{}", res);
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
