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
        if x >= self.n {
            panic!();
        }
        if self.root[x] == x {
            x
        } else {
            let root = self.find(self.root[x]);
            self.root[x] = root;
            root
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        if x >= self.n || y >= self.n {
            panic!();
        }
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
        if x >= self.n {
            panic!();
        }
        let x_root = self.find(x);
        self.size[x_root]
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        if x >= self.n || y >= self.n {
            panic!();
        }
        self.find(x) == self.find(y)
    }
}

fn solve() {
    input! {
        n: usize, m: usize, k: usize,
        abv: [(Usize1, Usize1); m],
        cdv: [(Usize1, Usize1); k]
    };

    let mut friend_relations = UnionFind::new(n);
    let mut friends = vec![0; n];
    for (a, b) in abv {
        friend_relations.unite(a, b);
        friends[a] += 1;
        friends[b] += 1;
    }
    let mut blocks = vec![Vec::new(); n];
    for (c, d) in cdv {
        blocks[c].push(d);
        blocks[d].push(c);
    }
    let res = (0..n)
        .map(|u| {
            let all = friend_relations.size(u) - 1;
            let mut res = all - friends[u];
            for &b in &blocks[u] {
                if friend_relations.is_same(u, b) {
                    res -= 1;
                }
            }
            res
        })
        .join(" ");
    println!("{}", res);
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
