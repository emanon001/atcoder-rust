use crate::segment_tree::{Monoid, SegmentTree};

#[derive(Debug, Copy, Clone)]
pub struct LcaDepth {
  depth: usize,
  idx: usize,
}

impl Monoid for LcaDepth {
  fn empty() -> Self {
    Self {
      depth: std::usize::MAX,
      idx: 0,
    }
  }

  fn append(&self, other: &Self) -> Self {
    if self.depth <= other.depth {
      *self
    } else {
      *other
    }
  }
}

pub struct Lca {
  vs: Vec<usize>,
  vdepth: Vec<usize>,
  vidx: Vec<usize>,
  vn: usize,
  st: SegmentTree<LcaDepth>,
}

impl Lca {
  pub fn new(edges: &[(usize, usize)], vn: usize, root: usize) -> Self {
    let mut graph = vec![Vec::new(); vn];
    for &(u, v) in edges {
      graph[u].push(v);
      graph[v].push(u);
    }
    let mut vidx = vec![0; vn];
    let mut vs = vec![0; vn * 2 - 1];
    let mut vdepth = vec![0; vn * 2 - 1];
    let mut k = 0;
    Self::traverse(root, vn, 0, &mut k, &graph, &mut vidx, &mut vs, &mut vdepth);
    let lca_depth = vdepth
      .iter()
      .copied()
      .enumerate()
      .map(|(i, d)| LcaDepth { depth: d, idx: i })
      .collect::<Vec<_>>();
    let st = SegmentTree::from_slice(&lca_depth);
    Self {
      vs,
      vdepth,
      vidx,
      vn,
      st,
    }
  }

  // 0-origin
  pub fn depth(&self, u: usize) -> usize {
    if u >= self.vn {
      panic!("u >= self.vn");
    }
    let i = self.vidx[u];
    self.vdepth[i]
  }

  pub fn query(&self, u: usize, v: usize) -> usize {
    let ui = self.vidx[u];
    let vi = self.vidx[v];
    let LcaDepth { idx, .. } = self
      .st
      .query(std::cmp::min(ui, vi), std::cmp::max(ui, vi) + 1);
    self.vs[idx]
  }

  fn traverse(
    u: usize,
    p: usize,
    d: usize,
    k: &mut usize,
    graph: &[Vec<usize>],
    vidx: &mut [usize],
    vs: &mut [usize],
    vdepth: &mut [usize],
  ) {
    vidx[u] = *k;
    vs[*k] = u;
    vdepth[*k] = d;
    *k += 1;
    for &v in &graph[u] {
      if v != p {
        Self::traverse(v, u, d + 1, k, graph, vidx, vs, vdepth);
        vs[*k] = u;
        vdepth[*k] = d;
        *k += 1;
      }
    }
  }
}
