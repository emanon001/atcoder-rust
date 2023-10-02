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
        n: usize, m: usize,
        edges: [(Usize1, Usize1);  m]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut bipartite_graphs = Vec::new();
    let mut visited = HashSet::new();
    let mut bipartite_v_count = 0;
    for u in 0..n {
        if visited.contains(&u) {
            continue;
        }
        let (is_ok, v_count1, v_count2, edge_count) = check_bipartite(u, &graph, &mut visited);
        if !is_ok {
            println!("0");
            return;
        }
        bipartite_graphs.push((v_count1, v_count2, edge_count));
        bipartite_v_count += v_count1 + v_count2;
    }

    // eprintln!("{:?}", bipartite_graphs);

    let mut ans = 0_usize;
    let mut other_edge_count = 0;
    for (v_count1, v_count2, edge_count) in bipartite_graphs {
        ans += v_count1 * v_count2 - edge_count;
        let v_count = v_count1 + v_count2;
        other_edge_count += v_count1 * (bipartite_v_count - v_count);
        other_edge_count += v_count2 * (bipartite_v_count - v_count);
    }
    ans += other_edge_count / 2;
    println!("{}", ans);
}

fn check_bipartite(
    u: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
) -> (bool, usize, usize, usize) {
    let mut colors = HashMap::new();
    let color = true;
    colors.insert(u, color);
    visited.insert(u);

    if !check_bipartite_rec(u, color, graph, visited, &mut colors) {
        return (false, 0, 0, 0);
    }
    let mut v_count1 = 0;
    let mut v_count2 = 0;
    let mut edge_count = 0;
    for (u, c) in colors {
        edge_count += &graph[u].len();
        if c {
            v_count1 += 1;
        } else {
            v_count2 += 1;
        }
    }
    edge_count /= 2;
    (true, v_count1, v_count2, edge_count)
}

fn check_bipartite_rec(
    u: usize,
    color: bool,
    graph: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    colors: &mut HashMap<usize, bool>,
) -> bool {
    let new_color = !color;
    let mut res = true;
    for &v in &graph[u] {
        if visited.contains(&v) {
            if colors[&v] != new_color {
                res = false;
            }
            continue;
        }
        visited.insert(v);
        colors.insert(v, new_color);
        res = check_bipartite_rec(v, new_color, graph, visited, colors) && res;
    }
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
