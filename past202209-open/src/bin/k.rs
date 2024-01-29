#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct UnionFind {
    n: usize,
    root: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let root = (0..n).collect();
        let rank = vec![0; n];
        let size = vec![1; n];
        Self {
            n,
            root,
            rank,
            size,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.root[x] == x {
            x
        } else {
            let root = self.find(self.root[x]);
            self.root[x] = root;
            root
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        assert!(x < self.n && y < self.n);
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            self.root[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else {
            self.root[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }
    pub fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let x_root = self.find(x);
        self.size[x_root]
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n && y < self.n);
        self.find(x) == self.find(y)
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut groups = std::collections::HashMap::new();
        for x in 0..self.n {
            let k = self.find(x);
            groups.entry(k).or_insert(Vec::new()).push(x);
        }
        groups.values().cloned().collect::<Vec<_>>()
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, M: usize,
        edges: [(Usize1, Usize1); M],
        Q: usize,
        queries: [(usize, Usize1, Usize1); Q]
    };

    let mut graph = vec![HashSet::new(); N];
    for (u, v) in edges {
        graph[u].insert(v);
        graph[v].insert(u);
    }
    for &(t, u, v) in &queries {
        match t {
            1 => {
                graph[u].insert(v);
                graph[v].insert(u);
            }
            2 => {
                graph[u].remove(&v);
                graph[v].remove(&u);
            }
            _ => {}
        }
    }

    let mut uf = new_uf(&graph, N);
    let mut ans = VecDeque::new();
    for (t, u, v) in queries.into_iter().rev() {
        match t {
            1 => {
                graph[u].remove(&v);
                graph[v].remove(&u);
                uf = new_uf(&graph, N);
            }
            2 => {
                graph[u].insert(v);
                graph[v].insert(u);
                uf.unite(u, v);
            }
            3 => {
                ans.push_front(if uf.is_same(u, v) { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans.iter().join("\n"));
}

fn new_uf(graph: &[HashSet<usize>], n: usize) -> UnionFind {
    let mut uf = UnionFind::new(n);
    for (u, vx) in graph.iter().enumerate() {
        for &v in vx {
            uf.unite(u, v);
        }
    }
    uf
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
