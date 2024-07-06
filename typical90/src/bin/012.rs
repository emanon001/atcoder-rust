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

fn uf_id(r: usize, c: usize, w: usize) -> usize {
    r * w + c
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize,
        Q: usize,
    };

    let mut uf = UnionFind::new(H * W);
    let mut red_set = HashSet::new();
    for _ in 0..Q {
        input_interactive! {
            t: usize,
        };
        match t {
            1 => {
                input_interactive! {
                    r: Usize1, c: Usize1,
                };
                let id = uf_id(r, c, W);
                red_set.insert(id);
                // 上
                if r > 0 && red_set.contains(&uf_id(r - 1, c, W)) {
                    uf.unite(id, uf_id(r - 1, c, W));
                }
                // 下
                if r + 1 < H && red_set.contains(&uf_id(r + 1, c, W)) {
                    uf.unite(id, uf_id(r + 1, c, W));
                }
                // 左
                if c > 0 && red_set.contains(&uf_id(r, c - 1, W)) {
                    uf.unite(id, uf_id(r, c - 1, W));
                }
                // 右
                if c + 1 < W && red_set.contains(&uf_id(r, c + 1, W)) {
                    uf.unite(id, uf_id(r, c + 1, W));
                }
            }
            2 => {
                input_interactive! {
                    ra: Usize1, ca: Usize1,
                    rb: Usize1, cb: Usize1,
                };
                let id1 = uf_id(ra, ca, W);
                let id2 = uf_id(rb, cb, W);
                if red_set.contains(&id1) && red_set.contains(&id2) && uf.is_same(id1, id2) {
                    println!("Yes");
                } else {
                    println!("No");
                }
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
