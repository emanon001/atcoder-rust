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

enum Input {
  Edge(usize, usize, usize),
  Query(usize, usize, usize),
}

fn main() {
  input! {
    n: usize, m: usize,
    edges: [(Usize1, Usize1, usize); m],
    q: usize,
    queries: [(Usize1, usize); q]
  };

  let mut input = edges
    .into_iter()
    .map(|(a, b, y)| Input::Edge(a, b, y))
    .chain(
      queries
        .into_iter()
        .enumerate()
        .map(|(i, (v, w))| Input::Query(i, v, w)),
    )
    .collect::<Vec<_>>();
  input.sort_by_key(|a| match a {
    Input::Edge(_, _, y) => (-(*y as isize), 1),
    Input::Query(_, _, w) => (-(*w as isize), 0),
  });

  let mut res = vec![0; q];
  let mut uf = UnionFind::new(n);
  for i in input {
    match i {
      Input::Edge(a, b, _) => uf.unite(a, b),
      Input::Query(i, v, _) => res[i] = uf.size(v),
    }
  }
  for c in res {
    println!("{}", c);
  }
}
