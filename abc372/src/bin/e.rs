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
        N: usize, Q: usize,
        queries: [(usize, usize, usize); Q],
    };

    let mut uf = UnionFind::new(N);
    let mut vertex_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for u in 0..N {
        vertex_map.insert(uf.find(u), vec![u].into_iter().collect());
    }
    for (kind, a, b) in queries {
        match kind {
            1 => {
                let u = a - 1;
                let v = b - 1;
                let find1 = uf.find(u);
                let find2 = uf.find(v);
                uf.unite(u, v);
                let mut new_vertexes = vec![];
                for &u in vertex_map.get(&find1).unwrap_or(&vec![]).iter() {
                    new_vertexes.push(u);
                }
                for &u in vertex_map.get(&find2).unwrap_or(&vec![]).iter() {
                    new_vertexes.push(u);
                }
                let new_vertexes = new_vertexes
                    .into_iter()
                    .sorted_by_key(|&x| std::cmp::Reverse(x))
                    .unique()
                    .take(10)
                    .collect::<Vec<_>>();
                let find3 = uf.find(u);
                vertex_map.insert(find3, new_vertexes);
            }
            2 => {
                let v = a - 1;
                let k = b;
                let find = uf.find(v);
                let ans = if let Some(vertexes) = vertex_map.get(&find) {
                    if vertexes.len() < k {
                        -1
                    } else {
                        vertexes[k - 1] as isize + 1
                    }
                } else {
                    -1
                };
                println!("{}", ans);
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
