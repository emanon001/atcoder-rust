#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct SegmentTree<T>
where
    T: Monoid + Clone,
{
    size: usize,
    data: Vec<T>,
}
impl<T> SegmentTree<T>
where
    T: Monoid + Clone,
{
    pub fn new(size: usize) -> Self {
        let size = Self::normalize_data_size(size);
        let data = vec![T::mempty(); size * 2 - 1];
        Self { size, data }
    }
    /// 0-origin
    pub fn update(&mut self, i: usize, v: T) {
        let mut i = i + self.size - 1;
        self.data[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1].mappend(&self.data[i * 2 + 2]);
        }
    }
    /// [a, b)
    /// 0-origin
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
            return T::mempty();
        }
        if a <= l && r <= b {
            return self.data[i].clone();
        }
        let vl = self.execute_query(a, b, i * 2 + 1, l, (l + r) / 2);
        let vr = self.execute_query(a, b, i * 2 + 2, (l + r) / 2, r);
        vl.mappend(&vr)
    }
}
impl<T> From<&[T]> for SegmentTree<T>
where
    T: Monoid + Clone,
{
    fn from(values: &[T]) -> Self {
        let mut st = SegmentTree::new(values.len());
        for (i, v) in values.into_iter().enumerate() {
            st.data[i + st.size - 1] = v.clone();
        }
        if st.size < 2 {
            return st;
        }
        for i in (0..=(st.size - 2)).rev() {
            st.data[i] = st.data[i * 2 + 1].mappend(&st.data[i * 2 + 2]);
        }
        st
    }
}
impl<T> From<Vec<T>> for SegmentTree<T>
where
    T: Monoid + Clone,
{
    fn from(values: Vec<T>) -> Self {
        let values: &[T] = &values;
        Self::from(values)
    }
}
pub trait Monoid {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}
#[derive(Debug, Copy, Clone)]
pub struct LcaDepth {
    depth: usize,
    idx: usize,
}
impl Monoid for LcaDepth {
    fn mempty() -> Self {
        Self {
            depth: std::usize::MAX,
            idx: 0,
        }
    }
    fn mappend(&self, other: &Self) -> Self {
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
    pub fn new(edges: Vec<(usize, usize)>, vn: usize, root: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for (u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut vidx = vec![0; vn];
        let mut vs = vec![0; vn * 2 - 1];
        let mut vdepth = vec![0; vn * 2 - 1];
        Self::traverse(
            root,
            root,
            0,
            &mut 0,
            &graph,
            &mut vidx,
            &mut vs,
            &mut vdepth,
        );
        let lca_depth = vdepth
            .iter()
            .copied()
            .enumerate()
            .map(|(i, d)| LcaDepth { depth: d, idx: i })
            .collect::<Vec<_>>();
        let st = SegmentTree::from(lca_depth);
        Self {
            vs,
            vdepth,
            vidx,
            vn,
            st,
        }
    }
    /// 0-origin
    pub fn depth(&self, u: usize) -> usize {
        if u >= self.vn {
            panic!("u >= self.vn");
        }
        let i = self.vidx[u];
        self.vdepth[i]
    }
    pub fn distance(&self, u: usize, v: usize) -> usize {
        let p = self.query(u, v);
        self.depth(u) + self.depth(v) - self.depth(p) * 2
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
        parent: usize,
        depth: usize,
        vid: &mut usize,
        graph: &[Vec<usize>],
        vids: &mut [usize],
        vs: &mut [usize],
        vdepth: &mut [usize],
    ) {
        vids[u] = *vid;
        vs[*vid] = u;
        vdepth[*vid] = depth;
        *vid += 1;
        for &v in &graph[u] {
            if v != parent {
                Self::traverse(v, u, depth + 1, vid, graph, vids, vs, vdepth);
                vs[*vid] = u;
                vdepth[*vid] = depth;
                *vid += 1;
            }
        }
    }
}

fn solve() {
    input! {
        n: usize, q: usize,
        edges: [(Usize1, Usize1); n - 1],
        queries: [(Usize1, Usize1); q]
    };

    let lca = Lca::new(edges, n, 0);
    for (c, d) in queries {
        let distance = lca.distance(c, d);
        let res = if distance % 2 == 0 {
            "Town"
        } else {
            "Road"
        };
        println!("{}", res);
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
