#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Clone)]
pub struct WeightedGraph {
    graph: Vec<Vec<(usize, i64)>>,
    vn: usize,
}
pub type WeightedEdge = (usize, usize, i64);
impl WeightedGraph {
    pub fn new(edges: &[WeightedEdge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        Self { graph, vn }
    }
    pub fn shortest_path_1(&self, start: usize) -> Vec<Option<usize>> {
        let mut prev_list = vec![None; self.vn];
        let mut que = std::collections::VecDeque::new();
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &(v, w) in &self.graph[u] {
                if w != 1 {
                    panic!("weight is not 1");
                }
                if v == start || prev_list[v].is_some() {
                    continue;
                }
                prev_list[v] = Some(u);
                que.push_back(v);
            }
        }
        prev_list
    }
}

fn can_change(a: &[char], b: &[char]) -> bool {
    a.into_iter().zip(b).filter(|(x, y)| x != y).count() == 1
}

fn main() {
    input! {
        first: Chars, last: Chars,
        n: usize,
        sv: [Chars; n]
    };

    if first == last {
        println!("0");
        let s = first.into_iter().collect::<String>();
        println!("{}", s);
        println!("{}", s);
        return;
    }

    let s_set = sv
        .into_iter()
        .chain(vec![first.clone(), last.clone()])
        .collect::<HashSet<_>>();
    let s_to_id = s_set
        .iter()
        .enumerate()
        .map(|(idx, s)| (s, idx))
        .collect::<HashMap<_, _>>();
    let id_to_s = s_set.iter().enumerate().collect::<HashMap<_, _>>();
    let mut edges = Vec::new();
    for (a, b) in s_set.iter().tuple_combinations() {
        if can_change(a, b) {
            let u = s_to_id[a];
            let v = s_to_id[b];
            edges.push((u, v, 1));
        }
    }
    let graph = WeightedGraph::new(&edges, s_set.len());
    let prev_list = graph.shortest_path_1(s_to_id[&first]);
    if prev_list[s_to_id[&last]].is_none() {
        println!("-1");
        return;
    }
    let mut res = VecDeque::new();
    res.push_front(&last);
    let mut cur = s_to_id[&last];
    while let Some(prev) = prev_list[cur] {
        res.push_front(id_to_s[&prev]);
        cur = prev;
    }
    println!("{}", res.len() - 2);
    for s in res {
        println!("{}", s.into_iter().join(""));
    }
}
