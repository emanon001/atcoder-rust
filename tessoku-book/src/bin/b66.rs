#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
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

enum Query {
    Remove(usize),
    Question(usize, usize),
}

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m],
        q: usize,
    };

    let mut removed_edges = HashSet::new();
    let mut query_list = Vec::new();
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    x: Usize1
                };
                removed_edges.insert(x);
                query_list.push(Query::Remove(x));
            }
            2 => {
                input! {
                    u: Usize1, v: Usize1
                };
                query_list.push(Query::Question(u, v));
            }
            _ => unreachable!(),
        }
    }
    let mut uf = UnionFind::new(n);
    for (i, &(u, v)) in edges.iter().enumerate() {
        if !removed_edges.contains(&i) {
            uf.unite(u, v)
        }
    }
    let mut res = VecDeque::new();
    for q in query_list.into_iter().rev() {
        match q {
            Query::Remove(x) => uf.unite(edges[x].0, edges[x].1),
            Query::Question(u, v) => {
                let ans = if uf.is_same(u, v) { "Yes" } else { "No" };
                res.push_front(ans.to_string());
            }
        }
    }
    println!("{}", res.iter().join("\n"));
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
