use proconio::input;
use proconio::marker::Usize1;

pub struct UnionFind {
  n: usize,
  root: Vec<usize>,
  rank: Vec<usize>,
  size: Vec<usize>,
}

impl UnionFind {
  pub fn new(n: usize) -> Self {
    let root = (0..n).into_iter().collect();
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

fn main() {
  input! {
    n: usize, m: usize,
    edges: [(Usize1, Usize1); m]
  };

  let mut res = 0;
  for skip in 0..m {
    let mut uf = UnionFind::new(n);
    for (i, &(u, v)) in edges.iter().enumerate() {
      if i == skip {
        continue;
      }
      uf.unite(u, v);
    }
    for u in 0..n {
      if uf.size(u) != n {
        res += 1;
        break;
      }
    }
  }
  println!("{}", res);
}
