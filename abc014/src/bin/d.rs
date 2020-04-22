#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

pub trait Monoid {
  fn empty() -> Self;
  fn append(&self, other: &Self) -> Self;
}

pub struct SegmentTree<T>
where
  T: Monoid + Clone + Copy,
{
  size: usize,
  data: Vec<T>,
}

impl<T> SegmentTree<T>
where
  T: Monoid + Clone + Copy,
{
  pub fn new(size: usize) -> Self {
    let size = Self::normalize_data_size(size);
    let data = vec![T::empty(); size * 2 - 1];
    Self { size, data }
  }

  pub fn from_slice(values: &[T]) -> Self {
    let mut st = SegmentTree::new(values.len());
    for (i, v) in values.into_iter().enumerate() {
      st.data[i + st.size - 1] = *v;
    }
    if st.size < 2 {
      return st;
    }
    for i in (0..=(st.size - 2)).rev() {
      st.data[i] = st.data[i * 2 + 1].append(&st.data[i * 2 + 2]);
    }
    st
  }

  // 0-origin
  pub fn update(&mut self, i: usize, v: T) {
    let mut i = i + self.size - 1;
    self.data[i] = v;
    while i > 0 {
      i = (i - 1) / 2;
      self.data[i] = self.data[i * 2 + 1].append(&self.data[i * 2 + 2]);
    }
  }

  // [a, b)
  // 0-origin
  pub fn query(&self, a: usize, b: usize) -> T {
    self.execute_query(a, b, 0, 0, self.size)
  }

  fn normalize_data_size(size: usize) -> usize {
    let mut n = 1;
    while n < size {
      n *= 2;
    }
    n
  }

  fn execute_query(&self, a: usize, b: usize, i: usize, l: usize, r: usize) -> T {
    if r <= a || b <= l {
      return T::empty();
    }
    if a <= l && r <= b {
      return self.data[i];
    }
    let vl = self.execute_query(a, b, i * 2 + 1, l, (l + r) / 2);
    let vr = self.execute_query(a, b, i * 2 + 2, (l + r) / 2, r);
    vl.append(&vr)
  }
}

fn dfs(
  u: usize,
  p: usize,
  d: usize,
  k: &mut usize,
  graph: &[Vec<usize>],
  id: &mut [usize],
  vs: &mut [usize],
  depth: &mut [usize],
) {
  id[u] = *k;
  vs[*k] = u;
  depth[*k] = d;
  *k += 1;
  for &v in &graph[u] {
    if v != p {
      dfs(v, u, d + 1, k, graph, id, vs, depth);
      vs[*k] = u;
      depth[*k] = d;
      *k += 1;
    }
  }
}

impl Monoid for usize {
  fn empty() -> Self {
    1 << 60
  }

  fn append(&self, other: &Self) -> Self {
    std::cmp::min(*self, *other)
  }
}

fn main() {
  input! {
    n: usize,
    edges: [(Usize1, Usize1); n - 1],
    q: usize,
    queries: [(Usize1, Usize1); q]
  };

  let mut graph = vec![Vec::new(); n];
  for (u, v) in edges {
    graph[u].push(v);
    graph[v].push(u);
  }
  let mut id = vec![0; n];
  let mut vs = vec![0; n * 2 - 1];
  let mut depth = vec![0; n * 2 - 1];
  let mut k = 0;
  dfs(0, n, 0, &mut k, &graph, &mut id, &mut vs, &mut depth);
  let st: SegmentTree<usize> = SegmentTree::from_slice(&depth);
  for (u, v) in queries {
    let d = st.query(std::cmp::min(id[u], id[v]), std::cmp::max(id[u], id[v]) + 1);
    let res = depth[id[u]] - d + depth[id[v]] - d + 1;
    println!("{}", res);
  }
}
