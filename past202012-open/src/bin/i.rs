#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, M: usize, K: usize,
        H: [usize; N],
        C: [Usize1; K],
        AB: [(Usize1, Usize1); M],
    };

    let mut graph = vec![vec![]; N];
    for (a, b) in AB {
        if H[a] > H[b] {
            graph[a].push(b);
        } else {
            graph[b].push(a);
        }
    }

    let max = 1 << 30;
    let c_set = C.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![None; N];
    for u in 0..N {
        let distance = traverse(u, &graph, &c_set, max, &mut dp);
        let ans = if distance == max {
            -1
        } else {
            distance as isize
        };
        println!("{}", ans);
    }
}

fn traverse(
    u: usize,
    graph: &Vec<Vec<usize>>,
    c_set: &HashSet<usize>,
    max: usize,
    dp: &mut Vec<Option<usize>>,
) -> usize {
    if let Some(v) = dp[u] {
        return v;
    }
    if c_set.contains(&u) {
        return 0;
    }

    let mut res = max;
    for &v in &graph[u] {
        let distance = traverse(v, graph, c_set, max, dp);
        chmin!(res, distance + 1);
    }
    dp[u] = Some(res);
    res
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
