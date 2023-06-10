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
    groups: Vec<BTreeSet<usize>>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let root = (0..n).collect();
        let rank = vec![0; n];
        let size = vec![1; n];
        let mut groups = vec![BTreeSet::new(); n];
        for u in 0..n {
            groups[u].insert(u);
        }
        Self {
            n,
            root,
            rank,
            size,
            groups,
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
            for v in self.groups[x_root].clone() {
                self.groups[y_root].insert(v);
            }
            self.groups[x_root] = BTreeSet::new();
        } else {
            self.root[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
            for v in self.groups[y_root].clone() {
                self.groups[x_root].insert(v);
            }
            self.groups[y_root] = BTreeSet::new();
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
    pub fn groups(&mut self, x: usize) -> std::collections::BTreeSet<usize> {
        let root = self.find(x);
        self.groups[root].clone()
    }
}

fn solve() {
    input! {
        n: usize, q: usize,
    };

    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                };
                uf.unite(u, v);
            }
            2 => {
                input! {
                    u: Usize1
                }
                let res = uf.groups(u).iter().map(|u| u + 1).join(" ");
                println!("{}", res);
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
