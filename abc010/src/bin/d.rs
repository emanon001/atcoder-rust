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
        n: usize, g: usize, e: usize,
        pv: [usize; g],
        edges: [(usize, usize); e]
    };

    // pおよびpに直接繋がっている頂点で連結成分を計算する
    let p_set = pv.iter().collect::<HashSet<_>>();
    let mut pcon_set = HashSet::new();
    let mut p_uf = UnionFind::new(n);
    for &(a, b) in &edges {
        if p_set.contains(&a) && p_set.contains(&b) {
            continue;
        }
        if p_set.contains(&a) || p_set.contains(&b) {
            p_uf.unite(a, b);
            let x = if p_set.contains(&a) { b } else { a };
            pcon_set.insert(x);
        }
    }
    // 連結成分毎にpが何個含まれるか
    let mut p_counts = HashMap::new();
    for &p in &pv {
        *p_counts.entry(p_uf.find(p)).or_insert(0) += 1;
    }
    // pおよびpに直接繋がっている頂点以外で連結成分を計算する
    let mut t_uf = UnionFind::new(n);
    for &(a, b) in &edges {
        if p_set.contains(&a) || p_set.contains(&b) {
            continue;
        }
        if pcon_set.contains(&a) || pcon_set.contains(&b) {
            continue;
        }
        t_uf.unite(a, b);
    }

    // 高橋君の属する連結成分と、pの属する連結成分間をエッジが何本繋がっているか
    let mut edge_counts = HashMap::new();
    for &(a, b) in &edges {
        if pcon_set.contains(&a) && !pcon_set.contains(&b) && !p_set.contains(&b) {
            if t_uf.is_same(0, b) {
                *edge_counts.entry(p_uf.find(a)).or_insert(0) += 1;
            }
        } else if pcon_set.contains(&b) && !pcon_set.contains(&a) && !p_set.contains(&a) {
            if t_uf.is_same(0, a) {
                *edge_counts.entry(p_uf.find(b)).or_insert(0) += 1;
            }
        }
    }

    let mut res = 0;
    for (k, ec) in edge_counts {
        if let Some(&pc) = p_counts.get(&k) {
            res += ec.min(pc);
        }
    }
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
