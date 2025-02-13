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
        AB: [(Usize1, Usize1); M],
    };

    let mut u_to_unnecessary_edges = HashMap::new();
    let mut uf = UnionFind::new(N);
    for (i, (a, b)) in AB.into_iter().enumerate() {
        if uf.is_same(a, b) {
            u_to_unnecessary_edges
                .entry(a)
                .or_insert(Vec::new())
                .push((b, i));
        } else {
            uf.unite(a, b);
        }
    }

    // グループ毎に不要な辺をまとめる
    let groups = uf.groups();
    let mut group_ids = Vec::new();
    let mut gid_to_unnecessary_edges = HashMap::new();
    for uv in &groups {
        let id = uf.find(uv[0]);
        group_ids.push((id, uv[0]));
        for u in uv {
            match u_to_unnecessary_edges.get(u) {
                Some(edges) => {
                    for &(v, i) in edges {
                        gid_to_unnecessary_edges
                            .entry(id)
                            .or_insert(Vec::new())
                            .push((u, v, i));
                    }
                }
                None => {}
            }
        }
    }

    let mut rest_group_ids: HashSet<(usize, usize)> = group_ids.clone().into_iter().collect();
    let mut ans = Vec::new();
    for &(gid, gu) in &group_ids {
        if uf.size(gu) == N {
            break;
        }
        match gid_to_unnecessary_edges.get(&gid) {
            Some(edges) => {
                for (_, v, i) in edges {
                    if uf.size(gu) == N {
                        break;
                    }
                    for &(gid2, gu2) in &rest_group_ids {
                        if uf.is_same(gu, gu2) {
                            continue;
                        }
                        uf.unite(gu, gu2);
                        rest_group_ids.remove(&(gid2, gu2));
                        ans.push((i + 1, v + 1, gu2 + 1));
                        break;
                    }
                }
            }
            None => {}
        }
    }
    println!("{}", ans.len());
    if ans.len() > 0 {
        println!(
            "{}",
            ans.into_iter()
                .map(|(i, v, v2)| format!("{} {} {}", i, v, v2))
                .join("\n")
        )
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
